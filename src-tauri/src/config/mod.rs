pub mod daemon;
pub mod pool;

use std::string::ToString;
use std::{
    default::Default,
    path::{Path, PathBuf},
};
use std::{fs, fs::File, io::Read};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use self::pool::{LocalPool, Pool};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub pool: Pool,
    pub xmrig: XmrigConfig,
}

impl Config {
    pub fn new(monero_address: &str) -> Config {
        Config {
            pool: Pool::Local(LocalPool {
                monero_address: Some(monero_address.to_string()),
                ..Default::default()
            }),
            ..Default::default()
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

impl Default for Config {
    fn default() -> Self {
        Config {
            pool: Pool::Local(LocalPool::default()),
            xmrig: XmrigConfig {
                verbose: false,
                bearer_token: None,
            },
        }
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
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct XmrigConfig {
    pub verbose: bool,
    /// Bearer token for API access. If left blank, a secure token will be generated randomly.
    pub bearer_token: Option<String>,
}
