use anyhow::Result;
use ic_lightclient_types::traits::ConfigManagerDyn;
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, rc::Rc};

pub trait StateManager {
    type Config: Debug;
    type StatePayload: Serialize + Debug;
    type UpdatePayload: DeserializeOwned + Debug;

    fn new(config: Rc<dyn ConfigManagerDyn<Config = Self::Config>>) -> Self;
    fn get_state(&self) -> Result<Self::StatePayload>;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>) -> Result<()>;
    fn get_latest_block_hash(&self) -> String;
    fn get_base_gas_fee(&self) -> u128;
    fn get_max_priority_fee(&self) -> u128;
}
