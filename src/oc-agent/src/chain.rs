use std::sync::Arc;

use crate::ethereum::EthereumChain;
use ic_lightclient_types::{ChainState, ChainUpdates};
use tokio::sync::Mutex;

pub trait Chain {
    type ConfigType;

    fn new() -> Self;
    async fn init(&mut self, config: Self::ConfigType);
    async fn get_updates(&mut self, state: ChainState) -> Option<ChainUpdates>;
}

#[derive(Clone)]
pub struct ChainManager {
    pub ethereum: Arc<Mutex<EthereumChain>>,
}

impl ChainManager {
    pub fn new() -> Self {
        Self {
            ethereum: Arc::new(Mutex::new(EthereumChain::new())),
        }
    }
}
