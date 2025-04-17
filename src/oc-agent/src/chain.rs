use std::sync::Arc;

use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::ethereum::EthereumChain;

pub trait Chain {
    type ConfigType;

    async fn init(&self, config: Self::ConfigType);
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