use std::{sync::Arc, time::Duration};

use anyhow::Result;
use log::{debug, warn};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{
    api::process::{Command, CommandChild, CommandEvent},
    State, Window,
};
use tokio::{sync::Mutex, time::interval};

use crate::config::{
    daemon::{Daemon, LocalDaemon},
    pool::{LocalPool, Pool},
};
use crate::MinistoState;

pub async fn start_monerod(window: Window, state: State<'_, MinistoState>) {
    let pool_config = &state.config.lock().await.pool;
    // No need to continue if we're not configured to use a local pool.
    if let Pool::Local(LocalPool {
        daemon:
            Daemon::Local(LocalDaemon {
                blockchain_dir,
                monerod_verbosity,
            }),
        ..
    }) = pool_config
    {
        let verbosity_str = monerod_verbosity.to_string();
        let args = vec![
            "--zmq-pub",
            "tcp://127.0.0.1:18083",
            "--non-interactive",         // Don't accept stdin commands.
            "--p2p-use-ipv6",            // Enable IPv6 (will also use IPv4 as usual).
            "--prune-blockchain",        // Prune.
            "--sync-pruned-blocks",      // Download pruned blocks (save bandwidth).
            "--disable-dns-checkpoints", // Don't listen to MoneroPulse checkpoints.
            "--db-sync-mode",
            "safe:sync:250000000bytes", // Sync resiliently in case of system crash. (consider fast:async:250000000bytes in future)
            "--data-dir",
            blockchain_dir, // Store blockchain here.
            "--log-level",
            &verbosity_str, // Use configured log level.
        ];

        let (mut rx, _child) = Command::new_sidecar("monerod")
            .expect("failed to create `monerod` binary command")
            .args(args)
            .spawn()
            .expect("failed to start Monerod");

        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            // Read stdout.
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    debug!("{}", line);

                    // Send stdout event.
                    let html = ansi_to_html::convert_escaped(&line).unwrap_or(line) + "</br>";
                    window_clone
                        .emit("monerod-stdout", html)
                        .expect("failed to emit monerod stdout event");
                }
            }
        });

        let monerod_state = state.monerod.clone();
        tauri::async_runtime::spawn(async move {
            let mut interval = interval(Duration::from_secs(8));
            loop {
                interval.tick().await;
                // Get monerod status.
                match monerod_status(monerod_state.clone()).await {
                    Ok(status) => {
                        // Save status.
                        *monerod_state.status.lock().await = status;
                        // Send status event.
                        window
                            .emit("monerod-status", &status)
                            .expect("failed to emit monerod stdout event");
                    }
                    Err(e) => {
                        warn!("No response from Monerod: {}", e.root_cause());
                    }
                };
            }
        });
    } else {
        panic!("Only local pools are supported!");
    }
}

async fn monerod_status(state: Arc<MonerodState>) -> Result<Status> {
    let info: Info = state
        .client
        .request(Method::GET, "http://127.0.0.1:18081/json_rpc")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "0",
            "method": "get_info",
        }))
        .send()
        .await?
        .json()
        .await?;

    if info.result.busy_syncing {
        Ok(Status::Synchronizing)
    } else if info.result.offline {
        Ok(Status::Offline)
    } else if info.result.synchronized {
        Ok(Status::Running)
    } else if info.result.status != "OK" {
        Ok(Status::Error)
    } else {
        Ok(Status::Starting)
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
enum Status {
    Stopped,
    Starting,
    Offline,
    Synchronizing,
    Running,
    Error,
}

/// Response from monerod's `get_info` json rpc method.
#[derive(Deserialize)]
#[allow(unused)]
struct Info {
    id: String,
    jsonrpc: String,
    result: InfoResult,
}

#[derive(Deserialize)]
#[allow(unused)]
struct InfoResult {
    adjusted_time: u64,
    alt_blocks_count: u64,
    block_size_limit: u64,
    block_size_median: u64,
    block_weight_limit: u64,
    block_weight_median: u64,
    bootstrap_daemon_address: String,
    busy_syncing: bool,
    credits: u64,
    cumulative_difficulty: u64,
    cumulative_difficulty_top64: u64,
    database_size: u64,
    difficulty: u64,
    difficulty_top64: u64,
    free_space: u64,
    grey_peerlist_size: u64,
    height: u64,
    height_without_bootstrap: u64,
    incoming_connections_count: u64,
    mainnet: bool,
    nettype: String,
    offline: bool,
    outgoing_connections_count: u64,
    rpc_connections_count: u64,
    stagenet: bool,
    start_time: u64,
    status: String,
    synchronized: bool,
    target: u64,
    target_height: u64,
    testnet: bool,
    top_block_hash: String,
    top_hash: String,
    tx_count: u64,
    tx_pool_size: u64,
    untrusted: bool,
    update_available: bool,
    version: String,
    was_bootstrap_ever_used: bool,
    white_peerlist_size: u64,
    wide_cumulative_difficulty: String,
    wide_difficulty: String,
}

#[derive(Debug)]
pub struct MonerodState {
    client: reqwest::Client,
    pub child: Mutex<Option<CommandChild>>,
    status: Mutex<Status>,
}

impl MonerodState {
    pub fn new() -> MonerodState {
        MonerodState {
            client: reqwest::Client::new(),
            child: Mutex::new(None),
            status: Mutex::new(Status::Stopped),
        }
    }
}
