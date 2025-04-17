use std::sync::Arc;

use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::ethereum::EthereumChain;

pub trait Chain {
    async fn init(&self);
    async fn get_updates(&self, state: ChainState) -> Option<ChainUpdates>;
}

pub struct ChainManager {
    pub ethereum: Arc<EthereumChain>
}

impl ChainManager {
    pub fn new() -> Self {
        Self {
            ethereum: Arc::new(EthereumChain::new())
        }
    }
}