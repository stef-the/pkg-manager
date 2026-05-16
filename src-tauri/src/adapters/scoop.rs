use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct ScoopAdapter;

impl PackageManagerAdapter for ScoopAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "scoop".to_string(),
            name: "Scoop".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("scoop")
    }

    fn version(&self) -> String {
        // scoop --version outputs multiple lines, first is the version
        run_command_allow_failure("scoop", &["--version"])
            .ok()
            .and_then(|v| v.lines().next().map(|l| format!("scoop {}", l.trim())))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Scoop packages...");

        let output = run_command("scoop", &["list"])?;

        let mut packages = Vec::new();
        for line in output.lines().skip(2) {
            // Skip header lines
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('-') || trimmed.starts_with("Name") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: String::new(),
                    manager: "scoop".to_string(),
                });
            }
        }

        log::info!("Found {} installed Scoop packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Scoop for '{}'...", query);

        let output = run_command("scoop", &["search", query])?;

        let mut packages = Vec::new();
        for line in output.lines().skip(2) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('-') || trimmed.starts_with("Name") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if !parts.is_empty() {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts.get(1).unwrap_or(&"").to_string(),
                    description: String::new(),
                    manager: "scoop".to_string(),
                });
            }
        }

        log::info!("Found {} Scoop search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Scoop packages...");

        let output = run_command("scoop", &["status"])?;

        let mut packages = Vec::new();
        for line in output.lines().skip(2) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('-') || trimmed.starts_with("Name") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                packages.push(OutdatedPackage {
                    name: parts[0].to_string(),
                    current_version: parts[1].to_string(),
                    latest_version: parts[2].to_string(),
                    description: String::new(),
                    manager: "scoop".to_string(),
                });
            }
        }

        log::info!("Found {} outdated Scoop packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Scoop package: {}", name);
        run_command("scoop", &["install", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Scoop package: {}", name);
        run_command("scoop", &["uninstall", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Scoop package: {}", name);
        run_command("scoop", &["update", name])?;
        Ok(())
    }
}
