use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Block {
    pub block_num: u128,
    pub block_hash: String,
    pub base_gas_fee: u128,
    pub max_priority_fee: u128,
}
