use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct CargoAdapter;

impl PackageManagerAdapter for CargoAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "cargo".to_string(),
            name: "Cargo (Rust)".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("cargo")
    }

    fn version(&self) -> String {
        run_command("cargo", &["--version"])
            .ok()
            .and_then(|v| {
                // "cargo 1.95.0 (f2d3ce0bd 2026-03-21)" -> "cargo 1.95.0"
                let line = v.lines().next().unwrap_or("").trim();
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(format!("{} {}", parts[0], parts[1]))
                } else {
                    Some(line.to_string())
                }
            })
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed cargo packages...");
        let output = run_command("cargo", &["install", "--list"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            // Lines starting without whitespace are package names: "package_name v1.2.3:"
            if !line.starts_with(' ') && !line.starts_with('\t') {
                let trimmed = line.trim().trim_end_matches(':');
                let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
                if let Some(name) = parts.first() {
                    let version = parts
                        .get(1)
                        .map(|v| v.trim_start_matches('v').to_string())
                        .unwrap_or_default();
                    packages.push(Package {
                        name: name.to_string(),
                        version,
                        description: String::new(),
                        manager: "cargo".to_string(),
                    });
                }
            }
        }

        log::info!("Found {} installed cargo packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching crates.io for '{}'...", query);
        let output = run_command("cargo", &["search", query, "--limit=20"])?;

        let packages: Vec<Package> = output
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                // Format: "crate_name = \"1.2.3\"    # Description"
                let eq_pos = line.find('=')?;
                let name = line[..eq_pos].trim().to_string();
                let rest = &line[eq_pos + 1..];
                let version = rest
                    .split('#')
                    .next()?
                    .trim()
                    .trim_matches('"')
                    .to_string();
                let description = rest
                    .split('#')
                    .nth(1)
                    .map(|d| d.trim().to_string())
                    .unwrap_or_default();

                Some(Package {
                    name,
                    version,
                    description,
                    manager: "cargo".to_string(),
                })
            })
            .collect();

        log::info!("Found {} cargo search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        // cargo doesn't have a built-in outdated command for installed binaries
        log::info!("Cargo has no built-in outdated check for installed binaries");
        Ok(Vec::new())
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing cargo package: {}", name);
        run_command("cargo", &["install", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling cargo package: {}", name);
        run_command("cargo", &["uninstall", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating cargo package: {}", name);
        run_command("cargo", &["install", name, "--force"])?;
        Ok(())
    }
}
