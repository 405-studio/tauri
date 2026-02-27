#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{plugin::{Builder, TauriPlugin}, Runtime};

#[tauri::command]
fn restricted_command() -> String {
    "Success! ACL bypassed.".to_string()
}

pub fn init_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("restricted-plugin")
        .invoke_handler(tauri::generate_handler![restricted_command])
        .build()
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(init_plugin())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
