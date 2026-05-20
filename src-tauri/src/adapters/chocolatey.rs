use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct ChocolateyAdapter;

impl PackageManagerAdapter for ChocolateyAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "choco".to_string(),
            name: "Chocolatey".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("choco")
    }

    fn version(&self) -> String {
        run_command("choco", &["--version"])
            .ok()
            .map(|v| format!("choco {}", v.trim()))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Chocolatey packages...");
        let output = run_command("choco", &["list", "--local-only", "--no-color"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.contains("packages installed") || trimmed.starts_with("Chocolatey") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: String::new(),
                    manager: "choco".to_string(),
                });
            }
        }

        log::info!("Found {} installed Chocolatey packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Chocolatey for '{}'...", query);
        let output = run_command("choco", &["search", query, "--no-color"])?;

        let mut packages = Vec::new();
        for line in output.lines().take(20) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.contains("packages found") || trimmed.starts_with("Chocolatey") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: String::new(),
                    manager: "choco".to_string(),
                });
            }
        }

        log::info!("Found {} Chocolatey search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Chocolatey packages...");
        let output = run_command("choco", &["outdated", "--no-color"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            // Format: "name|currentVersion|availableVersion|pinned"
            if trimmed.contains('|') {
                let parts: Vec<&str> = trimmed.split('|').collect();
                if parts.len() >= 3 && parts[0] != "Output is package name" {
                    packages.push(OutdatedPackage {
                        name: parts[0].trim().to_string(),
                        current_version: parts[1].trim().to_string(),
                        latest_version: parts[2].trim().to_string(),
                        description: String::new(),
                        manager: "choco".to_string(),
                    });
                }
            }
        }

        log::info!("Found {} outdated Chocolatey packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Chocolatey package: {}", name);
        run_command("choco", &["install", name, "-y", "--no-color"])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Chocolatey package: {}", name);
        run_command("choco", &["uninstall", name, "-y", "--no-color"])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Chocolatey package: {}", name);
        run_command("choco", &["upgrade", name, "-y", "--no-color"])?;
        Ok(())
    }
}
