use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;
use serde_json::Value;

pub struct NpmAdapter;

impl PackageManagerAdapter for NpmAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "npm".to_string(),
            name: "npm".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("npm")
    }

    fn version(&self) -> String {
        run_command("npm", &["--version"])
            .ok()
            .map(|v| format!("npm {}", v.trim()))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed npm global packages...");

        let output = run_command("npm", &["list", "-g", "--json", "--depth=0"])?;
        let json: Value = serde_json::from_str(&output).map_err(|e| {
            log::error!("Failed to parse npm list JSON: {}", e);
            AppError::ParseError {
                context: "npm list -g --json --depth=0".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();

        if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
            for (name, info) in deps {
                let version = info
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                packages.push(Package {
                    name: name.clone(),
                    version,
                    description: String::new(), // npm list doesn't include descriptions
                    manager: "npm".to_string(),
                });
            }
        }

        log::info!("Found {} installed npm global packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching npm for '{}'...", query);

        let output = run_command("npm", &["search", "--json", query])?;
        let results: Vec<Value> = serde_json::from_str(&output).map_err(|e| {
            log::error!("Failed to parse npm search JSON: {}", e);
            AppError::ParseError {
                context: "npm search --json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let packages: Vec<Package> = results
            .iter()
            .take(30)
            .filter_map(|item| {
                let name = item.get("name")?.as_str()?.to_string();
                let version = item
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let description = item
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("")
                    .to_string();

                Some(Package {
                    name,
                    version,
                    description,
                    manager: "npm".to_string(),
                })
            })
            .collect();

        log::info!("Found {} npm search results for '{}'", packages.len(), query);
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated npm global packages...");

        // npm outdated returns exit code 1 when there are outdated packages, so we allow failure
        let output = run_command_allow_failure("npm", &["outdated", "-g", "--json"])?;

        if output.trim().is_empty() {
            log::info!("No outdated npm packages found");
            return Ok(Vec::new());
        }

        let json: Value = serde_json::from_str(&output).map_err(|e| {
            log::error!("Failed to parse npm outdated JSON: {}", e);
            AppError::ParseError {
                context: "npm outdated -g --json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();

        if let Some(deps) = json.as_object() {
            for (name, info) in deps {
                let current_version = info
                    .get("current")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let latest_version = info
                    .get("latest")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                packages.push(OutdatedPackage {
                    name: name.clone(),
                    current_version,
                    latest_version,
                    description: String::new(),
                    manager: "npm".to_string(),
                });
            }
        }

        log::info!("Found {} outdated npm global packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing npm global package: {}", name);
        run_command("npm", &["install", "-g", name])?;
        log::info!("Successfully installed {}", name);
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling npm global package: {}", name);
        run_command("npm", &["uninstall", "-g", name])?;
        log::info!("Successfully uninstalled {}", name);
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating npm global package: {}", name);
        run_command("npm", &["update", "-g", name])?;
        log::info!("Successfully updated {}", name);
        Ok(())
    }
}
