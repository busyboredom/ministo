use log::debug;
use serde_json::json;
use tauri::api::process::{Command, CommandEvent};
use tauri::{command, State, Window};

use crate::MinistoState;

#[command(async)]
pub async fn xmrig_status(state: State<'_, MinistoState>) -> Result<String, ()> {
  let client = &state.xmrig.client;
  let res = client
    .get("http://127.0.0.1:3334/2/summary")
    .bearer_auth(&state.xmrig.bearer_token)
    .send()
    .await
    .expect("failed to get xmrig summary using http api")
    .text()
    .await
    .expect("failed to parse xmrig summary http response as json");
  Ok(res)
}

pub fn start_xmrig(window: Window) {
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
pub async fn pause_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
  // Pause XMRig.
  let client = &state.xmrig.client;
  let res = match client
    .post("http://127.0.0.1:3334/json_rpc")
    .json(&json!({"method":"pause","id":1}))
    .bearer_auth(&state.xmrig.bearer_token)
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
pub async fn resume_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
  // Resume XMRig.
  let client = &state.xmrig.client;
  let res = match client
    .post("http://127.0.0.1:3334/json_rpc")
    .json(&json!({"method":"resume","id":1}))
    .bearer_auth(&state.xmrig.bearer_token)
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

pub struct XmrigState {
  client: reqwest::Client,
  bearer_token: String,
}

impl XmrigState {
  pub fn new() -> XmrigState {
    XmrigState {
      client: reqwest::Client::new(),
      bearer_token: "12345".to_string(),
    }
  }
}
