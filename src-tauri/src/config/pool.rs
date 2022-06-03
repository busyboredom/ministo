use serde::{Deserialize, Serialize};

use super::daemon::{Daemon, LocalDaemon};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Pool {
    Local(LocalPool),
    Remote(RemotePool),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LocalPool {
    pub monero_address: Option<String>,
    pub chain: P2poolChain,
    /// Verbosity of P2Pool. Should be an integer between 0 and 6.
    pub p2pool_verbosity: u8,
    pub daemon: Daemon,
}

impl Default for LocalPool {
    fn default() -> Self {
        LocalPool {
            monero_address: None,
            daemon: Daemon::Local(LocalDaemon::default()),
            chain: P2poolChain::Main,
            p2pool_verbosity: 2, // Moderate verbosity
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RemotePool {
    pub ip: String,
    pub port: u16,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum P2poolChain {
    Main,
    Mini,
}
