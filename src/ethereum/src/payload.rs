use alloy_primitives::B256;
use alloy_rpc_types_eth::Header;
use serde::{Deserialize, Serialize};
use crate::helios::{spec::ConsensusSpec, types::{Bootstrap, LightClientHeader, Update}};

#[derive(Serialize, Deserialize)]
pub struct BootstrapPayload<S: ConsensusSpec> {
    execution_header: Header,
    bootstrap: Bootstrap<S>
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePayload<S: ConsensusSpec> {
    execution_header: Vec<Header>,
    update: Update<S>
}

#[derive(Serialize, Deserialize)]
pub struct LightClientStateBootstrap {
    block_hash: B256
}

#[derive(Serialize, Deserialize)]
pub struct LightClientStateActive<S: ConsensusSpec> {
    finalized_header: LightClientHeader,
    optimistic_header: LightClientHeader,
    awaiting_challenge: Vec<Update<S>>,
}

#[derive(Serialize, Deserialize)]
pub enum LightClientStatePayload<S: ConsensusSpec> {
    BOOTSTRAP(LightClientStateBootstrap),
    ACTIVE(LightClientStateActive<S>)
}
