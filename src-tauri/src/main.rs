#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod monerod;
mod p2pool;
mod settings;
mod xmrig;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Arg;
use log::{error, info};
use tauri::{command, RunEvent, State, Window, Manager};
use tokio::{join, sync::Mutex};

use config::{default_configuraton_dir, Config};
use monerod::start_monerod;
use p2pool::start_p2pool;
use settings::{get_config, save_settings, select_blockchain_folder};
use xmrig::{kill_xmrig, pause_mining, resume_mining, start_xmrig, XmrigState};

#[command(async)]
async fn start_mining(window: Window, state: State<'_, MinistoState>) -> Result<(), String> {
    let (_, res, _) = join!(
        start_monerod(window.clone(), state.clone()),
        start_p2pool(window.clone(), state.clone()),
        start_xmrig(window, state)
    );
    res.map_err(|e| e.to_string())?;

    // Return result because of https://github.com/tauri-apps/tauri/issues/2533
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = clap::Command::new("Ministo")
        .about("A performant and user-friendly Monero mining interface")
        .arg(
            Arg::new("config")
                .long("config")
                .takes_value(true)
                .help("Path to your 'ministo.json' configuration file"),
        )
        .get_matches();
    let default_config_path = default_configuraton_dir().to_string_lossy().into_owned();
    let config_path = Path::new(matches.value_of("config").unwrap_or(&default_config_path));

    #[cfg_attr(not(debug_assertions), allow(unused_variables))]
    tauri::Builder::default()
        .manage(MinistoState::new(config_path.to_path_buf()))
        .invoke_handler(tauri::generate_handler![
            start_mining,
            pause_mining,
            resume_mining,
            select_blockchain_folder,
            save_settings,
            get_config
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, e| {
            if let RunEvent::ExitRequested { .. } = e {
                // Kill XMRig on exit.
                let xmrig_state = app_handle.state::<MinistoState>().xmrig.clone();
                tokio::spawn(async move {
                    if xmrig_state.child.lock().await.is_some() {
                        info!("Stopping XMRig");
                        kill_xmrig(&xmrig_state.clone())
                            .await
                            .unwrap_or_else(|e| error!("{}", e));
                    }
                });
            }
        })
}

#[derive(Debug)]
pub struct MinistoState {
    xmrig: Arc<XmrigState>,
    config: Arc<Mutex<Config>>,
    config_path: PathBuf,
}

impl MinistoState {
    pub fn new(config_path: PathBuf) -> MinistoState {
        let config = match Config::open(&config_path) {
            Ok(c) => c,
            Err(_) => {
                info!(
                    "A 'ministo.json' file could not be found in {}; creating ministo.json",
                    config_path.display()
                );
                let config = Config::new("");
                config
                    .save(&config_path)
                    .expect("failed to create configuration file");
                config
            }
        };
        MinistoState {
            xmrig: Arc::new(XmrigState::new()),
            config: Arc::new(Mutex::new(config)),
            config_path,
        }
    }
}
