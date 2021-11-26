#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::info;
use tauri::command;

#[command(async)]
fn print_status() -> String {
  info!("Running");
  "Running".to_string()
}

fn main() {
  env_logger::init();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![print_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
