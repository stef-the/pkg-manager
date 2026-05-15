use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct SnapAdapter;

impl PackageManagerAdapter for SnapAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "snap".to_string(),
            name: "Snap".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("snap")
    }

    fn version(&self) -> String {
        run_command("snap", &["version"])
            .ok()
            .and_then(|v| {
                // First line: "snap    2.xx.x"
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
        log::info!("Listing installed Snap packages...");
        let output = run_command("snap", &["list"])?;

        let packages: Vec<Package> = output
            .lines()
            .skip(1) // skip header
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(Package {
                        name: parts[0].to_string(),
                        version: parts[1].to_string(),
                        description: String::new(),
                        manager: "snap".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} installed Snap packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Snap for '{}'...", query);
        let output = run_command("snap", &["find", query])?;

        let packages: Vec<Package> = output
            .lines()
            .skip(1)
            .take(20)
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    Some(Package {
                        name: parts[0].to_string(),
                        version: parts[1].to_string(),
                        description: parts[4..].join(" "),
                        manager: "snap".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} Snap search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Snap auto-updates; no manual outdated check");
        Ok(Vec::new())
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Snap package: {}", name);
        run_command("snap", &["install", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Snap package: {}", name);
        run_command("snap", &["remove", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Snap package: {}", name);
        run_command("snap", &["refresh", name])?;
        Ok(())
    }
}
