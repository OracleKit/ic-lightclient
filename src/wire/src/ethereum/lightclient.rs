use crate::protocol::WireProtocol;
use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated,
    helios::{spec::ConsensusSpec, types::Bootstrap},
    payload::{LightClientState, LightClientStoreDiff},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::marker::PhantomData;

pub use crate::ethereum::common::Block;

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
