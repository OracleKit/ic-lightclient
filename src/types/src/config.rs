use candid::Principal;
pub use ic_lightclient_ethereum::config::EthereumConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IcpConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}