use alloy_primitives::B256;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EthereumCheckpoint {
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

pub fn parse_checkpointz_output_to_config(data: Vec<u8>) -> Result<EthereumCheckpoint> {
    let data: RawSlotResponse = serde_json::from_slice(data.as_slice())?;
    let data = data.data.slots[0]
        .block_root
        .ok_or(anyhow!("Block root doesn't exist in checkpointz data"))?;

    Ok(EthereumCheckpoint { checkpoint_block_root: data })
}
