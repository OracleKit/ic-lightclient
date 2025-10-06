use std::{fmt::Debug, rc::Rc};
use serde::{de::DeserializeOwned, Serialize};
use crate::{checkpoint::EthereumCheckpoint, config::EthereumConfig, helios::{consensus::{apply_bootstrap, verify_bootstrap}, spec::ConsensusSpec, types::LightClientStore}, payload::{apply_update_payload, LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload, LightClientUpdatePayload}};

pub trait TConfigManager {
    fn new(config: String) -> Self;
    fn init(&mut self) -> impl std::future::Future<Output = ()>;
}

pub trait TEthereumLightClientConfigManager : Debug {
    fn get_config(&self) -> &EthereumConfig;
    fn get_checkpoint(&self) -> &EthereumCheckpoint;
}

pub trait TConsensusManager {
    type StatePayload : Serialize + Debug;
    type UpdatePayload : DeserializeOwned + Debug;
    type ConfigManager : TEthereumLightClientConfigManager;

    fn new(config: Rc<Self::ConfigManager>) -> Self;
    fn get_state(&self) -> Self::StatePayload;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>);
    fn get_latest_block_hash(&self) -> String;
}

#[derive(Debug)]
pub struct EthereumLightClientConsensus<S: ConsensusSpec, ConfigManager: TEthereumLightClientConfigManager> {
    is_bootstrapped: bool,
    store: LightClientStore<S>,
    config: Rc<ConfigManager>
}

impl<S: ConsensusSpec + Serialize + DeserializeOwned, ConfigManager: TEthereumLightClientConfigManager> TConsensusManager for EthereumLightClientConsensus<S, ConfigManager> {
    type StatePayload = LightClientStatePayload<S>;
    type UpdatePayload = LightClientUpdatePayload<S>;
    type ConfigManager = ConfigManager;

    fn new(config: Rc<ConfigManager>) -> Self {
        Self {
            is_bootstrapped: false,
            store: LightClientStore::default(),
            config
        }
    }

    fn get_state(&self) -> LightClientStatePayload<S> {
        if !self.is_bootstrapped {
            let checkpoint_root = self.config.get_checkpoint().checkpoint_block_root;
            let state = LightClientStateBootstrap { block_hash: checkpoint_root };
            LightClientStatePayload::<S>::Bootstrap(state)
        } else {
            let state = LightClientStateActive { store: self.store.clone() };
            LightClientStatePayload::<S>::Active(state)
        }
    }

    fn update_state(&mut self, updates: Vec<LightClientUpdatePayload<S>>) {
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
    
    fn get_latest_block_hash(&self) -> String {
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