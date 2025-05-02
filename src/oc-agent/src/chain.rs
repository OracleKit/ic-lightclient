use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::ethereum::EthereumChain;

pub trait Chain {
    type ConfigType;

    fn new() -> Self;
    async fn init(&mut self, config: Self::ConfigType);
    async fn get_updates(&mut self, state: ChainState) -> Option<ChainUpdates>;
}

pub struct ChainManager {
    pub ethereum: EthereumChain
}

impl ChainManager {
    pub fn new() -> Self {
        Self {
            ethereum: EthereumChain::new()
        }
    }
}