mod api;

use api::{ConsensusApi, ExecutionApi};
use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::{chain::Chain, config::EthereumConfig};

pub struct EthereumChain {}

impl Chain for EthereumChain {
    type ConfigType = EthereumConfig;

    async fn init(&self, config: EthereumConfig) {
        ExecutionApi::init(config.execution_api.clone());
        ConsensusApi::init(config.consensus_api.clone());
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