use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;
use serde_json::Value;

pub struct PipAdapter;

impl PackageManagerAdapter for PipAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "pip".to_string(),
            name: "pip (Python)".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("pip3") || command_exists("pip")
    }

    fn version(&self) -> String {
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        run_command(cmd, &["--version"])
            .ok()
            .and_then(|v| {
                // "pip 26.1.1 from /path... (python 3.14)" -> "pip 26.1.1"
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
        log::info!("Listing installed pip packages...");
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        let output = run_command(cmd, &["list", "--format=json"])?;

        let items: Vec<Value> = serde_json::from_str(&output).map_err(|e| {
            AppError::ParseError {
                context: "pip list --format=json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let packages: Vec<Package> = items
            .iter()
            .filter_map(|item| {
                let name = item.get("name")?.as_str()?.to_string();
                let version = item.get("version")?.as_str()?.to_string();
                Some(Package {
                    name,
                    version,
                    description: String::new(),
                    manager: "pip".to_string(),
                })
            })
            .collect();

        log::info!("Found {} installed pip packages", packages.len());
        Ok(packages)
    }

    fn search(&self, _query: &str) -> Result<Vec<Package>, AppError> {
        // pip search has been disabled by PyPI since 2021
        Err(AppError::CommandFailed {
            cmd: "pip search".to_string(),
            stderr: "pip search is disabled. Use https://pypi.org to search for packages.".to_string(),
        })
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated pip packages...");
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        let output = run_command(cmd, &["list", "--outdated", "--format=json"])?;

        let items: Vec<Value> = serde_json::from_str(&output).map_err(|e| {
            AppError::ParseError {
                context: "pip list --outdated --format=json".to_string(),
                detail: e.to_string(),
            }
        })?;

        let packages: Vec<OutdatedPackage> = items
            .iter()
            .filter_map(|item| {
                let name = item.get("name")?.as_str()?.to_string();
                let current = item.get("version")?.as_str()?.to_string();
                let latest = item.get("latest_version")?.as_str()?.to_string();
                Some(OutdatedPackage {
                    name,
                    current_version: current,
                    latest_version: latest,
                    description: String::new(),
                    manager: "pip".to_string(),
                })
            })
            .collect();

        log::info!("Found {} outdated pip packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing pip package: {}", name);
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        run_command(cmd, &["install", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling pip package: {}", name);
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        run_command(cmd, &["uninstall", "-y", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating pip package: {}", name);
        let cmd = if command_exists("pip3") { "pip3" } else { "pip" };
        run_command(cmd, &["install", "--upgrade", name])?;
        Ok(())
    }
}
