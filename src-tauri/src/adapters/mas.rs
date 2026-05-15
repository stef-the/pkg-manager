use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct MasAdapter;

/// Parse a mas output line like "123456789 App Name (1.2.3)" into (id, name, version)
fn parse_mas_line(line: &str) -> Option<(String, String, String)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() < 2 {
        return None;
    }
    let app_id = parts[0].trim().to_string();
    let rest = parts[1];
    let (name, version) = if let Some(paren_start) = rest.rfind('(') {
        let name = rest[..paren_start].trim().to_string();
        let version = rest[paren_start + 1..]
            .trim_end_matches(')')
            .trim()
            .to_string();
        (name, version)
    } else {
        (rest.trim().to_string(), String::new())
    };
    Some((app_id, name, version))
}

impl PackageManagerAdapter for MasAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "mas".to_string(),
            name: "Mac App Store".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("mas")
    }

    fn version(&self) -> String {
        run_command("mas", &["version"])
            .ok()
            .map(|v| format!("mas {}", v.trim()))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Mac App Store apps...");
        let output = run_command("mas", &["list"])?;

        let packages: Vec<Package> = output
            .lines()
            .filter_map(|line| {
                let (app_id, name, version) = parse_mas_line(line)?;
                Some(Package {
                    name,
                    version,
                    description: app_id, // Store ID in description for later use
                    manager: "mas".to_string(),
                })
            })
            .collect();

        log::info!("Found {} installed Mac App Store apps", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Mac App Store for '{}'...", query);
        let output = run_command("mas", &["search", query])?;

        let packages: Vec<Package> = output
            .lines()
            .take(20)
            .filter_map(|line| {
                let (app_id, name, version) = parse_mas_line(line)?;
                Some(Package {
                    name,
                    version,
                    description: app_id,
                    manager: "mas".to_string(),
                })
            })
            .collect();

        log::info!("Found {} Mac App Store search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Mac App Store apps...");
        let output = run_command("mas", &["outdated"])?;

        let packages: Vec<OutdatedPackage> = output
            .lines()
            .filter_map(|line| {
                let (app_id, name, version) = parse_mas_line(line)?;
                Some(OutdatedPackage {
                    // Use "name|id" so the frontend can split and pass the ID for updates
                    name: format!("{}|{}", name, app_id),
                    current_version: version,
                    latest_version: "newer".to_string(),
                    description: app_id,
                    manager: "mas".to_string(),
                })
            })
            .collect();

        log::info!("Found {} outdated Mac App Store apps", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        // name might be an app ID directly, or "name|id" format
        let app_id = if name.contains('|') {
            name.split('|').last().unwrap_or(name)
        } else if name.chars().all(|c| c.is_ascii_digit()) {
            name
        } else {
            return Err(AppError::CommandFailed {
                cmd: "mas install".to_string(),
                stderr: format!("Mac App Store requires an app ID. Got: {}", name),
            });
        };
        log::info!("Installing Mac App Store app ID: {}", app_id);
        run_command("mas", &["install", app_id])?;
        Ok(())
    }

    fn uninstall(&self, _name: &str) -> Result<(), AppError> {
        Err(AppError::CommandFailed {
            cmd: "mas uninstall".to_string(),
            stderr: "Mac App Store apps must be removed from Finder or Launchpad".to_string(),
        })
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        let app_id = if name.contains('|') {
            name.split('|').last().unwrap_or(name)
        } else if name.chars().all(|c| c.is_ascii_digit()) {
            name
        } else {
            // Try to find the ID from the installed list
            if let Ok(output) = run_command("mas", &["list"]) {
                for line in output.lines() {
                    if let Some((id, app_name, _)) = parse_mas_line(line) {
                        if app_name == name {
                            log::info!("Resolved '{}' to app ID {}", name, id);
                            return run_command("mas", &["upgrade", &id]).map(|_| ()).map_err(|e| e);
                        }
                    }
                }
            }
            return Err(AppError::CommandFailed {
                cmd: "mas upgrade".to_string(),
                stderr: format!("Could not find app ID for '{}'", name),
            });
        };
        log::info!("Updating Mac App Store app ID: {}", app_id);
        run_command("mas", &["upgrade", app_id])?;
        Ok(())
    }
}
