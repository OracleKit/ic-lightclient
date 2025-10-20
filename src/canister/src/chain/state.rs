use ic_lightclient_types::traits::ConfigManagerDyn;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait StateManager {
    type Config: Debug;
    type StatePayload: Serialize + Debug;
    type UpdatePayload: DeserializeOwned + Debug;

    fn new(config: Box<dyn ConfigManagerDyn<Config = Self::Config>>) -> Self;
    fn get_state(&self) -> Self::StatePayload;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>);
    fn get_latest_block_hash(&self) -> String;
}
