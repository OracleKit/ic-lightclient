use crate::{
    config::EthereumConfigPopulated, helios::{
        consensus::{apply_bootstrap, apply_generic_update, expected_current_slot, verify_bootstrap, verify_generic_update},
        spec::ConsensusSpec,
        types::{Bootstrap, GenericUpdate, LightClientStore},
    }, payload::{diff_store, patch_store, LightClientState, LightClientStoreDiff}
};
use alloy_primitives::B256;
use anyhow::{anyhow, Result};

#[derive(Default)]
pub struct EthereumLightClientConsensus<S: ConsensusSpec> {
    is_bootstrapped: bool,
    store: LightClientStore<S>,
    config: EthereumConfigPopulated,
}

impl<S: ConsensusSpec> EthereumLightClientConsensus<S> {
    pub fn new(config: EthereumConfigPopulated) -> Self {
        Self { is_bootstrapped: false, store: LightClientStore::default(), config }
    }

    pub fn get_state(&self) -> Result<LightClientState<S>> {
        if !self.is_bootstrapped {
            let checkpoint = self.config.checkpoint.checkpoint_block_root;
            Ok(LightClientState::Bootstrap(checkpoint))
        } else {
            let store = self.store.clone();
            Ok(LightClientState::Active(store))
        }
    }

    pub fn bootstrap(&mut self, bootstrap: &Bootstrap<S>) -> Result<()> {
        if self.is_bootstrapped {
            return Err(anyhow!("Received bootstrap update after being bootstrapped."));
        }

        let config = &self.config;
        let checkpoint = config.checkpoint.checkpoint_block_root;
        let forks = &config.forks;

        verify_bootstrap(bootstrap, checkpoint, forks)
            .map_err(|e| anyhow!(e))?;

        apply_bootstrap(&mut self.store, bootstrap);
        self.is_bootstrapped = true;
        
        Ok(())
    }

    pub fn update(&mut self, update: &GenericUpdate<S>, current_time: u64) -> Result<()> {
        let config = &self.config;
        let genesis_root = config.genesis_validator_root;
        let genesis_time = config.genesis_time;
        let forks = &config.forks;
        let current_slot = expected_current_slot(current_time, genesis_time);

        verify_generic_update(update, current_slot, &self.store, genesis_root, forks)
            .map_err(|e| anyhow!(e))?;

        apply_generic_update(&mut self.store, update);

        Ok(())
    }

    pub fn diff(&self, store: &LightClientStore<S>) -> Option<LightClientStoreDiff<S>> {
        diff_store(&self.store, store)
    }

    pub fn patch(&mut self, diff: LightClientStoreDiff<S>) {
        patch_store(&mut self.store, diff);
    }

    pub fn get_latest_block_hash(&self) -> String {
        if !self.is_bootstrapped {
            self.config
                .checkpoint
                .checkpoint_block_root
                .to_string()
        } else {
            format!(
                "Slot: {}, hash: {}",
                self.store.optimistic_header.beacon.slot,
                self.store.optimistic_header.beacon.state_root.to_string()
            )
        }
    }

    pub fn get_checkpoint_root(&self) -> B256 {
        self.config.checkpoint.checkpoint_block_root
    }

    pub fn get_optimistic_slot(&self) -> u64 {
        self.store.optimistic_header.beacon.slot
    }

    pub fn get_finalized_slot(&self) -> u64 {
        self.store.finalized_header.beacon.slot
    }

    pub fn is_next_sync_committee_known(&self) -> bool {
        self.store.next_sync_committee.is_some()
    }
}
