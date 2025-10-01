use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct EthereumConfig {
    pub checkpoint_block_root: B256,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawSlotResponse {
    pub data: RawSlotResponseData,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RawSlotResponseData {
    pub slots: Vec<Slot>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Slot {
    pub block_root: Option<B256>,
}

pub fn parse_checkpointz_output_to_config(data: Vec<u8>) -> EthereumConfig {
    let data: RawSlotResponse = serde_json::from_slice(data.as_slice()).unwrap();

    EthereumConfig { checkpoint_block_root: data.data.slots[0].block_root.unwrap() }
}
