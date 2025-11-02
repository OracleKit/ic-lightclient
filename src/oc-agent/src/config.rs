use anyhow::{anyhow, Context, Result};
use ic_lightclient_oc_utils::IcpConfig;
use serde::Deserialize;
use std::{fs::read_to_string, sync::OnceLock};

static INNER: OnceLock<ConfigSchema> = OnceLock::new();

#[derive(Deserialize)]
struct ConfigSchema {
    icp: IcpConfig,
}

pub struct Config {}

impl Config {
    pub fn init(file_path: &str) -> Result<()> {
        let config = read_to_string(file_path).context(format!("Config file not found: {}", file_path))?;
        let config: ConfigSchema = toml::from_str(&config).context("Error while parsing config file")?;
        INNER.set(config).map_err(|_| anyhow!("Attempting to reinitialize config."))?;

        Ok(())
    }

    pub fn icp() -> IcpConfig {
        INNER.get().unwrap().icp.clone()
    }
}
