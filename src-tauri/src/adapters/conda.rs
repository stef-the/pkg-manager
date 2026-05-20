use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;
use serde_json::Value;

pub struct CondaAdapter;

impl PackageManagerAdapter for CondaAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "conda".to_string(),
            name: "Conda".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("conda")
    }

    fn version(&self) -> String {
        run_command("conda", &["--version"])
            .ok()
            .map(|v| v.trim().to_string())
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Conda packages...");
        let output = run_command("conda", &["list", "--json"])?;

        let items: Vec<Value> = serde_json::from_str(&output).map_err(|e| {
            AppError::ParseError {
                context: "conda list --json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let packages: Vec<Package> = items.iter().filter_map(|item| {
            let name = item.get("name")?.as_str()?.to_string();
            let version = item.get("version")?.as_str()?.to_string();
            let channel = item.get("channel").and_then(|c| c.as_str()).unwrap_or("");
            Some(Package {
                name,
                version,
                description: if channel.is_empty() { String::new() } else { format!("channel: {}", channel) },
                manager: "conda".to_string(),
            })
        }).collect();

        log::info!("Found {} installed Conda packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Conda for '{}'...", query);
        let output = run_command("conda", &["search", query, "--json"])?;

        let json: Value = serde_json::from_str(&output).map_err(|e| {
            AppError::ParseError {
                context: "conda search --json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();
        if let Some(obj) = json.as_object() {
            for (name, versions) in obj.iter().take(20) {
                let latest = versions.as_array()
                    .and_then(|arr| arr.last())
                    .and_then(|v| v.get("version"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                packages.push(Package {
                    name: name.clone(),
                    version: latest.to_string(),
                    description: String::new(),
                    manager: "conda".to_string(),
                });
            }
        }

        log::info!("Found {} Conda search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Conda packages...");
        let output = run_command_allow_failure("conda", &["update", "--all", "--dry-run", "--json"])?;

        let json: Value = serde_json::from_str(&output).unwrap_or_default();
        let mut packages = Vec::new();

        if let Some(actions) = json.get("actions").and_then(|a| a.get("LINK")).and_then(|l| l.as_array()) {
            for item in actions {
                let name = item.get("name").and_then(|n| n.as_str()).unwrap_or("");
                let version = item.get("version").and_then(|v| v.as_str()).unwrap_or("");
                if !name.is_empty() {
                    packages.push(OutdatedPackage {
                        name: name.to_string(),
                        current_version: String::new(),
                        latest_version: version.to_string(),
                        description: String::new(),
                        manager: "conda".to_string(),
                    });
                }
            }
        }

        log::info!("Found {} outdated Conda packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Conda package: {}", name);
        run_command("conda", &["install", "-y", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Conda package: {}", name);
        run_command("conda", &["remove", "-y", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Conda package: {}", name);
        run_command("conda", &["update", "-y", name])?;
        Ok(())
    }
}
