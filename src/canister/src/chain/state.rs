use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait StateManager {
    type Config: Debug;
    type StatePayload: Serialize + Debug;
    type UpdatePayload: DeserializeOwned + Debug;

    fn new(config: Self::Config) -> Self;
    fn get_state(&self) -> Result<Self::StatePayload>;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>) -> Result<()>;
    fn get_latest_block_hash(&self) -> String;
    fn get_base_gas_fee(&self) -> u128;
    fn get_max_priority_fee(&self) -> u128;
}
