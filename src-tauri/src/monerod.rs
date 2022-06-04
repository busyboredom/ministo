use log::debug;
use tauri::{
    api::process::{Command, CommandEvent},
    State, Window,
};

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

        tauri::async_runtime::spawn(async move {
            // Read stdout.
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    debug!("{}", line);

                    // Send stdout event.
                    let html = ansi_to_html::convert_escaped(&line).unwrap_or(line) + "</br>";
                    window
                        .emit("monerod-stdout", html)
                        .expect("failed to emit monerod stdout event");
                }
            }
        });
    } else {
        panic!("Only local pools are supported!");
    }
}
