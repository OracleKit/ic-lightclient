use ic_lightclient_ethereum::{helios::{spec::ConsensusSpec, types::{Bootstrap, LightClientStore}}, payload::{LightClientStatePayload, LightClientUpdatePayload, UpdatePayload}};

pub struct EthereumStateDiff<S: ConsensusSpec> {
    bootstrap: Option<Bootstrap<S>>
}

impl<S: ConsensusSpec> Default for EthereumStateDiff<S> {
    fn default() -> Self {
        Self {
            bootstrap: None
        }
    }
}

impl<S: ConsensusSpec> EthereumStateDiff<S> {
    pub fn add_bootstrap(
        &mut self,
        bootstrap: Bootstrap<S>
    ) {
        self.bootstrap = Some(bootstrap);
    }

    pub fn get_diff_updates(
        &self,
        canister_state: &LightClientStatePayload<S>,
        store: &LightClientStore<S>
    ) -> Vec<LightClientUpdatePayload<S>> {
        match &canister_state {
            LightClientStatePayload::Bootstrap(_state) => {
                // if state.block_hash != bootstrap_update.header.beacon.tree_hash_root() {
                //     panic!("Bootstrap block hash mismatch, {:?}", bootstrap_update.header);
                // }

                println!("Received request for bootstrap!");
                return self.get_diff_updates_for_bootstrap(store);
            }

            LightClientStatePayload::Active(state) => {
                let slot = state.store.optimistic_header.beacon.slot;
                println!("Received request for slot: {}!", slot);

                return self.get_diff_updates_for_active(&state.store, &store);
            }
        }
    }

    fn get_diff_updates_for_bootstrap(
        &self,
        store: &LightClientStore<S>
    ) -> Vec<LightClientUpdatePayload<S>> {
        let mut updates = vec![];

        updates.push(LightClientUpdatePayload::Bootstrap(self.bootstrap.as_ref().expect("Bootstrap update not found").clone()));
        updates.push(
            LightClientUpdatePayload::Update(
                UpdatePayload {
                    optimistic_header: Some(store.optimistic_header.clone()),
                    finalized_header: Some(store.finalized_header.clone()),
                    current_sync_committee: Some(store.current_sync_committee.clone()),
                    next_sync_committee: Some(store.next_sync_committee.clone()),
                    best_valid_update: Some(store.best_valid_update.clone())
                }
            )
        );

        updates
    }

    fn get_diff_updates_for_active(
        &self,
        canister_store: &LightClientStore<S>,
        store: &LightClientStore<S>
    ) -> Vec<LightClientUpdatePayload<S>> {
        let mut update = UpdatePayload::default();
        
        if store.optimistic_header != canister_store.optimistic_header {
            update.optimistic_header = Some(store.optimistic_header.clone());
        }

        if store.finalized_header != canister_store.finalized_header {
            update.finalized_header = Some(store.finalized_header.clone());
        }

        if store.best_valid_update != canister_store.best_valid_update {
            update.best_valid_update = Some(store.best_valid_update.clone());
        }

        if store.current_sync_committee != canister_store.current_sync_committee {
            update.current_sync_committee = Some(store.current_sync_committee.clone());
        }

        if store.next_sync_committee != canister_store.next_sync_committee {
            update.next_sync_committee = Some(store.next_sync_committee.clone());
        }

        vec![LightClientUpdatePayload::Update(update)]
    }
}