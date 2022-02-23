#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod xmrig;

use tauri::{command, Window};

use xmrig::*;

#[command(async)]
async fn start_mining(window: Window) {
  start_xmrig(window);
}

fn main() {
  env_logger::init();

  tauri::Builder::default()
    .manage(MinistoState::new())
    .invoke_handler(tauri::generate_handler![
      xmrig_status,
      start_mining,
      pause_mining,
      resume_mining
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

pub struct MinistoState {
  xmrig: XmrigState,
}

impl MinistoState {
  pub fn new() -> MinistoState {
    MinistoState {
      xmrig: XmrigState::new(),
    }
  }
}
