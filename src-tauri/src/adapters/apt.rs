use crate::adapters::{command_exists, run_command, PackageManagerAdapter};
use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;

/// APT adapter — works natively on Linux or through WSL on Windows.
pub struct AptAdapter;

impl AptAdapter {
    /// Get the command prefix: empty on Linux, "wsl" on Windows (WSL2).
    fn cmd_prefix() -> Vec<&'static str> {
        if cfg!(target_os = "windows") {
            vec!["wsl"]
        } else {
            vec![]
        }
    }

    fn run_apt(args: &[&str]) -> Result<String, AppError> {
        let prefix = Self::cmd_prefix();
        let mut full_args: Vec<&str> = prefix.clone();
        full_args.push("apt");
        full_args.extend_from_slice(args);

        if prefix.is_empty() {
            run_command("apt", args)
        } else {
            run_command("wsl", &full_args[1..])
        }
    }

    fn run_dpkg(args: &[&str]) -> Result<String, AppError> {
        let prefix = Self::cmd_prefix();
        if prefix.is_empty() {
            run_command("dpkg-query", args)
        } else {
            let mut full_args = vec!["dpkg-query"];
            full_args.extend_from_slice(args);
            run_command("wsl", &full_args)
        }
    }
}

impl PackageManagerAdapter for AptAdapter {
    fn info(&self) -> ManagerInfo {
        ManagerInfo {
            id: "apt".to_string(),
            name: if cfg!(target_os = "windows") {
                "apt (WSL)".to_string()
            } else {
                "apt".to_string()
            },
            available: self.is_available(),
            version: self.version(),
        }
    }

    fn is_available(&self) -> bool {
        if cfg!(target_os = "linux") {
            command_exists("apt")
        } else if cfg!(target_os = "windows") {
            // Check if WSL is available and has apt
            run_command("wsl", &["which", "apt"]).is_ok()
        } else {
            false
        }
    }

    fn version(&self) -> String {
        Self::run_apt(&["--version"])
            .ok()
            .and_then(|v| {
                // "apt 2.9.5 (amd64)" -> "apt 2.9.5"
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
        log::info!("Listing installed apt packages...");

        let output = Self::run_dpkg(&["-W", "--showformat=${Package}\\t${Version}\\t${Description}\\n"])?;

        let packages: Vec<Package> = output
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(3, '\t').collect();
                if parts.len() >= 2 {
                    Some(Package {
                        name: parts[0].trim().to_string(),
                        version: parts[1].trim().to_string(),
                        description: parts.get(2).unwrap_or(&"").trim().to_string(),
                        manager: "apt".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} installed apt packages", packages.len());
        Ok(packages)
    }

    fn search(&self, query: &str) -> Result<Vec<Package>, AppError> {
        log::info!("Searching apt for '{}'...", query);
        let output = Self::run_apt(&["search", "--names-only", query])?;

        let packages: Vec<Package> = output
            .lines()
            .take(30)
            .filter_map(|line| {
                if line.starts_with("Sorting") || line.starts_with("Full") || line.trim().is_empty() {
                    return None;
                }
                // Format: "package/suite version arch\n  description"
                let parts: Vec<&str> = line.splitn(2, '/').collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim().to_string();
                    Some(Package {
                        name,
                        version: String::new(),
                        description: String::new(),
                        manager: "apt".to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        log::info!("Found {} apt search results", packages.len());
        Ok(packages)
    }

    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError> {
        log::info!("Checking outdated apt packages...");
        let output = Self::run_apt(&["list", "--upgradable"])?;

        let packages: Vec<OutdatedPackage> = output
            .lines()
            .filter_map(|line| {
                if line.starts_with("Listing") || line.trim().is_empty() {
                    return None;
                }
                // Format: "package/suite version arch [upgradable from: old_version]"
                let slash = line.find('/')?;
                let name = line[..slash].to_string();
                let rest = &line[slash + 1..];
                let parts: Vec<&str> = rest.split_whitespace().collect();
                let latest = parts.get(0).unwrap_or(&"").to_string();
                let current = rest
                    .rsplit("from: ")
                    .next()
                    .map(|v| v.trim_end_matches(']').trim().to_string())
                    .unwrap_or_default();

                Some(OutdatedPackage {
                    name,
                    current_version: current,
                    latest_version: latest,
                    description: String::new(),
                    manager: "apt".to_string(),
                })
            })
            .collect();

        log::info!("Found {} outdated apt packages", packages.len());
        Ok(packages)
    }

    fn install(&self, name: &str) -> Result<(), AppError> {
        log::info!("Installing apt package: {}", name);
        Self::run_apt(&["install", "-y", name])?;
        Ok(())
    }

    fn uninstall(&self, name: &str) -> Result<(), AppError> {
        log::info!("Uninstalling apt package: {}", name);
        Self::run_apt(&["remove", "-y", name])?;
        Ok(())
    }

    fn update(&self, name: &str) -> Result<(), AppError> {
        log::info!("Updating apt package: {}", name);
        Self::run_apt(&["install", "--only-upgrade", "-y", name])?;
        Ok(())
    }
}
