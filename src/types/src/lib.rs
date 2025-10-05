use candid::{CandidType, Deserialize};
use serde::Serialize;

pub mod config;

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct ChainState {
    pub version: u64,
    pub state: Vec<u8>,
    pub tasks: Vec<Vec<u8>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Default)]
pub struct CanisterState {
    pub version: u64,
    pub ethereum: ChainState,
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct ChainUpdates {
    pub version: u64,
    pub updates: Vec<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug, Default)]
pub struct CanisterUpdates {
    pub version: u64,
    pub ethereum: ChainUpdates,
}
