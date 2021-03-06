use std::time::Duration;

use anyhow::{Error, Result};
use log::{debug, warn};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::json;
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    command,
    utils::platform::current_exe,
    Manager, State, Window,
};
use tokio::{sync::Mutex, time::interval};

use crate::{
    config::pool::{Pool, RemotePool},
    MinistoState,
};

async fn xmrig_status(state: &XmrigState) -> Result<String, Error> {
    let client = &state.client;
    let token = &state.bearer_token.lock().await;
    let res = client
        .get("http://127.0.0.1:3334/2/summary")
        .bearer_auth(token)
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn start_xmrig(window: Window, state: State<'_, MinistoState>) -> Result<()> {
    let config = &state.config.lock().await;

    // If a token was supplied in config, use it. Otherwise, generate one.
    let token = match &config.xmrig.bearer_token {
        Some(t) => t.to_owned(),
        None => {
            // Generate a 32 character bearer token.
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect()
        }
    };
    *state.xmrig.bearer_token.lock().await = token.to_string();
    let token_arg = format!("--http-access-token={}", &token);

    let pool_address = match &config.pool {
        Pool::Local { .. } => "127.0.0.1:3333".to_string(),
        Pool::Remote(RemotePool { ip, port }) => format!("{}:{}", ip, port),
    };

    let mut args = vec![
        "-o",
        &pool_address,
        "--coin=MONERO",
        &token_arg,
        "--http-enabled",
        "--http-no-restricted",
        "--http-port",
        "3334",
    ];

    if config.xmrig.verbose {
        args.push("--verbose");
    }

    #[cfg(unix)]
    let xmrig_path = match current_exe()?.parent() {
        Some(exec_dir) => format!("{}/{}", exec_dir.display(), "xmrig"),
        None => return Err(Error::msg("Failed to determine xmrig directory.")),
    };
    #[cfg(unix)]
    let (mut rx, child) = Command::new("pkexec")
        .args([xmrig_path])
        .args(args)
        .spawn()
        .expect("failed to start XMRig");

    #[cfg(not(unix))]
    let (mut rx, child) = Command::new_sidecar("xmrig")
        .expect("failed to create `xmrig` command")
        .args(args)
        .spawn()
        .expect("failed to start XMRig");

    // Store child so we can kill it on exit.
    *state.xmrig.child.lock().await = Some(child);

    let window_label = window.label();
    let window_clone = window.get_window(window_label).unwrap();
    tauri::async_runtime::spawn(async move {
        // Read stdout.
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                debug!("{}", line);

                // Send stdout event.
                let html = ansi_to_html::convert_escaped(&line).unwrap_or(line) + "</br>";
                window_clone
                    .emit("xmrig-stdout", html)
                    .expect("failed to emit xmrig stdout event");
            }
        }
    });

    let xmrig_state = state.xmrig.clone();
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(8));
        loop {
            interval.tick().await;
            // Get xmrig status.
            match xmrig_status(&xmrig_state).await {
                Ok(status) => {
                    // Send status event.
                    window
                        .emit("xmrig-status", &status)
                        .expect("failed to emit xmrig stdout event");
                }
                Err(e) => {
                    warn!("No response from XMRig: {}", e.root_cause());
                }
            };
        }
    });
    Ok(())
}

/// Kill XMRig.
pub async fn kill_xmrig(state: &XmrigState) -> Result<()> {
    match &mut *state.child.lock().await {
        Some(child) => {
            child.write(b"q")?;
        }
        None => return Err(Error::msg("XMRig child process not stored")),
    }
    Ok(())
}

#[command(async)]
pub async fn pause_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
    // Pause XMRig.
    let client = &state.xmrig.client;
    let token = &state.xmrig.bearer_token.lock().await;
    let res = match client
        .post("http://127.0.0.1:3334/json_rpc")
        .json(&json!({"method":"pause","id":1}))
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(r) => r
            .text()
            .await
            .unwrap_or_else(|_| "failed to parse XMRig response".to_string()),
        Err(e) => e.to_string(),
    };

    debug!("XMRig pause command result: \"{}\"", res);
    Ok(res)
}

#[command(async)]
pub async fn resume_mining(state: State<'_, MinistoState>) -> Result<String, ()> {
    // Resume XMRig.
    let client = &state.xmrig.client;
    let token = &state.xmrig.bearer_token.lock().await;
    let res = match client
        .post("http://127.0.0.1:3334/json_rpc")
        .json(&json!({"method":"resume","id":1}))
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(r) => r
            .text()
            .await
            .unwrap_or_else(|_| "failed to parse XMRig response".to_string()),
        Err(e) => e.to_string(),
    };

    debug!("XMRig resume command result: \"{}\"", res);
    Ok(res)
}

#[derive(Debug)]
pub struct XmrigState {
    client: reqwest::Client,
    bearer_token: Mutex<String>,
    pub child: Mutex<Option<CommandChild>>,
}

impl XmrigState {
    pub fn new() -> XmrigState {
        XmrigState {
            client: reqwest::Client::new(),
            bearer_token: Mutex::new(String::default()),
            child: Mutex::new(None),
        }
    }
}
