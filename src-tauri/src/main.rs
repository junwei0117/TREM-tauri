// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    let tray_menu = SystemTrayMenu::new()
        .add_item(
            CustomMenuItem::new(
                "version".to_string(),
                format!("TREM Tauri v{}", env!("CARGO_PKG_VERSION")),
            )
            .disabled(),
        )
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "關閉"));

    let system_tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_tooltip("TREM Tauri | 臺灣即時地震監測");

    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => match matches.args["quiet"].value.as_bool() {
                    Some(true) => {}
                    _ => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                },
                Err(_) => {}
            }
            Ok(())
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--quiet"]),
        ))
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
