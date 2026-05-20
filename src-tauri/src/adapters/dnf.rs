use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct DnfAdapter;

impl PackageManagerAdapter for DnfAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "dnf".to_string(),
            name: "DNF".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("dnf")
    }

    fn version(&self) -> String {
        run_command("dnf", &["--version"])
            .ok()
            .and_then(|v| v.lines().next().map(|l| format!("dnf {}", l.trim())))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed DNF packages...");
        let output = run_command("dnf", &["list", "installed", "--quiet"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("Installed") {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                // Name might include .arch suffix like "git.x86_64"
                let name = parts[0].split('.').next().unwrap_or(parts[0]).to_string();
                packages.push(Package {
                    name,
                    version: parts[1].to_string(),
                    description: String::new(),
                    manager: "dnf".to_string(),
                });
            }
        }

        log::info!("Found {} installed DNF packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching DNF for '{}'...", query);
        let output = run_command("dnf", &["search", query, "--quiet"])?;

        let mut packages = Vec::new();
        for line in output.lines().take(20) {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with("=") {
                continue;
            }
            // Format: "name.arch : description"
            if let Some(colon_pos) = trimmed.find(" : ") {
                let name_part = &trimmed[..colon_pos];
                let desc = &trimmed[colon_pos + 3..];
                let name = name_part.split('.').next().unwrap_or(name_part).trim().to_string();
                packages.push(Package {
                    name,
                    version: String::new(),
                    description: desc.trim().to_string(),
                    manager: "dnf".to_string(),
                });
            }
        }

        log::info!("Found {} DNF search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated DNF packages...");
        let output = run_command("dnf", &["check-update", "--quiet"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].split('.').next().unwrap_or(parts[0]).to_string();
                packages.push(OutdatedPackage {
                    name,
                    current_version: String::new(),
                    latest_version: parts[1].to_string(),
                    description: String::new(),
                    manager: "dnf".to_string(),
                });
            }
        }

        log::info!("Found {} outdated DNF packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing DNF package: {}", name);
        run_command("dnf", &["install", "-y", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling DNF package: {}", name);
        run_command("dnf", &["remove", "-y", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating DNF package: {}", name);
        run_command("dnf", &["upgrade", "-y", name])?;
        Ok(())
    }
}
