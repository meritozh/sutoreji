// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use sutoreji::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_dir_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
