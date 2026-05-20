use crate::adapters::{command_exists, run_command, run_command_allow_failure, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct PacmanAdapter;

impl PackageManagerAdapter for PacmanAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "pacman".to_string(),
            name: "Pacman".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("pacman")
    }

    fn version(&self) -> String {
        run_command("pacman", &["--version"])
            .ok()
            .and_then(|v| {
                v.lines()
                    .find(|l| l.contains("Pacman") || l.contains("pacman"))
                    .map(|l| l.trim().to_string())
            })
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Pacman packages...");
        let output = run_command("pacman", &["-Q"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    description: String::new(),
                    manager: "pacman".to_string(),
                });
            }
        }

        log::info!("Found {} installed Pacman packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Pacman for '{}'...", query);
        let output = run_command("pacman", &["-Ss", query])?;

        let mut packages = Vec::new();
        let mut lines = output.lines().peekable();
        while let Some(line) = lines.next() {
            // Format: "repo/name version\n    description"
            if line.starts_with(' ') { continue; }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].split('/').last().unwrap_or(parts[0]).to_string();
                let version = parts[1].to_string();
                let description = lines.peek()
                    .map(|l| l.trim().to_string())
                    .unwrap_or_default();
                packages.push(Package {
                    name,
                    version,
                    description,
                    manager: "pacman".to_string(),
                });
            }
            if packages.len() >= 20 { break; }
        }

        log::info!("Found {} Pacman search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Pacman packages...");
        let output = run_command_allow_failure("pacman", &["-Qu"])?;

        let mut packages = Vec::new();
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() { continue; }
            // Format: "name currentVersion -> newVersion"
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 && parts[2] == "->" {
                packages.push(OutdatedPackage {
                    name: parts[0].to_string(),
                    current_version: parts[1].to_string(),
                    latest_version: parts[3].to_string(),
                    description: String::new(),
                    manager: "pacman".to_string(),
                });
            }
        }

        log::info!("Found {} outdated Pacman packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Pacman package: {}", name);
        run_command("pacman", &["-S", "--noconfirm", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Pacman package: {}", name);
        run_command("pacman", &["-R", "--noconfirm", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Pacman package: {}", name);
        run_command("pacman", &["-S", "--noconfirm", name])?;
        Ok(())
    }
}
