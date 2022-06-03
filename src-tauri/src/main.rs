#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod monerod;
mod p2pool;
mod permissions;
mod settings;
mod xmrig;

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use clap::Arg;
use log::{error, info, warn};
#[cfg(debug_assertions)]
use tauri::Manager;
use tauri::{command, State, Window};
use tokio::{join, sync::Mutex};

use config::{default_configuraton_dir, Config};
use monerod::start_monerod;
use p2pool::start_p2pool;
#[cfg(unix)]
use permissions::unix::UserPermissions;
use settings::{get_config, save_settings, select_blockchain_folder};
use xmrig::{pause_mining, resume_mining, start_xmrig, XmrigState};

#[command(async)]
async fn start_mining(window: Window, state: State<'_, MinistoState>) -> Result<(), String> {
    let (_, res) = join!(
        start_monerod(window.clone(), state.clone()),
        start_p2pool(window.clone(), state.clone())
    );
    res.map_err(|e| e.to_string())?;

    #[cfg(unix)]
    if let Some(user_perms) = state.user_perms.clone().lock().await.as_mut() {
        if let Err(e) = user_perms.elevate_permissions() {
            error!(
                "Failed to elevate permissions; running XMRig as non-root: {}",
                e
            );
        };
        start_xmrig(window, state).await;
        if let Err(e) = user_perms.reduce_permissions() {
            error!("Failed to reduce permissions: {}", e);
        };
    }

    #[cfg(not(unix))]
    start_xmrig(window, state).await;

    // Return result because of https://github.com/tauri-apps/tauri/issues/2533
    Ok(())
}

fn main() {
    env_logger::init();

    // Attempt to reduce permissions if possible.
    #[cfg(unix)]
    let user_perms = match UserPermissions::new() {
        Ok(mut user) if user.is_root() => {
            if let Err(e) = user.reduce_permissions() {
                error!("{}", e);
            }
            Some(user)
        }
        Ok(user) => {
            warn!("Ministo is not running as root. Hashrate will be low.");
            Some(user)
        }
        Err(e) => {
            error!("{}", e);
            None
        }
    };

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

    #[cfg_attr(not(unix), allow(unused_mut))]
    let mut ministo_state = MinistoState::new(config_path.to_path_buf());
    #[cfg(unix)]
    {
        ministo_state.user_perms = Arc::new(Mutex::new(user_perms));
    }

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug)]
pub struct MinistoState {
    xmrig: Arc<XmrigState>,
    config: Arc<Mutex<Config>>,
    config_path: PathBuf,
    #[cfg(unix)]
    user_perms: Arc<Mutex<Option<UserPermissions>>>,
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
            #[cfg(unix)]
            user_perms: Arc::new(Mutex::new(None)),
        }
    }
}
