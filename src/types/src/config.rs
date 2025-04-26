use serde::Deserialize;
use alloy_primitives::B256;
use ic_lightclient_ethereum::helios::types::Forks;
use ic_agent::export::Principal;

#[derive(Deserialize, Debug, Default)]
pub struct EthereumConfig {
    pub consensus_api: String,
    pub execution_api: String,
    pub checkpoint_block_root: B256,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
}

#[derive(Deserialize, Debug)]
pub struct ICPConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}

impl Default for ICPConfig {
    fn default() -> Self {
        Self {
            canister_id: Principal::anonymous(),
            agent_url: "".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub ethereum: EthereumConfig,
    pub icp: ICPConfig,
}
