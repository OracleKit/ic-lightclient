use std::rc::Rc;
use alloy_primitives::B256;
use crate::{config::EthereumConfig, helios::{consensus::{apply_bootstrap, verify_bootstrap}, spec::ConsensusSpec, types::LightClientStore}, payload::{apply_update_payload, LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload, LightClientUpdatePayload}};

#[derive(Debug)]
pub struct EthereumLightClientConsensus<S: ConsensusSpec> {
    is_bootstrapped: bool,
    store: LightClientStore<S>,
    checkpoint: B256, // block root
    config: Rc<EthereumConfig>
}

impl<S: ConsensusSpec> EthereumLightClientConsensus<S> {
    pub fn new(checkpoint: B256, config: Rc<EthereumConfig>) -> Self {
        Self {
            is_bootstrapped: false,
            store: LightClientStore::default(),
            checkpoint,
            config
        }
    }

    pub fn get_state(&self) -> LightClientStatePayload<S> {
        if !self.is_bootstrapped {
            let checkpoint_root = self.checkpoint.clone();
            let state = LightClientStateBootstrap { block_hash: checkpoint_root };
            LightClientStatePayload::Bootstrap(state)
        } else {
            let state = LightClientStateActive { store: self.store.clone() };
            LightClientStatePayload::Active(state)
        }
    }

    pub fn update_state(&mut self, updates: Vec<LightClientUpdatePayload<S>>) {
        for update in updates {
            match update {
                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    if self.is_bootstrapped {
                        panic!("Received bootstrap update after being bootstrapped");
                    }

                    let checkpoint_root = self.checkpoint.clone();
                    let forks = &self.config.forks;

                    verify_bootstrap(&bootstrap, checkpoint_root, forks).unwrap();
                    apply_bootstrap(&mut self.store, &bootstrap);
                    self.is_bootstrapped = true;
                }

                LightClientUpdatePayload::Update(update) => {
                    apply_update_payload(&mut self.store, update);
                }
            }
        }
    }
    
    pub fn get_latest_block_hash(&self) -> String {
        if !self.is_bootstrapped {
            self.checkpoint.to_string()
        } else {
            format!(
                "Slot: {}, hash: {}",
                self.store.optimistic_header.beacon.slot,
                self.store.optimistic_header.beacon.state_root.to_string()
            )
        }
    }
}