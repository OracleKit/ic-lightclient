use anyhow::Result;
use ic_lightclient_wire::outcalls::Config;
use crate::chain::ConfigManager;

pub struct OutcallsConfigManager;

impl ConfigManager for OutcallsConfigManager {
    type Config = Config;

    async fn process(config: String) -> Result<Config> {
        let config = serde_json::from_str(&config)?;
        Ok(config)
    }
}