use serde::{Serialize, Deserialize};
use candid::CandidType;

#[derive(CandidType, Serialize, Debug, Default)]
pub struct ChainState {
    pub version: u64,
    pub chain_id: u64,
    pub state: Vec<u8>,
    pub tasks: Vec<Vec<u8>>
}

#[derive(CandidType, Serialize, Debug, Default)]
pub struct CanisterState {
    pub version: u64,
    pub state: Vec<ChainState>
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct ChainUpdates {
    pub version: u64,
    pub chain_id: u64,
    pub updates: Vec<Vec<u8>>
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct CanisterUpdates {
    pub version: u64,
    pub updates: Vec<ChainUpdates>
}