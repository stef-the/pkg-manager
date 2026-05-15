use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;
use serde_json::Value;

pub struct BrewAdapter;

impl BrewAdapter {
    fn parse_formula_package(formula: &Value) -> Option<Package> {
        let name = formula.get("name")?.as_str()?.to_string();
        let version = formula
            .get("installed")
            .and_then(|i| i.as_array())
            .and_then(|arr| arr.first())
            .and_then(|v| v.get("version"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let description = formula
            .get("desc")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();

        Some(Package {
            name,
            version,
            description,
            manager: "brew".to_string(),
        })
    }

    fn parse_cask_package(cask: &Value) -> Option<Package> {
        let name = cask.get("token")?.as_str()?.to_string();
        let version = cask
            .get("installed")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let description = cask
            .get("desc")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();

        Some(Package {
            name,
            version,
            description,
            manager: "brew".to_string(),
        })
    }
}

impl PackageManagerAdapter for BrewAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "brew".to_string(),
            name: "Homebrew".to_string(),
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        command_exists("brew")
    }

    fn version(&self) -> String {
        run_command("brew", &["--version"])
            .ok()
            .and_then(|v| v.lines().next().map(|l| l.trim().to_string()))
            .unwrap_or_default()
    }

    fn list_installed(&self) -> Result<Vec<Package>, AppError> {
        log::info!("Listing installed Homebrew packages...");

        let output = run_command("brew", &["info", "--installed", "--json=v2"])?;
        let json: Value = serde_json::from_str(&output).map_err(|e| {
            log::error!("Failed to parse brew JSON: {}", e);
            AppError::ParseError {
                context: "brew info --installed --json=v2".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();

        if let Some(formulae) = json.get("formulae").and_then(|f| f.as_array()) {
            for formula in formulae {
                if let Some(pkg) = Self::parse_formula_package(formula) {
                    packages.push(pkg);
                }
            }
        }

        if let Some(casks) = json.get("casks").and_then(|c| c.as_array()) {
            for cask in casks {
                if let Some(pkg) = Self::parse_cask_package(cask) {
                    packages.push(pkg);
                }
            }
        }

        log::info!("Found {} installed Homebrew packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching Homebrew for '{}'...", query);

        // brew search returns a simple list; we then get details for matching packages
        let output = run_command("brew", &["search", "--formula", query])?;
        let names: Vec<&str> = output
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .take(20) // limit results to avoid slow info lookups
            .collect();

        if names.is_empty() {
            return Ok(Vec::new());
        }

        // Get detailed info for the found packages
        let mut args = vec!["info", "--json=v2"];
        args.extend_from_slice(&names);

        let info_output = run_command("brew", &args)?;
        let json: Value = serde_json::from_str(&info_output).map_err(|e| {
            log::error!("Failed to parse brew search results JSON: {}", e);
            AppError::ParseError {
                context: "brew info --json=v2 (search results)".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();
        if let Some(formulae) = json.get("formulae").and_then(|f| f.as_array()) {
            for formula in formulae {
                let name = formula
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or_default()
                    .to_string();
                let version = formula
                    .get("versions")
                    .and_then(|v| v.get("stable"))
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let description = formula
                    .get("desc")
                    .and_then(|d| d.as_str())
                    .unwrap_or("")
                    .to_string();

                packages.push(Package {
                    name,
                    version,
                    description,
                    manager: "brew".to_string(),
                });
            }
        }

        log::info!("Found {} Homebrew search results for '{}'", packages.len(), query);
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated Homebrew packages...");

        let output = run_command("brew", &["outdated", "--json=v2"])?;
        let json: Value = serde_json::from_str(&output).map_err(|e| {
            log::error!("Failed to parse brew outdated JSON: {}", e);
            AppError::ParseError {
                context: "brew outdated --json=v2".to_string(),
                detail: e.to_string(),
            }
        })?;

        let mut packages = Vec::new();

        if let Some(formulae) = json.get("formulae").and_then(|f| f.as_array()) {
            for formula in formulae {
                let name = formula
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or_default()
                    .to_string();
                let current_version = formula
                    .get("installed_versions")
                    .and_then(|v| v.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let latest_version = formula
                    .get("current_version")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                packages.push(OutdatedPackage {
                    name,
                    current_version,
                    latest_version,
                    description: String::new(),
                    manager: "brew".to_string(),
                });
            }
        }

        if let Some(casks) = json.get("casks").and_then(|c| c.as_array()) {
            for cask in casks {
                let name = cask
                    .get("name")
                    .and_then(|n| n.as_str())
                    .unwrap_or_default()
                    .to_string();
                let current_version = cask
                    .get("installed_versions")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let latest_version = cask
                    .get("current_version")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                packages.push(OutdatedPackage {
                    name,
                    current_version,
                    latest_version,
                    description: String::new(),
                    manager: "brew".to_string(),
                });
            }
        }

        log::info!("Found {} outdated Homebrew packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing Homebrew package: {}", name);
        run_command("brew", &["install", name])?;
        log::info!("Successfully installed {}", name);
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling Homebrew package: {}", name);
        run_command("brew", &["uninstall", name])?;
        log::info!("Successfully uninstalled {}", name);
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating Homebrew package: {}", name);
        run_command("brew", &["upgrade", name])?;
        log::info!("Successfully updated {}", name);
        Ok(())
    }
}
