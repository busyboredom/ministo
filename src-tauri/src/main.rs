#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::debug;
use tauri::api::process::{Command, CommandEvent};

#[tauri::command(async)]
async fn print_status() -> String {
  reqwest::get("http://127.0.0.1:3334/2/summary")
    .await
    .expect("failed to get xmrig summary using http api")
    .text()
    .await
    .expect("failed to parse xmrig summary http response as json")
}

fn main() {
  env_logger::init();

  let (mut rx, _child) = Command::new_sidecar("xmrig")
    .expect("failed to create `xmrig` binary command")
    .args([
      "-o",
      "127.0.0.1:3333",
      "--coin=MONERO",
      "--verbose",
      "--http-port",
      "3334",
    ])
    .spawn()
    .expect("Failed to spawn xmrig sidecar");

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = rx.recv().await {
      if let CommandEvent::Stdout(line) = event {
        debug!("XMRig Output: {}", line);
      }
    }
  });

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![print_status])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
