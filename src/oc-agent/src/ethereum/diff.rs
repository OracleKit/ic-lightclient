use ic_lightclient_ethereum::{
    helios::{
        spec::ConsensusSpec,
        types::{Bootstrap, LightClientStore},
    },
    EthereumLightClientConsensus,
};
use ic_lightclient_wire::{LightClientStatePayload, LightClientUpdatePayload};

pub struct EthereumStateDiff<S: ConsensusSpec> {
    bootstrap: Option<Bootstrap<S>>,
}

impl<S: ConsensusSpec> Default for EthereumStateDiff<S> {
    fn default() -> Self {
        Self { bootstrap: None }
    }
}

impl<S: ConsensusSpec> EthereumStateDiff<S> {
    pub fn add_bootstrap(&mut self, bootstrap: Bootstrap<S>) {
        self.bootstrap = Some(bootstrap);
    }

    pub fn get_diff_updates(
        &self,
        canister_state: &LightClientStatePayload<S>,
        store: &EthereumLightClientConsensus<S>,
    ) -> Vec<LightClientUpdatePayload<S>> {
        match &canister_state {
            LightClientStatePayload::Bootstrap(_state) => {
                // if root != bootstrap_update.header.beacon.tree_hash_root() {
                //     panic!("Bootstrap block hash mismatch, {:?}", bootstrap_update.header);
                // }

                println!("Received request for bootstrap!");
                return self.get_diff_updates_for_bootstrap(store);
            }

            LightClientStatePayload::Active(state) => {
                let slot = state.optimistic_header.beacon.slot;
                println!("Received request for slot: {}!", slot);

                return self.get_diff_updates_for_active(&state, &store);
            }
        }
    }

    fn get_diff_updates_for_bootstrap(
        &self,
        store: &EthereumLightClientConsensus<S>,
    ) -> Vec<LightClientUpdatePayload<S>> {
        let mut updates = vec![LightClientUpdatePayload::Bootstrap(
            self.bootstrap.as_ref().expect("Bootstrap update not found").clone(),
        )];

        if let Some(diff) = store.diff(&LightClientStore::default()) {
            updates.push(LightClientUpdatePayload::Update(diff));
        }

        updates
    }

    fn get_diff_updates_for_active(
        &self,
        canister_store: &LightClientStore<S>,
        store: &EthereumLightClientConsensus<S>,
    ) -> Vec<LightClientUpdatePayload<S>> {
        let mut updates = vec![];

        if let Some(diff) = store.diff(canister_store) {
            updates.push(LightClientUpdatePayload::Update(diff));
        }

        updates
    }
}
