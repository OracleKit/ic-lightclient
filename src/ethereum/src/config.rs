use crate::helios::types::Forks;
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
