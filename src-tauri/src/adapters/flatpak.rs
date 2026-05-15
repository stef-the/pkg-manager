use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct FlatpakAdapter;

impl PackageManagerAdapter for FlatpakAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "flatpak".to_string(),
            name: "Flatpak".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("flatpak")
    }

    fn version(&self) -> String {
        run_command("flatpak", &["--version"])
            .ok()
            .and_then(|v| {
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
        log::info!("Listing installed Flatpak apps...");
        let output = run_command("flatpak", &["list", "--app", "--columns=application,version,name"])?;

        let packages: Vec<Package> = output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 2 {
                    let app_id = parts[0].trim();
                    let version = parts.get(1).map(|v| v.trim()).unwrap_or("");
                    let name = parts.get(2).map(|n| n.trim()).unwrap_or(app_id);
                    Some(Package {
                        name: name.to_string(),
                        version: version.to_string(),
                        description: app_id.to_string(),
                        manager: "flatpak".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} installed Flatpak apps", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Flatpak for '{}'...", query);
        let output = run_command("flatpak", &["search", query, "--columns=application,version,name,description"])?;

        let packages: Vec<Package> = output
            .lines()
            .take(20)
            .filter_map(|line| {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 3 {
                    Some(Package {
                        name: parts.get(2).unwrap_or(&"").trim().to_string(),
                        version: parts.get(1).unwrap_or(&"").trim().to_string(),
                        description: parts.get(3).unwrap_or(&"").trim().to_string(),
                        manager: "flatpak".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} Flatpak search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Flatpak updates are handled via flatpak update");
        Ok(Vec::new())
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Flatpak app: {}", name);
        run_command("flatpak", &["install", "-y", "flathub", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Flatpak app: {}", name);
        run_command("flatpak", &["uninstall", "-y", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Flatpak app: {}", name);
        run_command("flatpak", &["update", "-y", name])?;
        Ok(())
    }
}
