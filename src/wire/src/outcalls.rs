use serde::{Deserialize, Serialize};
use crate::WireProtocol;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Block {
    pub block_num: u128,
    pub block_hash: String,
    pub base_gas_fee: u128,
    pub max_priority_fee: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub execution_apis: Vec<String>
}

pub struct OutcallsWireProtocol;

impl WireProtocol for OutcallsWireProtocol {
    type StatePayload = Block;
    type UpdatePayload = Block;
    type Config = Config;
}