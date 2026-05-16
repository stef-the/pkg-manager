use serde::{Deserialize, Serialize};
use log;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub name: String,
    pub version: String,
    pub path: String,
    pub size_bytes: u64,
    pub app_type: String, // "app", "cli", "system"
    pub can_uninstall: bool,
}

/// Scan for installed applications on the system.
pub fn scan_apps() -> Vec<InstalledApp> {
    let mut apps = Vec::new();

    #[cfg(target_os = "macos")]
    {
        scan_macos_apps(&mut apps);
    }

    #[cfg(target_os = "windows")]
    {
        scan_windows_apps(&mut apps);
    }

    #[cfg(target_os = "linux")]
    {
        scan_linux_apps(&mut apps);
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    log::info!("Scanned {} installed apps", apps.len());
    apps
}

#[cfg(target_os = "macos")]
fn scan_macos_apps(apps: &mut Vec<InstalledApp>) {
    let dirs = ["/Applications", &format!("{}/Applications", std::env::var("HOME").unwrap_or_default())];

    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension().and_then(|e| e.to_str()) == Some("app") {
                    let name = entry_path.file_stem()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    if name.is_empty() {
                        continue;
                    }

                    // Try to get version from Info.plist
                    let version = get_macos_app_version(&entry_path);

                    // Get app size (rough estimate from metadata)
                    let size = get_dir_size_fast(&entry_path);

                    let is_system = dir == &"/Applications" && (
                        name == "Safari" || name == "Mail" || name == "Calendar" ||
                        name == "Messages" || name == "FaceTime" || name == "Maps" ||
                        name == "Photos" || name == "Music" || name == "TV" ||
                        name == "Podcasts" || name == "News" || name == "Stocks" ||
                        name == "Books" || name == "Freeform" || name == "Notes"
                    );

                    apps.push(InstalledApp {
                        name,
                        version,
                        path: entry_path.to_string_lossy().to_string(),
                        size_bytes: size,
                        app_type: if is_system { "system".to_string() } else { "app".to_string() },
                        can_uninstall: !is_system,
                    });
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn get_macos_app_version(app_path: &Path) -> String {
    let plist_path = app_path.join("Contents/Info.plist");
    if !plist_path.exists() {
        return String::new();
    }

    // Use defaults read to get version
    let output = std::process::Command::new("defaults")
        .args(["read", &plist_path.to_string_lossy(), "CFBundleShortVersionString"])
        .stdin(std::process::Stdio::null())
        .output();

    match output {
        Ok(o) if o.status.success() => {
            String::from_utf8_lossy(&o.stdout).trim().to_string()
        }
        _ => String::new(),
    }
}

#[cfg(target_os = "windows")]
fn scan_windows_apps(apps: &mut Vec<InstalledApp>) {
    // Read from registry
    let output = std::process::Command::new("reg")
        .args(["query", r"HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall", "/s"])
        .stdin(std::process::Stdio::null())
        .output();

    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        let mut current_name = String::new();
        let mut current_version = String::new();
        let mut current_path = String::new();
        let mut current_uninstall = String::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.starts_with("HKEY_") {
                // Save previous entry
                if !current_name.is_empty() {
                    apps.push(InstalledApp {
                        name: current_name.clone(),
                        version: current_version.clone(),
                        path: current_path.clone(),
                        size_bytes: 0,
                        app_type: "app".to_string(),
                        can_uninstall: !current_uninstall.is_empty(),
                    });
                }
                current_name.clear();
                current_version.clear();
                current_path.clear();
                current_uninstall.clear();
            } else if line.contains("DisplayName") {
                current_name = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            } else if line.contains("DisplayVersion") {
                current_version = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            } else if line.contains("InstallLocation") {
                current_path = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            } else if line.contains("UninstallString") {
                current_uninstall = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            }
        }
        // Don't forget last entry
        if !current_name.is_empty() {
            apps.push(InstalledApp {
                name: current_name,
                version: current_version,
                path: current_path,
                size_bytes: 0,
                app_type: "app".to_string(),
                can_uninstall: !current_uninstall.is_empty(),
            });
        }
    }

    // Also check user registry
    let output = std::process::Command::new("reg")
        .args(["query", r"HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall", "/s"])
        .stdin(std::process::Stdio::null())
        .output();

    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        let mut current_name = String::new();
        let mut current_version = String::new();
        let mut current_uninstall = String::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.starts_with("HKEY_") {
                if !current_name.is_empty() {
                    apps.push(InstalledApp {
                        name: current_name.clone(),
                        version: current_version.clone(),
                        path: String::new(),
                        size_bytes: 0,
                        app_type: "app".to_string(),
                        can_uninstall: !current_uninstall.is_empty(),
                    });
                }
                current_name.clear();
                current_version.clear();
                current_uninstall.clear();
            } else if line.contains("DisplayName") {
                current_name = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            } else if line.contains("DisplayVersion") {
                current_version = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            } else if line.contains("UninstallString") {
                current_uninstall = line.rsplit("REG_SZ").nth(0).unwrap_or("").trim().to_string();
            }
        }
        if !current_name.is_empty() {
            apps.push(InstalledApp {
                name: current_name,
                version: current_version,
                path: String::new(),
                size_bytes: 0,
                app_type: "app".to_string(),
                can_uninstall: !current_uninstall.is_empty(),
            });
        }
    }
}

#[cfg(target_os = "linux")]
fn scan_linux_apps(apps: &mut Vec<InstalledApp>) {
    let desktop_dirs = [
        "/usr/share/applications",
        "/usr/local/share/applications",
        &format!("{}/.local/share/applications", std::env::var("HOME").unwrap_or_default()),
    ];

    for dir in &desktop_dirs {
        let path = Path::new(dir);
        if !path.exists() {
            continue;
        }

        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                    continue;
                }

                if let Ok(content) = std::fs::read_to_string(&entry_path) {
                    let mut name = String::new();
                    let mut version = String::new();
                    let mut exec = String::new();

                    for line in content.lines() {
                        if line.starts_with("Name=") && name.is_empty() {
                            name = line[5..].to_string();
                        } else if line.starts_with("Version=") {
                            version = line[8..].to_string();
                        } else if line.starts_with("Exec=") {
                            exec = line[5..].to_string();
                        }
                    }

                    if !name.is_empty() && !content.contains("NoDisplay=true") {
                        apps.push(InstalledApp {
                            name,
                            version,
                            path: exec,
                            size_bytes: 0,
                            app_type: "app".to_string(),
                            can_uninstall: true,
                        });
                    }
                }
            }
        }
    }
}

/// Get a rough size of a directory by counting direct children sizes.
/// Doesn't recurse deeply to stay fast.
fn get_dir_size_fast(path: &Path) -> u64 {
    let mut total = 0u64;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(meta) = entry.metadata() {
                total += meta.len();
            }
        }
    }
    total
}
