pub mod apt;
pub mod brew;
pub mod cargo;
pub mod flatpak;
pub mod mas;
pub mod nix;
pub mod npm;
pub mod pip;
pub mod scoop;
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
        "nix" => Ok(Box::new(nix::NixAdapter)),
        "scoop" => Ok(Box::new(scoop::ScoopAdapter)),
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
        Box::new(nix::NixAdapter),
        Box::new(scoop::ScoopAdapter),
    ]
}

/// Platform-aware command existence check (quiet — no logging on failure)
pub fn command_exists(name: &str) -> bool {
    let check_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
    Command::new(check_cmd)
        .arg(name)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run a shell command with a 90-second timeout. Returns stdout on success.
/// Stdin is closed to prevent interactive prompts from hanging.
pub fn run_command(program: &str, args: &[&str]) -> Result<String, AppError> {
    let cmd_str = format!("{} {}", program, args.join(" "));
    log::debug!("Running command: {}", cmd_str);

    let mut child = Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            log::error!("Failed to execute '{}': {}", cmd_str, e);
            AppError::CommandFailed {
                cmd: cmd_str.clone(),
                stderr: e.to_string(),
            }
        })?;

    // Wait with 90 second timeout
    let timeout = std::time::Duration::from_secs(300);
    let start = std::time::Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(_status)) => break,
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    log::error!("Command '{}' timed out after 90s", cmd_str);
                    return Err(AppError::CommandFailed {
                        cmd: cmd_str,
                        stderr: "Command timed out after 90 seconds".to_string(),
                    });
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                return Err(AppError::CommandFailed {
                    cmd: cmd_str,
                    stderr: format!("Failed to wait for process: {}", e),
                });
            }
        }
    }

    let output = child.wait_with_output().map_err(|e| {
        AppError::CommandFailed {
            cmd: cmd_str.clone(),
            stderr: e.to_string(),
        }
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        log::warn!("Command '{}' exited with {}: {}", cmd_str, output.status, stderr);
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
/// Also has a 90-second timeout.
pub fn run_command_allow_failure(program: &str, args: &[&str]) -> Result<String, AppError> {
    let cmd_str = format!("{} {}", program, args.join(" "));
    log::debug!("Running command (allow failure): {}", cmd_str);

    let mut child = Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            log::error!("Failed to execute '{}': {}", cmd_str, e);
            AppError::CommandFailed {
                cmd: cmd_str.clone(),
                stderr: e.to_string(),
            }
        })?;

    let timeout = std::time::Duration::from_secs(300);
    let start = std::time::Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    log::error!("Command '{}' timed out after 90s (allow_failure)", cmd_str);
                    return Err(AppError::CommandFailed {
                        cmd: cmd_str,
                        stderr: "Command timed out after 90 seconds".to_string(),
                    });
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                return Err(AppError::CommandFailed {
                    cmd: cmd_str,
                    stderr: format!("Failed to wait for process: {}", e),
                });
            }
        }
    }

    let output = child.wait_with_output().map_err(|e| {
        AppError::CommandFailed { cmd: cmd_str.clone(), stderr: e.to_string() }
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
