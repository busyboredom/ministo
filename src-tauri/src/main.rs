#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use log::debug;
use serde_json::json;
use tauri::api::process::{Command, CommandEvent};
use tauri::{command, State, Window};

#[command(async)]
async fn xmrig_status(state: State<'_, MinistoState>) -> Result<String, ()> {
  let client = &state.xmrig_client;
  let res = client
    .get("http://127.0.0.1:3334/2/summary")
    .bearer_auth(&state.xmrig_bearer_token)
    .send()
    .await
    .expect("failed to get xmrig summary using http api")
    .text()
    .await
    .expect("failed to parse xmrig summary http response as json");
  Ok(res)
}

#[command(async)]
async fn start_mining(window: Window) {
  start_xmrig(window);
}

fn start_xmrig(window: Window) {
  let (mut rx, _child) = Command::new_sidecar("xmrig")
    .expect("failed to create `xmrig` binary command")
    .args([
      "-o",
      "127.0.0.1:3333",
      "--coin=MONERO",
      "--verbose",
      "--http-access-token=12345",
      "--http-enabled",
      "--http-no-restricted",
      "--http-port",
      "3334",
    ])
    .spawn()
    .expect("failed to start XMRig");

  tauri::async_runtime::spawn(async move {
    // Read stdout.
    while let Some(event) = rx.recv().await {
      if let CommandEvent::Stdout(line) = event {
        debug!("XMRig Output: {}", line);

        // Send stdout event.
        let html = ansi_to_html::convert_escaped(&line).unwrap_or(line) + "</br>";
        window
          .emit("xmrig-stdout", html)
          .expect("failed to emit xmrig stdout event");
      }
    }
  });
}

#[command(async)]
async fn pause_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
  // Pause XMRig.
  let client = &state.xmrig_client;
  let res = match client
    .post("http://127.0.0.1:3334/json_rpc")
    .json(&json!({"method":"pause","id":1}))
    .bearer_auth(&state.xmrig_bearer_token)
    .send()
    .await
  {
    Ok(r) => r
      .text()
      .await
      .unwrap_or("failed to parse XMRig response".to_string()),
    Err(e) => e.to_string(),
  };

  debug!("XMRig pause command result: \"{}\"", res);
  Ok(res)
}

#[command(async)]
async fn resume_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
  // Resume XMRig.
  let client = &state.xmrig_client;
  let res = match client
    .post("http://127.0.0.1:3334/json_rpc")
    .json(&json!({"method":"resume","id":1}))
    .bearer_auth(&state.xmrig_bearer_token)
    .send()
    .await
  {
    Ok(r) => r
      .text()
      .await
      .unwrap_or("failed to parse XMRig response".to_string()),
    Err(e) => e.to_string(),
  };

  debug!("XMRig resume command result: \"{}\"", res);
  Ok(res)
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

struct MinistoState {
  xmrig_client: reqwest::Client,
  xmrig_bearer_token: String,
  xmrig_stdout: Vec<String>,
}

impl MinistoState {
  pub fn new() -> MinistoState {
    MinistoState {
      xmrig_client: reqwest::Client::new(),
      xmrig_bearer_token: "12345".to_string(),
      xmrig_stdout: Vec::new(),
    }
  }
}
