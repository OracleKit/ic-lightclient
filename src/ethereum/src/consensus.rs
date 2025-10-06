use std::{fmt::Debug, rc::Rc};
use crate::{checkpoint::EthereumCheckpoint, config::EthereumConfig, helios::{consensus::{apply_bootstrap, verify_bootstrap}, spec::ConsensusSpec, types::LightClientStore}, payload::{apply_update_payload, LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload, LightClientUpdatePayload}};

pub trait EthereumLightClientConfigManager : Debug {
    fn get_config(&self) -> &EthereumConfig;
    fn get_checkpoint(&self) -> &EthereumCheckpoint;
}

#[derive(Debug)]
pub struct EthereumLightClientConsensus<S: ConsensusSpec, ConfigManager: EthereumLightClientConfigManager> {
    is_bootstrapped: bool,
    store: LightClientStore<S>,
    config: Rc<ConfigManager>
}

impl<S: ConsensusSpec, ConfigManager: EthereumLightClientConfigManager> EthereumLightClientConsensus<S, ConfigManager> {
    pub fn new(config: Rc<ConfigManager>) -> Self {
        Self {
            is_bootstrapped: false,
            store: LightClientStore::default(),
            config
        }
    }

    pub fn get_state(&self) -> LightClientStatePayload<S> {
        if !self.is_bootstrapped {
            let checkpoint_root = self.config.get_checkpoint().checkpoint_block_root;
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

                    let checkpoint_root = self.config.get_checkpoint().checkpoint_block_root;
                    let forks = &self.config.get_config().forks;

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
            self.config.get_checkpoint().checkpoint_block_root.to_string()
        } else {
            format!(
                "Slot: {}, hash: {}",
                self.store.optimistic_header.beacon.slot,
                self.store.optimistic_header.beacon.state_root.to_string()
            )
        }
    }
}