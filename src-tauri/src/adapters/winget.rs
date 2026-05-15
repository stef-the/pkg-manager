use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

pub struct WingetAdapter;

impl PackageManagerAdapter for WingetAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "winget".to_string(),
            name: "winget".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("winget")
    }

    fn version(&self) -> String {
        run_command("winget", &["--version"])
            .ok()
            .map(|v| format!("winget {}", v.trim()))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed winget packages...");

        // winget list outputs JSON with --source winget
        let output = run_command("winget", &["list", "--accept-source-agreements", "--disable-interactivity"])?;

        // Parse the tabular output (winget doesn't have great JSON support for list)
        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        // Find the header line with dashes to determine column positions
        let mut data_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("---") || line.starts_with("Name") {
                data_start = i + 1;
                if line.starts_with("---") {
                    break;
                }
            }
        }

        for line in &lines[data_start..] {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            // Best-effort parsing of tabular output
            let parts: Vec<&str> = trimmed.splitn(3, "  ").filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                let name = parts[0].trim().to_string();
                let version = parts.get(1).map(|v| v.trim().to_string()).unwrap_or_default();
                packages.push(Package {
                    name,
                    version,
                    description: String::new(),
                    manager: "winget".to_string(),
                });
            }
        }

        log::info!("Found {} installed winget packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching winget for '{}'...", query);

        let output = run_command("winget", &["search", query, "--accept-source-agreements", "--disable-interactivity"])?;

        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut data_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("---") || line.contains("Name") {
                data_start = i + 1;
                if line.starts_with("---") {
                    break;
                }
            }
        }

        for line in &lines[data_start..] {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let parts: Vec<&str> = trimmed.splitn(3, "  ").filter(|s| !s.is_empty()).collect();
            if parts.len() >= 2 {
                packages.push(Package {
                    name: parts[0].trim().to_string(),
                    version: parts.get(1).map(|v| v.trim().to_string()).unwrap_or_default(),
                    description: String::new(),
                    manager: "winget".to_string(),
                });
            }
        }

        log::info!("Found {} winget search results for '{}'", packages.len(), query);
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated winget packages...");

        let output = run_command("winget", &["upgrade", "--accept-source-agreements", "--disable-interactivity"])?;

        let mut packages = Vec::new();
        let lines: Vec<&str> = output.lines().collect();

        let mut data_start = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("---") {
                data_start = i + 1;
                break;
            }
        }

        for line in &lines[data_start..] {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.contains("upgrade") {
                continue;
            }
            let parts: Vec<&str> = trimmed.splitn(4, "  ").filter(|s| !s.is_empty()).collect();
            if parts.len() >= 3 {
                packages.push(OutdatedPackage {
                    name: parts[0].trim().to_string(),
                    current_version: parts.get(1).map(|v| v.trim().to_string()).unwrap_or_default(),
                    latest_version: parts.get(2).map(|v| v.trim().to_string()).unwrap_or_default(),
                    description: String::new(),
                    manager: "winget".to_string(),
                });
            }
        }

        log::info!("Found {} outdated winget packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing winget package: {}", name);
        run_command("winget", &["install", name, "--accept-source-agreements", "--accept-package-agreements", "--disable-interactivity"])?;
        log::info!("Successfully installed {}", name);
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling winget package: {}", name);
        run_command("winget", &["uninstall", name, "--disable-interactivity"])?;
        log::info!("Successfully uninstalled {}", name);
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating winget package: {}", name);
        run_command("winget", &["upgrade", name, "--accept-source-agreements", "--accept-package-agreements", "--disable-interactivity"])?;
        log::info!("Successfully updated {}", name);
        Ok(())
    }
}
