use std::fmt::Debug;
use async_trait::async_trait;
use ic_lightclient_types::{ChainState, ChainUpdates};

#[async_trait]
pub trait Chain: Debug {
    async fn init(&mut self);
    fn get_state(&self) -> ChainState;
    fn update_state(&mut self, updates: ChainUpdates);
    fn get_latest_block_hash(&self) -> String;
}