use serde::Deserialize;
use ic_lightclient_ethereum::config::EthereumConfig;
use ic_agent::export::Principal;

#[derive(Deserialize, Debug)]
pub struct ICPConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub ethereum: EthereumConfig,
    pub icp: ICPConfig,
}
