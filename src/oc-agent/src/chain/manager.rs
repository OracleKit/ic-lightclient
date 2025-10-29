use std::sync::Arc;

use crate::{blueprint::EthereumMainnetBlueprint, chain::chain::{Chain, GenericChain}};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ChainManager {
    pub ethereum: Arc<Mutex<dyn Chain + Send>>,
}

impl ChainManager {
    pub fn new() -> Self {
        Self { ethereum: Arc::new(Mutex::new(GenericChain::<EthereumMainnetBlueprint>::new())) }
    }
}
