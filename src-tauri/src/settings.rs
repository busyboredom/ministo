use tauri::{api::dialog::FileDialogBuilder, command, State, Window};

use crate::{
    config::{
        daemon::{Daemon, LocalDaemon},
        pool::{LocalPool, Pool},
        Config,
    },
    MinistoState,
};

#[command]
pub async fn save_settings(
    state: State<'_, MinistoState>,
    address: String,
    folder: String,
) -> Result<(), String> {
    let config_path = &state.config_path;
    let mut config = state.config.lock().await;

    match &mut config.pool {
        Pool::Local(LocalPool {
            monero_address,
            daemon: Daemon::Local(LocalDaemon { blockchain_dir, .. }),
            ..
        }) => {
            *monero_address = Some(address);
            *blockchain_dir = folder;
        }
        _ => Err("")?,
    }

    config.save(config_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub async fn get_config(state: State<'_, MinistoState>) -> Result<Config, String> {
    let config = state.config.lock().await;
    Ok(config.clone())
}

#[command]
pub async fn select_blockchain_folder(window: Window) {
    FileDialogBuilder::new().pick_folder(move |selected_path| {
        if selected_path.is_some() {
            window
                .emit("blockchain-folder-selected", selected_path)
                .expect("failed to emit blockchain folder selected event")
        }
    })
}
