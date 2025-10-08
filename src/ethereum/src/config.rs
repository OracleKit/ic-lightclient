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

#[derive(Deserialize, Debug, Default, Serialize, Clone)]
pub struct EthereumConfigPopulated {
    pub execution_api: String,
    pub consensus_api: String,
    pub checkpoint_sync_host: String,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
    
    #[serde(skip)]
    pub checkpoint: Option<EthereumCheckpoint>,
}

impl From<EthereumConfig> for EthereumConfigPopulated {
    fn from(value: EthereumConfig) -> Self {
        Self {
            execution_api: value.execution_api,
            consensus_api: value.consensus_api,
            checkpoint_sync_host: value.checkpoint_sync_host,
            genesis_validator_root: value.genesis_validator_root,
            genesis_time: value.genesis_time,
            forks: value.forks,
            checkpoint: None
        }
    }
}