use std::marker::PhantomData;
use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated, helios::{spec::ConsensusSpec, types::Bootstrap}, payload::{LightClientState, LightClientStoreDiff}
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::protocol::WireProtocol;

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

pub struct EthereumWireProtocol<S: ConsensusSpec> {
    _s: PhantomData<S>,
}

impl<S: ConsensusSpec + Serialize + DeserializeOwned> WireProtocol for EthereumWireProtocol<S> {
    type StatePayload = LightClientStatePayload<S>;
    type UpdatePayload = LightClientUpdatePayload<S>;
    type Config = EthereumConfigPopulated;
}
