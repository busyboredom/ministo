use std::{default::Default, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Daemon {
    Local(LocalDaemon),
    Remote(RemoteDaemon),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LocalDaemon {
    pub blockchain_dir: String,
    /// Verbosity of Monerod. Should be an integer between 0 and 4.
    pub monerod_verbosity: u8,
}

impl Default for LocalDaemon {
    fn default() -> Self {
        LocalDaemon {
            blockchain_dir: default_blockchain_dir().to_string_lossy().into_owned(),
            monerod_verbosity: 0, // Low verbosity
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RemoteDaemon {
    ip: String,
    port: u16,
}

/// Default blockchain location according to
/// https://monerodocs.org/interacting/overview/#data-directory
pub fn default_blockchain_dir() -> PathBuf {
    if cfg!(window) {
        PathBuf::from(r"C:\ProgramData\bitmonero\")
    } else {
        home::home_dir()
            .expect("failed to determine home directory")
            .join(".bitmonero/")
    }
}
