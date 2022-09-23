use anyhow::{Error, Result};
use log::debug;
use tauri::{
    api::process::{Command, CommandEvent},
    State, Window,
};

use crate::config::pool::{LocalPool, P2poolChain, Pool};
use crate::MinistoState;

pub async fn start_p2pool(window: Window, state: State<'_, MinistoState>) -> Result<()> {
    let pool_config = &state.config.lock().await.pool;
    // No need to continue if we're not configured to use a local pool.
    if let Pool::Local(LocalPool {
        monero_address,
        chain,
        p2pool_verbosity,
        ..
    }) = pool_config
    {
        let address = monero_address
            .as_ref()
            .ok_or_else(|| Error::msg("Monero address not configured"))?;
        let verbosity_str = p2pool_verbosity.to_string();
        let mut args = vec![
            "--host",
            "127.0.0.1",
            "--light-mode",
            "--loglevel",
            &verbosity_str,
            "--wallet",
            address,
        ];
        if let P2poolChain::Mini = chain {
            args.push("--mini");
        }

        let (mut rx, _child) = Command::new_sidecar("p2pool")
            .expect("failed to create `p2pool` binary command")
            .args(args)
            .spawn()
            .expect("failed to start P2Pool");

        tauri::async_runtime::spawn(async move {
            // Read stdout.
            while let Some(event) = rx.recv().await {
                let line = match event {
                    CommandEvent::Stdout(line) => line,
                    CommandEvent::Stderr(line) => line,
                    _ => continue
                };
                debug!("{}", line);

                // Send stdout event.
                let html = ansi_to_html::convert_escaped(&line).unwrap_or(line) + "</br>";
                window
                    .emit("p2pool-stdout", html)
                    .expect("failed to emit p2pool stdout event");
            }
        });
    } else {
        return Err(Error::msg(
            "Only local pools are supported. Have you been messing with your configuration file?",
        ));
    }
    Ok(())
}
