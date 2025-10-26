use crate::helios::{
    spec::ConsensusSpec,
    types::{GenericUpdate, LightClientHeader, LightClientStore, SyncCommittee},
};
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LightClientStoreDiff<S: ConsensusSpec> {
    pub optimistic_header: Option<LightClientHeader>,
    pub finalized_header: Option<LightClientHeader>,
    pub current_sync_committee: Option<SyncCommittee<S>>,
    pub next_sync_committee: Option<Option<SyncCommittee<S>>>,
    pub best_valid_update: Option<Option<GenericUpdate<S>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LightClientState<S: ConsensusSpec> {
    Bootstrap(B256),
    Active(LightClientStore<S>),
}

pub fn patch_store<S: ConsensusSpec>(store: &mut LightClientStore<S>, diff: LightClientStoreDiff<S>) {
    if let Some(current_sync_committee) = diff.current_sync_committee {
        store.current_sync_committee = current_sync_committee;
    }

    if let Some(next_sync_committee) = diff.next_sync_committee {
        store.next_sync_committee = next_sync_committee;
    }

    if let Some(optimistic_header) = diff.optimistic_header {
        store.optimistic_header = optimistic_header;
    }

    if let Some(finalized_header) = diff.finalized_header {
        store.finalized_header = finalized_header;
    }

    if let Some(best_valid_update) = diff.best_valid_update {
        store.best_valid_update = best_valid_update;
    }
}

pub fn diff_store<S: ConsensusSpec>(reference: &LightClientStore<S>, store: &LightClientStore<S>) -> Option<LightClientStoreDiff<S>> {
    let mut update = LightClientStoreDiff::default();
    let mut update_required = false;

    if store.optimistic_header != reference.optimistic_header {
        update.optimistic_header = Some(reference.optimistic_header.clone());
        update_required = true;
    }

    if store.finalized_header != reference.finalized_header {
        update.finalized_header = Some(reference.finalized_header.clone());
        update_required = true;
    }

    if store.best_valid_update != reference.best_valid_update {
        update.best_valid_update = Some(reference.best_valid_update.clone());
        update_required = true;
    }

    if store.current_sync_committee != reference.current_sync_committee {
        update.current_sync_committee = Some(reference.current_sync_committee.clone());
        update_required = true;
    }

    if store.next_sync_committee != reference.next_sync_committee {
        update.next_sync_committee = Some(reference.next_sync_committee.clone());
        update_required = true;
    }

    if update_required {
        Some(update)
    } else {
        None
    }
}