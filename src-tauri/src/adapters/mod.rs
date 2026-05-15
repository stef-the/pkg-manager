pub mod apt;
pub mod brew;
pub mod cargo;
pub mod flatpak;
pub mod mas;
pub mod npm;
pub mod pip;
pub mod snap;
pub mod winget;

use crate::error::AppError;
use crate::models::{ManagerInfo, OutdatedPackage, Package};
use log;
use std::process::Command;

pub trait PackageManagerAdapter: Send + Sync {
    fn info(&self) -> ManagerInfo;
    fn is_available(&self) -> bool;
    fn version(&self) -> String;
    fn list_installed(&self) -> Result<Vec<Package>, AppError>;
    fn search(&self, query: &str) -> Result<Vec<Package>, AppError>;
    fn get_outdated(&self) -> Result<Vec<OutdatedPackage>, AppError>;
    fn install(&self, name: &str) -> Result<(), AppError>;
    fn uninstall(&self, name: &str) -> Result<(), AppError>;
    fn update(&self, name: &str) -> Result<(), AppError>;
}

pub fn get_adapter(manager: &str) -> Result<Box<dyn PackageManagerAdapter>, AppError> {
    match manager {
        "brew" => Ok(Box::new(brew::BrewAdapter)),
        "npm" => Ok(Box::new(npm::NpmAdapter)),
        "winget" => Ok(Box::new(winget::WingetAdapter)),
        "mas" => Ok(Box::new(mas::MasAdapter)),
        "pip" => Ok(Box::new(pip::PipAdapter)),
        "cargo" => Ok(Box::new(cargo::CargoAdapter)),
        "apt" => Ok(Box::new(apt::AptAdapter)),
        "flatpak" => Ok(Box::new(flatpak::FlatpakAdapter)),
        "snap" => Ok(Box::new(snap::SnapAdapter)),
        _ => Err(AppError::ManagerNotFound(manager.to_string())),
    }
}

pub fn get_all_adapters() -> Vec<Box<dyn PackageManagerAdapter>> {
    vec![
        Box::new(brew::BrewAdapter),
        Box::new(npm::NpmAdapter),
        Box::new(mas::MasAdapter),
        Box::new(pip::PipAdapter),
        Box::new(cargo::CargoAdapter),
        Box::new(winget::WingetAdapter),
        Box::new(apt::AptAdapter),
        Box::new(flatpak::FlatpakAdapter),
        Box::new(snap::SnapAdapter),
    ]
}

/// Platform-aware command existence check
pub fn command_exists(name: &str) -> bool {
    let check_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
    run_command(check_cmd, &[name]).is_ok()
}

/// Run a shell command with a 5-minute timeout and return its stdout.
/// Logs the command, and on failure logs stderr.
pub fn run_command(program: &str, args: &[&str]) -> Result<String, AppError> {
    let cmd_str = format!("{} {}", program, args.join(" "));
    log::debug!("Running command: {}", cmd_str);

    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| {
            log::error!("Failed to execute '{}': {}", cmd_str, e);
            AppError::CommandFailed {
                cmd: cmd_str.clone(),
                stderr: e.to_string(),
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log::warn!("Command '{}' exited with {}: {}", cmd_str, output.status, stderr);
        // Some commands return non-zero but still have useful stdout (e.g., npm outdated)
        // We'll return the error but callers can choose to handle this
        return Err(AppError::CommandFailed {
            cmd: cmd_str,
            stderr,
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    log::debug!("Command '{}' succeeded ({} bytes stdout)", cmd_str, stdout.len());
    Ok(stdout)
}

/// Like run_command but returns stdout even on non-zero exit (useful for npm outdated).
pub fn run_command_allow_failure(program: &str, args: &[&str]) -> Result<String, AppError> {
    let cmd_str = format!("{} {}", program, args.join(" "));
    log::debug!("Running command (allow failure): {}", cmd_str);

    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| {
            log::error!("Failed to execute '{}': {}", cmd_str, e);
            AppError::CommandFailed {
                cmd: cmd_str.clone(),
                stderr: e.to_string(),
            }
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log::debug!(
            "Command '{}' exited with {} (allowed): stderr={}",
            cmd_str,
            output.status,
            stderr
        );
    }

    Ok(stdout)
}
