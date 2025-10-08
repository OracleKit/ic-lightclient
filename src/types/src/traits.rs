use std::{fmt::Debug, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

pub trait TConfigManager<Config> {
    fn new(config: String) -> impl std::future::Future<Output =  Self>;
    fn get_config(&self) -> &Config;
}

pub trait TConsensusManager<Config, ConfigManager: TConfigManager<Config>> {
    type StatePayload : Serialize + Debug;
    type UpdatePayload : DeserializeOwned + Debug;

    fn new(config: Rc<ConfigManager>) -> Self;
    fn get_state(&self) -> Self::StatePayload;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>);
    fn get_latest_block_hash(&self) -> String;
}