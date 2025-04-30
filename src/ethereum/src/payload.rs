use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use crate::helios::{spec::ConsensusSpec, types::{Bootstrap, FinalityUpdate, LightClientHeader, OptimisticUpdate, Update}};

#[derive(Serialize, Deserialize, Clone)]
pub struct BootstrapPayload<S: ConsensusSpec> {
    pub bootstrap: Bootstrap<S>
}

impl<S> From<Bootstrap<S>> for BootstrapPayload<S>
where
    S: ConsensusSpec,
{
    fn from(bootstrap: Bootstrap<S>) -> Self {
        Self { bootstrap }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdatePayload<S: ConsensusSpec> {
    pub update: Update<S>
}

impl<S> From<Update<S>> for UpdatePayload<S>
where
    S: ConsensusSpec,
{
    fn from(update: Update<S>) -> Self {
        Self { update }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OptimisticUpdatePayload<S: ConsensusSpec> {
    pub update: OptimisticUpdate<S>
}

impl<S> From<OptimisticUpdate<S>> for OptimisticUpdatePayload<S>
where
    S: ConsensusSpec,
{
    fn from(update: OptimisticUpdate<S>) -> Self {
        Self { update }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FinalityUpdatePayload<S: ConsensusSpec> {
    pub update: FinalityUpdate<S>
}

impl<S> From<FinalityUpdate<S>> for FinalityUpdatePayload<S>
where
    S: ConsensusSpec,
{
    fn from(update: FinalityUpdate<S>) -> Self {
        Self { update }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LightClientUpdatePayload<S: ConsensusSpec> {
    Bootstrap(BootstrapPayload<S>),
    Update(UpdatePayload<S>),
    OptimisticUpdate(OptimisticUpdatePayload<S>),
    FinalityUpdate(FinalityUpdatePayload<S>)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LightClientStateBootstrap {
    pub block_hash: B256
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LightClientStateActive<S: ConsensusSpec> {
    pub finalized_header: LightClientHeader,
    pub optimistic_header: LightClientHeader,
    pub has_next_sync_committee: bool,
    pub awaiting_challenge: Vec<LightClientUpdatePayload<S>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LightClientStatePayload<S: ConsensusSpec> {
    Bootstrap(LightClientStateBootstrap),
    Active(LightClientStateActive<S>)
}
