#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{generate_context};

fn main() {
  tauri::Builder::default()
    .run(generate_context!())
    .expect("error while running tauri application");
}
