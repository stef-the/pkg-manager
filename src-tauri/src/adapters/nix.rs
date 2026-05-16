use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct NixAdapter;

impl PackageManagerAdapter for NixAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "nix".to_string(),
            name: "Nix".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("nix")
    }

    fn version(&self) -> String {
        run_command("nix", &["--version"])
            .ok()
            .and_then(|v| {
                let line = v.lines().next().unwrap_or("").trim();
                Some(line.to_string())
            })
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Nix packages...");

        // nix profile list gives installed packages
        let output = run_command_allow_failure("nix", &["profile", "list"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            // Format varies but typically: Index FlakeRef StorePath
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                // Try to extract package name from the store path or flake ref
                let name = parts.last().unwrap_or(&"")
                    .rsplit('/')
                    .next()
                    .unwrap_or("")
                    .split('-')
                    .skip(1) // skip the hash
                    .collect::<Vec<&str>>()
                    .join("-");

                if !name.is_empty() {
                    packages.push(Package {
                        name,
                        version: String::new(),
                        description: String::new(),
                        manager: "nix".to_string(),
                    });
                }
            }
        }

        log::info!("Found {} installed Nix packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Nix for '{}'...", query);

        let output = run_command("nix", &["search", "nixpkgs", query, "--json"])?;

        let json: serde_json::Value = serde_json::from_str(&output).map_err(|e| {
            AppError::ParseError {
                context: "nix search --json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();
        if let Some(obj) = json.as_object() {
            for (key, val) in obj.iter().take(20) {
                let name = key.rsplit('.').next().unwrap_or(key).to_string();
                let version = val.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let description = val.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string();
                packages.push(Package {
                    name,
                    version,
                    description,
                    manager: "nix".to_string(),
                });
            }
        }

        log::info!("Found {} Nix search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Nix handles updates via flake.lock — no per-package outdated check");
        Ok(Vec::new())
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Nix package: {}", name);
        run_command("nix", &["profile", "install", &format!("nixpkgs#{}", name)])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Nix package: {}", name);
        run_command("nix", &["profile", "remove", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Nix package: {}", name);
        run_command("nix", &["profile", "upgrade", name])?;
        Ok(())
    }
}
