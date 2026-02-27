#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct ProcessInfo {
    pid: u32,
    name: String,
    memory: u64,
}

#[tauri::command]
fn get_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut processes = Vec::new();
    for (pid, process) in sys.processes() {
        processes.push(ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().into_owned(),
            memory: process.memory(),
        });
    }
    // Sort by PID
    processes.sort_by_key(|p| p.pid);
    processes
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
