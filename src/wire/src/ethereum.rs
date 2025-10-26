use ic_lightclient_ethereum::{helios::{spec::ConsensusSpec, types::Bootstrap}, payload::{LightClientState, LightClientStoreDiff}};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Block {
    pub base_gas_fee: u128,
    pub max_priority_fee: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LightClientUpdatePayload<S: ConsensusSpec> {
    Bootstrap(Bootstrap<S>),
    Update(LightClientStoreDiff<S>),
    Block(Block),
}

pub type LightClientStatePayload<S> = LightClientState<S>;
