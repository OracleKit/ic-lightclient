use candid::Principal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IcpConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}
