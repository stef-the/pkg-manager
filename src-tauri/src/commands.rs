use crate::adapters::{self, run_command, run_command_allow_failure};
use crate::models::{ManagerInfo, OutdatedPackage, Package, SystemStats};
use log;
use tauri::Manager;

/// Helper: run blocking work on the Tokio thread pool so it never blocks IPC.
async fn blocking<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| format!("Task panicked: {}", e))?
}

/// Helper for infallible blocking work.
async fn blocking_infallible<F, T>(f: F) -> T
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .unwrap_or_else(|e| panic!("Task panicked: {}", e))
}

#[tauri::command]
pub async fn list_packages(manager: String) -> Result<Vec<Package>, String> {
    blocking(move || {
        log::info!("Command: list_packages({})", manager);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.list_installed().map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn search_packages(manager: String, query: String) -> Result<Vec<Package>, String> {
    blocking(move || {
        log::info!("Command: search_packages({}, '{}')", manager, query);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.search(&query).map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn get_outdated(manager: String) -> Result<Vec<OutdatedPackage>, String> {
    blocking(move || {
        log::info!("Command: get_outdated({})", manager);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.get_outdated().map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn install_package(manager: String, name: String) -> Result<(), String> {
    blocking(move || {
        log::info!("Command: install_package({}, {})", manager, name);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.install(&name).map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn uninstall_package(manager: String, name: String) -> Result<(), String> {
    blocking(move || {
        log::info!("Command: uninstall_package({}, {})", manager, name);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.uninstall(&name).map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn update_package(manager: String, name: String) -> Result<(), String> {
    blocking(move || {
        log::info!("Command: update_package({}, {})", manager, name);
        let adapter = adapters::get_adapter(&manager).map_err(|e| e.to_string())?;
        if !adapter.is_available() {
            return Err(format!("Package manager '{}' is not available", manager));
        }
        adapter.update(&name).map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn get_package_managers() -> Vec<ManagerInfo> {
    log::info!("Command: get_package_managers");
    let all_adapters = adapters::get_all_adapters();
    let mut handles = Vec::new();

    for adapter in all_adapters {
        handles.push(tokio::task::spawn_blocking(move || {
            let available = adapter.is_available();
            let version = if available { adapter.version() } else { String::new() };
            let info = adapter.info();
            // Override with our quiet check results
            ManagerInfo {
                id: info.id,
                name: info.name,
                available,
                version,
            }
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(info) = handle.await {
            results.push(info);
        }
    }
    results
}

#[tauri::command]
pub async fn run_cleanup(manager: String) -> Result<String, String> {
    blocking(move || {
        log::info!("Command: run_cleanup({})", manager);
        let result = match manager.as_str() {
            "brew" => run_command("brew", &["cleanup", "--prune=all"]),
            "npm" => run_command("npm", &["cache", "clean", "--force"]),
            "pip" => run_command_allow_failure("pip3", &["cache", "purge"]),
            "winget" => run_command_allow_failure("winget", &["source", "reset", "--force"]),
            "apt" => run_command_allow_failure("apt", &["autoremove", "-y"]),
            _ => return Err(format!("Cleanup not supported for '{}'", manager)),
        };
        result.map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn run_doctor(manager: String) -> Result<String, String> {
    blocking(move || {
        log::info!("Command: run_doctor({})", manager);
        let result = match manager.as_str() {
            "brew" => run_command_allow_failure("brew", &["doctor"]),
            "npm" => run_command_allow_failure("npm", &["doctor"]),
            "pip" => run_command_allow_failure("pip3", &["check"]),
            "winget" => run_command_allow_failure("winget", &["source", "update"]),
            "apt" => run_command_allow_failure("apt", &["update"]),
            _ => return Err(format!("Doctor not supported for '{}'", manager)),
        };
        result.map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn run_terminal_command(manager: String, args: String) -> Result<String, String> {
    blocking(move || {
        log::info!("Command: run_terminal_command({}, '{}')", manager, args);
        let program = match manager.as_str() {
            "brew" => "brew",
            "npm" => "npm",
            "pip" => if crate::adapters::command_exists("pip3") { "pip3" } else { "pip" },
            "cargo" => "cargo",
            "mas" => "mas",
            "winget" => "winget",
            "apt" => "apt",
            _ => return Err(format!("Unknown manager '{}'", manager)),
        };
        let arg_vec: Vec<&str> = args.split_whitespace().collect();
        if arg_vec.is_empty() {
            return Err("No arguments provided".to_string());
        }
        run_command_allow_failure(program, &arg_vec).map_err(|e| e.to_string())
    }).await
}

#[tauri::command]
pub async fn read_log_file(app_handle: tauri::AppHandle) -> Result<String, String> {
    let log_dir = app_handle.path().app_log_dir()
        .map_err(|e| format!("Failed to get log directory: {}", e))?;
    blocking(move || {
        log::info!("Command: read_log_file");
        let log_file = log_dir.join("pkg-manager.log");
        if !log_file.exists() {
            return Ok("No log file found yet.".to_string());
        }
        std::fs::read_to_string(&log_file)
            .map(|content| {
                let lines: Vec<&str> = content.lines().collect();
                let start = if lines.len() > 500 { lines.len() - 500 } else { 0 };
                lines[start..].join("\n")
            })
            .map_err(|e| format!("Failed to read log file: {}", e))
    }).await
}

#[tauri::command]
pub async fn open_log_directory(app_handle: tauri::AppHandle) -> Result<(), String> {
    let log_dir = app_handle.path().app_log_dir()
        .map_err(|e| format!("Failed to get log directory: {}", e))?;
    log::info!("Command: open_log_directory");
    #[cfg(target_os = "macos")]
    { std::process::Command::new("open").arg(&log_dir).spawn().map_err(|e| format!("{}", e))?; }
    #[cfg(target_os = "windows")]
    { std::process::Command::new("explorer").arg(&log_dir).spawn().map_err(|e| format!("{}", e))?; }
    #[cfg(target_os = "linux")]
    { std::process::Command::new("xdg-open").arg(&log_dir).spawn().map_err(|e| format!("{}", e))?; }
    Ok(())
}

#[tauri::command]
pub async fn get_package_detail(manager: String, name: String) -> Result<crate::models::PackageDetail, String> {
    blocking(move || {
        log::info!("Command: get_package_detail({}, {})", manager, name);
        use crate::models::PackageDetail;
        match manager.as_str() {
            "brew" => {
                let output = run_command("brew", &["info", "--json=v2", &name]).map_err(|e| e.to_string())?;
                let json: serde_json::Value = serde_json::from_str(&output).map_err(|e| format!("Failed to parse: {}", e))?;
                let formula = json.get("formulae").and_then(|f| f.as_array()).and_then(|a| a.first());
                let cask = json.get("casks").and_then(|c| c.as_array()).and_then(|a| a.first());
                let pkg = formula.or(cask).ok_or_else(|| format!("Package '{}' not found", name))?;
                let deps = pkg.get("dependencies").and_then(|d| d.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();
                Ok(PackageDetail {
                    name: pkg.get("name").or(pkg.get("token")).and_then(|n| n.as_str()).unwrap_or(&name).to_string(),
                    version: pkg.get("versions").and_then(|v| v.get("stable")).and_then(|v| v.as_str())
                        .or(pkg.get("version").and_then(|v| v.as_str())).unwrap_or_default().to_string(),
                    description: pkg.get("desc").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                    manager: "brew".to_string(),
                    homepage: pkg.get("homepage").and_then(|h| h.as_str()).unwrap_or("").to_string(),
                    license: pkg.get("license").and_then(|l| l.as_str()).unwrap_or("Unknown").to_string(),
                    repository: pkg.get("urls").and_then(|u| u.get("stable")).and_then(|s| s.get("url")).and_then(|u| u.as_str()).unwrap_or("").to_string(),
                    dependencies: deps,
                    install_size: String::new(),
                    installed_on: pkg.get("installed").and_then(|i| i.as_array()).and_then(|a| a.first()).and_then(|v| v.get("installed_on")).and_then(|d| d.as_str()).unwrap_or("").to_string(),
                })
            }
            "npm" => {
                let output = run_command_allow_failure("npm", &["view", &name, "--json"]).map_err(|e| e.to_string())?;
                let json: serde_json::Value = serde_json::from_str(&output).map_err(|e| format!("Failed to parse: {}", e))?;
                let deps = json.get("dependencies").and_then(|d| d.as_object()).map(|obj| obj.keys().cloned().collect()).unwrap_or_default();
                Ok(PackageDetail {
                    name: json.get("name").and_then(|n| n.as_str()).unwrap_or(&name).to_string(),
                    version: json.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    description: json.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                    manager: "npm".to_string(),
                    homepage: json.get("homepage").and_then(|h| h.as_str()).unwrap_or("").to_string(),
                    license: json.get("license").and_then(|l| l.as_str()).unwrap_or("Unknown").to_string(),
                    repository: json.get("repository").and_then(|r| r.get("url")).and_then(|u| u.as_str()).unwrap_or("").to_string(),
                    dependencies: deps,
                    install_size: String::new(),
                    installed_on: String::new(),
                })
            }
            _ => Err(format!("Unsupported manager '{}'", manager)),
        }
    }).await
}

#[tauri::command]
pub async fn enrich_descriptions(manager: String, names: Vec<String>) -> Result<Vec<(String, String)>, String> {
    blocking(move || {
        log::info!("Command: enrich_descriptions({}, {} packages)", manager, names.len());
        let mut results: Vec<(String, String)> = Vec::new();

        if manager != "brew" {
            return Ok(results);
        }

        // Process in chunks of 30
        for chunk in names.chunks(30) {
            let mut args: Vec<&str> = vec!["info", "--json=v2"];
            args.extend(chunk.iter().map(|s| s.as_str()));

            if let Ok(output) = run_command("brew", &args) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
                    if let Some(formulae) = json.get("formulae").and_then(|f| f.as_array()) {
                        for formula in formulae {
                            let name = formula.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let desc = formula.get("desc").and_then(|d| d.as_str()).unwrap_or("");
                            if !name.is_empty() && !desc.is_empty() {
                                results.push((name.to_string(), desc.to_string()));
                            }
                        }
                    }
                    if let Some(casks) = json.get("casks").and_then(|c| c.as_array()) {
                        for cask in casks {
                            let name = cask.get("token").and_then(|n| n.as_str()).unwrap_or("");
                            let desc = cask.get("desc").and_then(|d| d.as_str()).unwrap_or("");
                            if !name.is_empty() && !desc.is_empty() {
                                results.push((name.to_string(), desc.to_string()));
                            }
                        }
                    }
                }
            }
        }

        log::info!("Enriched {} packages with descriptions", results.len());
        Ok(results)
    }).await
}

#[tauri::command]
pub fn get_package_info_url(manager: String, name: String) -> Result<String, String> {
    match manager.as_str() {
        "brew" => Ok(format!("https://formulae.brew.sh/formula/{}", name)),
        "npm" => Ok(format!("https://www.npmjs.com/package/{}", name)),
        "pip" => Ok(format!("https://pypi.org/project/{}", name)),
        "cargo" => Ok(format!("https://crates.io/crates/{}", name)),
        "mas" => Ok(format!("https://apps.apple.com/app/id{}", name)),
        "winget" => Ok(format!("https://winget.run/pkg/{}", name)),
        "apt" => Ok(format!("https://packages.ubuntu.com/search?keywords={}", name)),
        _ => Err(format!("No URL pattern for '{}'", manager)),
    }
}

#[tauri::command]
pub async fn get_package_icon_url(manager: String, name: String) -> Result<String, String> {
    blocking(move || {
        match manager.as_str() {
            "npm" => Ok(format!("https://www.npmjs.com/npm-avatar/eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9/{}", name)),
            "brew" => {
                let output = run_command("brew", &["info", "--json=v2", &name]);
                if let Ok(json_str) = output {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
                        if let Some(homepage) = json.get("formulae").and_then(|f| f.as_array()).and_then(|a| a.first()).and_then(|f| f.get("homepage")).and_then(|h| h.as_str()) {
                            return Ok(format!("https://www.google.com/s2/favicons?domain={}&sz=32", homepage));
                        }
                    }
                }
                Err("No icon available".to_string())
            }
            _ => Err("No icon available".to_string()),
        }
    }).await
}

#[tauri::command]
pub fn import_packages(contents: String) -> Result<Vec<(String, String)>, String> {
    log::info!("Command: import_packages ({} bytes)", contents.len());
    let mut packages = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') { continue; }
        let parts: Vec<&str> = trimmed.split('\t').collect();
        if parts.len() >= 2 {
            let manager = parts[0].trim().to_string();
            let name = parts[1].trim().to_string();
            if ["brew", "npm", "winget", "pip", "cargo", "apt", "nix", "scoop"].contains(&manager.as_str()) {
                packages.push((manager, name));
            }
        }
    }
    Ok(packages)
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    log::info!("Command: open_url({})", url);
    #[cfg(target_os = "macos")]
    { std::process::Command::new("open").arg(&url).spawn().map_err(|e| format!("{}", e))?; }
    #[cfg(target_os = "windows")]
    { std::process::Command::new("cmd").args(["/C", "start", &url]).spawn().map_err(|e| format!("{}", e))?; }
    #[cfg(target_os = "linux")]
    { std::process::Command::new("xdg-open").arg(&url).spawn().map_err(|e| format!("{}", e))?; }
    Ok(())
}

#[tauri::command]
pub async fn get_storage_info() -> crate::models::StorageInfo {
    blocking_infallible(|| {
        log::info!("Command: get_storage_info");
        let (total_kb, avail_kb) = match std::process::Command::new("df").args(["-k", "/"]).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let line = stdout.lines().nth(1).unwrap_or("");
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    (parts[1].parse::<u64>().unwrap_or(0), parts[3].parse::<u64>().unwrap_or(0))
                } else { (0, 0) }
            }
            Err(_) => (0, 0),
        };
        let total = total_kb * 1024;
        let available = avail_kb * 1024;
        let used = total.saturating_sub(available);
        fn fmt(bytes: u64) -> String {
            if bytes >= 1_000_000_000_000 { format!("{:.1}TB", bytes as f64 / 1e12) }
            else if bytes >= 1_000_000_000 { format!("{:.1}GB", bytes as f64 / 1e9) }
            else if bytes >= 1_000_000 { format!("{:.0}MB", bytes as f64 / 1e6) }
            else { format!("{:.0}KB", bytes as f64 / 1e3) }
        }
        let pct = if total > 0 { (used as f64 / total as f64 * 100.0).round() as u64 } else { 0 };
        crate::models::StorageInfo { disk_total: fmt(total), disk_used: fmt(used), disk_available: fmt(available), disk_pct: pct }
    }).await
}

#[tauri::command]
pub async fn install_manager(manager: String) -> Result<String, String> {
    blocking(move || {
        log::info!("Command: install_manager({})", manager);
        let result = match manager.as_str() {
            "brew" => run_command_allow_failure("bash", &["-c", "/bin/bash -c \"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""]),
            "mas" => run_command("brew", &["install", "mas"]),
            "pip" => if crate::adapters::command_exists("brew") { run_command("brew", &["install", "python3"]) } else { return Err("Install Python from https://python.org".to_string()); },
            "cargo" => run_command_allow_failure("bash", &["-c", "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"]),
            "npm" => if crate::adapters::command_exists("brew") { run_command("brew", &["install", "node"]) } else { return Err("Install Node.js from https://nodejs.org".to_string()); },
            "nix" => run_command_allow_failure("bash", &["-c", "curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install"]),
            _ => return Err(format!("Auto-install not supported for '{}'", manager)),
        };
        result.map_err(|e| e.to_string())
    }).await
}

// --- Cache ---

#[tauri::command]
pub fn save_cache(app_handle: tauri::AppHandle, key: String, data: String) -> Result<(), String> {
    let cache_dir = app_handle.path().app_cache_dir().map_err(|e| format!("{}", e))?;
    std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    std::fs::write(cache_dir.join(format!("{}.json", key)), &data).map_err(|e| format!("{}", e))
}

#[tauri::command]
pub fn load_cache(app_handle: tauri::AppHandle, key: String) -> Result<String, String> {
    let cache_dir = app_handle.path().app_cache_dir().map_err(|e| format!("{}", e))?;
    let path = cache_dir.join(format!("{}.json", key));
    if !path.exists() { return Err("Cache miss".to_string()); }
    std::fs::read_to_string(&path).map_err(|e| format!("{}", e))
}

// --- Pinning ---

#[tauri::command]
pub fn get_pinned_packages(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let path = app_handle.path().app_config_dir().map_err(|e| format!("{}", e))?.join("pinned.json");
    if !path.exists() { return Ok(Vec::new()); }
    let data = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_pinned_packages(app_handle: tauri::AppHandle, packages: Vec<String>) -> Result<(), String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| format!("{}", e))?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    let data = serde_json::to_string(&packages).map_err(|e| e.to_string())?;
    std::fs::write(config_dir.join("pinned.json"), &data).map_err(|e| e.to_string())
}

// --- History ---

#[tauri::command]
pub fn append_history(app_handle: tauri::AppHandle, entry: String) -> Result<(), String> {
    let data_dir = app_handle.path().app_data_dir().map_err(|e| format!("{}", e))?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new().create(true).append(true)
        .open(data_dir.join("history.jsonl")).map_err(|e| e.to_string())?;
    writeln!(file, "{}", entry).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn read_history(app_handle: tauri::AppHandle, limit: usize) -> Result<Vec<String>, String> {
    let path = app_handle.path().app_data_dir().map_err(|e| format!("{}", e))?.join("history.jsonl");
    if !path.exists() { return Ok(Vec::new()); }
    let data = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let lines: Vec<String> = data.lines().map(String::from).collect();
    let start = if lines.len() > limit { lines.len() - limit } else { 0 };
    Ok(lines[start..].to_vec())
}

// --- Notifications ---

#[tauri::command]
pub fn send_notification(app_handle: tauri::AppHandle, title: String, body: String) -> Result<(), String> {
    log::info!("Notification: {} - {}", title, body);
    use tauri::Emitter;
    app_handle.emit("notification", serde_json::json!({ "title": title, "body": body })).map_err(|e| e.to_string())
}

// --- Window/Tray ---

#[tauri::command]
pub fn set_window_title(app_handle: tauri::AppHandle, title: String) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.set_title(&title).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn set_tray_tooltip(app_handle: tauri::AppHandle, tooltip: String) -> Result<(), String> {
    if let Some(tray) = app_handle.tray_by_id("main") {
        tray.set_tooltip(Some(&tooltip)).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn save_window_state(app_handle: tauri::AppHandle, state: String) -> Result<(), String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| format!("{}", e))?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    std::fs::write(config_dir.join("window-state.json"), &state).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_window_state(app_handle: tauri::AppHandle) -> Result<String, String> {
    let path = app_handle.path().app_config_dir().map_err(|e| format!("{}", e))?.join("window-state.json");
    if !path.exists() { return Err("No saved state".to_string()); }
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

// --- App Scanner ---

#[tauri::command]
pub async fn scan_installed_apps() -> Vec<crate::apps::InstalledApp> {
    blocking_infallible(|| {
        log::info!("Command: scan_installed_apps");
        crate::apps::scan_apps()
    }).await
}

#[tauri::command]
pub async fn uninstall_app(path: String) -> Result<(), String> {
    blocking(move || {
        log::info!("Command: uninstall_app({})", path);
        let app_path = std::path::Path::new(&path);
        if !app_path.exists() {
            return Err(format!("Path not found: {}", path));
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("osascript")
                .args(["-e", &format!("tell application \"Finder\" to delete POSIX file \"{}\"", path)])
                .stdin(std::process::Stdio::null())
                .output()
                .map_err(|e| format!("Failed to move to Trash: {}", e))?;
            return Ok(());
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("gio").args(["trash", &path]).stdin(std::process::Stdio::null()).output()
                .map_err(|e| format!("Failed to trash: {}", e))?;
            return Ok(());
        }
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd").args(["/C", &path]).stdin(std::process::Stdio::null()).spawn()
                .map_err(|e| format!("Failed to run uninstaller: {}", e))?;
            return Ok(());
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        Err("Uninstall not supported".to_string())
    }).await
}

#[tauri::command]
pub fn get_system_stats() -> SystemStats {
    log::info!("Command: get_system_stats");
    SystemStats {
        os: format!("{} {}", std::env::consts::OS, std::env::consts::ARCH),
        arch: std::env::consts::ARCH.to_string(),
        hostname: hostname::get().map(|h| h.to_string_lossy().to_string()).unwrap_or_else(|_| "unknown".to_string()),
    }
}
