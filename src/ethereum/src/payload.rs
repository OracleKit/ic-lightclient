use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use crate::helios::{spec::ConsensusSpec, types::{Bootstrap, GenericUpdate, LightClientHeader, LightClientStore, SyncCommittee}};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdatePayload<S: ConsensusSpec> {
    pub optimistic_header: Option<LightClientHeader>,
    pub finalized_header: Option<LightClientHeader>,
    pub current_sync_committee: Option<SyncCommittee<S>>,
    pub next_sync_committee: Option<Option<SyncCommittee<S>>>,
    pub best_valid_update: Option<Option<GenericUpdate<S>>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LightClientUpdatePayload<S: ConsensusSpec> {
    Bootstrap(Bootstrap<S>),
    Update(UpdatePayload<S>)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LightClientStateBootstrap {
    pub block_hash: B256
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LightClientStateActive<S: ConsensusSpec> {
    pub store: LightClientStore<S>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LightClientStatePayload<S: ConsensusSpec> {
    Bootstrap(LightClientStateBootstrap),
    Active(LightClientStateActive<S>)
}


pub fn apply_update_payload<S: ConsensusSpec>(store: &mut LightClientStore<S>, update: UpdatePayload<S>) {
    if let Some(current_sync_committee) = update.current_sync_committee {
        store.current_sync_committee = current_sync_committee;
    }

    if let Some(next_sync_committee) = update.next_sync_committee {
        store.next_sync_committee = next_sync_committee;
    }

    if let Some(optimistic_header) = update.optimistic_header {
        store.optimistic_header = optimistic_header;
    }

    if let Some(finalized_header) = update.finalized_header {
        store.finalized_header = finalized_header;
    }

    if let Some(best_valid_update) = update.best_valid_update {
        store.best_valid_update = best_valid_update;
    }
}