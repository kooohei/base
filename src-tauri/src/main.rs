
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_blur};
use window_shadows::set_shadow;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            
            #[cfg(target_os = "macos")]
            apply_vibrancy(&win, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            
            #[cfg(target_os = "windows")]
            apply_blur(&win, Some((10, 10, 10, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&win, true).unwrap();


            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
