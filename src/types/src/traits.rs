use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

pub trait ConfigManager<Config> {
    fn new(config: String) -> impl std::future::Future<Output = Self>;
    fn get_config(&self) -> &Config;
}

pub trait ConfigManagerDyn<Config> {
    fn get_config(&self) -> &Config;
}

impl<Config, T: ConfigManager<Config>> ConfigManagerDyn<Config> for T {
    fn get_config(&self) -> &Config {
        self.get_config()
    }
}

pub trait ConsensusManager<Config> {
    type StatePayload: Serialize + Debug;
    type UpdatePayload: DeserializeOwned + Debug;

    fn new(config: Box<dyn ConfigManagerDyn<Config>>) -> Self;
    fn get_state(&self) -> Self::StatePayload;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>);
    fn get_latest_block_hash(&self) -> String;
}
