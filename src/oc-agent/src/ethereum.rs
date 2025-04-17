use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::chain::Chain;

pub struct EthereumChain {}

impl Chain for EthereumChain {
    async fn init(&self) {
    }

    async fn get_updates(&self, state: ChainState) -> Option<ChainUpdates> {
        None
    }
}

impl EthereumChain {
    pub fn new() -> Self {
        Self {}
    }
}