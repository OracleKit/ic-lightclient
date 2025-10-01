use std::sync::Arc;

use crate::ethereum::EthereumChain;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ChainManager {
    pub ethereum: Arc<Mutex<EthereumChain>>,
}

impl ChainManager {
    pub fn new() -> Self {
        Self { ethereum: Arc::new(Mutex::new(EthereumChain::new())) }
    }
}
