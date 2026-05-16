mod adapters;
mod apps;
mod commands;
mod error;
mod models;

use tauri::Manager;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::LogDir {
                        file_name: Some("pkg-manager".into()),
                    }),
                    Target::new(TargetKind::Stdout),
                ])
                .max_file_size(5_000_000)
                .rotation_strategy(RotationStrategy::KeepAll)
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .level(if cfg!(debug_assertions) {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .build(),
        )
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::list_packages,
            commands::search_packages,
            commands::get_outdated,
            commands::install_package,
            commands::uninstall_package,
            commands::update_package,
            commands::get_package_managers,
            commands::get_system_stats,
            commands::run_cleanup,
            commands::run_doctor,
            commands::run_terminal_command,
            commands::read_log_file,
            commands::open_log_directory,
            commands::get_package_detail,
            commands::get_package_info_url,
            commands::open_url,
            commands::get_package_icon_url,
            commands::import_packages,
            commands::install_manager,
            commands::get_storage_info,
            commands::save_cache,
            commands::load_cache,
            commands::get_pinned_packages,
            commands::set_pinned_packages,
            commands::append_history,
            commands::read_history,
            commands::send_notification,
            commands::enrich_descriptions,
            commands::scan_installed_apps,
            commands::uninstall_app,
            commands::set_window_title,
            commands::set_tray_tooltip,
            commands::save_window_state,
            commands::load_window_state,
        ])
        .setup(|app| {
            log::info!("Pkg Manager starting up");

            // Build tray menu
            let show_item =
                MenuItem::with_id(app, "show", "Show Pkg Manager", true, None::<&str>)?;
            let refresh_item =
                MenuItem::with_id(app, "refresh", "Refresh Packages", true, None::<&str>)?;
            let outdated_item =
                MenuItem::with_id(app, "outdated", "Check Outdated", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &refresh_item, &outdated_item, &quit_item])?;

            // Build tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(true)
                .menu(&menu)
                .tooltip("Pkg Manager")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            log::info!("Quit requested from tray");
                            app.exit(0);
                        }
                        "show" | "refresh" | "outdated" => {
                            log::info!("{} requested from tray", event.id.as_ref());
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                                // Emit event so frontend can handle the action
                                use tauri::Emitter;
                                let _ = app.emit("tray-action", event.id.as_ref());
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            log::info!("System tray initialized");
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                log::info!("Window close requested — hiding to tray");
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
