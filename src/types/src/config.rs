use ic_lightclient_ethereum::config::EthereumConfig;
use ic_principal::Principal;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ICPConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}

impl Default for ICPConfig {
    fn default() -> Self {
        Self { canister_id: Principal::anonymous(), agent_url: "".to_string() }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub ethereum: EthereumConfig,
    pub icp: ICPConfig,
}
