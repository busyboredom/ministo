use std::path::{Path, PathBuf};
use std::string::ToString;
use std::{fs, fs::File, io::Read};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub pool: Pool,
    pub xmrig: XmrigConfig,
}

impl Config {
    pub fn new(monero_address: &str) -> Config {
        Config {
            pool: Pool::Local {
                monero_address: monero_address.to_string(),
                blockchain_dir: default_blockchain_dir().to_string_lossy().into_owned(),
                chain: P2poolChain::Main,
                verbosity: 2,
            },
            xmrig: XmrigConfig {
                verbose: false,
                bearer_token: None,
            },
        }
    }

    /// Open configuration file, returning an error if it doesn't exist. Panics if the file cannot be
    /// parsed.
    pub fn open(path: &Path) -> Result<Config> {
        let mut full_path = path.to_path_buf();
        if !path.ends_with("ministo.json") {
            full_path.push("ministo.json");
        }

        let mut file_contents = String::new();
        File::open(&full_path)?.read_to_string(&mut file_contents)?;

        let config: Config =
            serde_json::from_str(&file_contents).expect("failed to parse ministo.json");
        Ok(config)
    }

    /// Create a new configuration file.
    pub fn save(&self, path: &Path) -> Result<()> {
        let mut full_path = path.to_path_buf();
        if !path.ends_with("ministo.json") {
            full_path.push("ministo.json");
        }

        fs::write(full_path, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }
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

/// Default configuration location.
pub fn default_configuraton_dir() -> PathBuf {
    if cfg!(window) {
        home::home_dir()
            .expect("failed to determine home directory")
            .join(r"AppData\Local\")
    } else {
        home::home_dir()
            .expect("failed to determine home directory")
            .join(".config/")
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Pool {
    Local {
        monero_address: String,
        blockchain_dir: String,
        chain: P2poolChain,
        /// Verbosity of P2Pool. Should be an integer between 0 and 6.
        verbosity: u8,
    },
    Remote {
        ip: String,
        port: u16,
    },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum P2poolChain {
    Main,
    Mini,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct XmrigConfig {
    pub verbose: bool,
    /// Bearer token for API access. If left blank, a secure token will be generated randomly.
    pub bearer_token: Option<String>,
}
