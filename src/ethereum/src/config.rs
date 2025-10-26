use crate::{checkpoint::EthereumCheckpoint, helios::types::Forks};
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize, Clone)]
pub struct EthereumConfig {
    pub execution_api: String,
    pub consensus_api: String,
    pub checkpoint_sync_host: String,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
}

#[derive(Default, Debug, Clone)]
pub struct EthereumConfigPopulated {
    pub execution_api: String,
    pub consensus_api: String,
    pub checkpoint_sync_host: String,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
    pub checkpoint: EthereumCheckpoint,
}

impl EthereumConfig {
    pub fn populate(self, checkpoint: EthereumCheckpoint) -> EthereumConfigPopulated {
        EthereumConfigPopulated {
            execution_api: self.execution_api,
            consensus_api: self.consensus_api,
            checkpoint_sync_host: self.checkpoint_sync_host,
            genesis_validator_root: self.genesis_validator_root,
            genesis_time: self.genesis_time,
            forks: self.forks,
            checkpoint,
        }
    }
}
