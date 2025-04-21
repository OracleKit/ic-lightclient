use serde::Deserialize;
use alloy_primitives::B256;
use crate::helios::types::Forks;

#[derive(Deserialize, Debug)]
pub struct EthereumConfig {
    pub consensus_api: String,
    pub execution_api: String,
    pub checkpoint_block_root: B256,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
}