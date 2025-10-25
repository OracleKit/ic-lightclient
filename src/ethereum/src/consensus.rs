use crate::{
    config::EthereumConfigPopulated,
    helios::{
        consensus::{apply_bootstrap, verify_bootstrap},
        spec::ConsensusSpec,
        types::LightClientStore,
    },
    payload::{
        apply_update_payload, LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload,
        LightClientUpdatePayload,
    },
};
use ic_lightclient_types::traits::ConfigManagerDyn;
use serde::{de::DeserializeOwned, Serialize};

pub struct EthereumLightClientConsensus<S: ConsensusSpec> {
    is_bootstrapped: bool,
    store: LightClientStore<S>,
    config: Box<dyn ConfigManagerDyn<Config = EthereumConfigPopulated>>,
}

impl<S: ConsensusSpec + Serialize + DeserializeOwned> EthereumLightClientConsensus<S> {
    pub fn new(config: Box<dyn ConfigManagerDyn<Config = EthereumConfigPopulated>>) -> Self {
        Self { is_bootstrapped: false, store: LightClientStore::default(), config }
    }

    pub fn get_state(&self) -> LightClientStatePayload<S> {
        if !self.is_bootstrapped {
            let checkpoint_root = self.config.get_config().checkpoint.as_ref().unwrap().checkpoint_block_root;
            let state = LightClientStateBootstrap { block_hash: checkpoint_root };
            LightClientStatePayload::<S>::Bootstrap(state)
        } else {
            let state = LightClientStateActive { store: self.store.clone() };
            LightClientStatePayload::<S>::Active(state)
        }
    }

    pub fn update_state(&mut self, updates: Vec<LightClientUpdatePayload<S>>) {
        for update in updates {
            match update {
                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    if self.is_bootstrapped {
                        panic!("Received bootstrap update after being bootstrapped");
                    }

                    let checkpoint_root = self.config.get_config().checkpoint.as_ref().unwrap().checkpoint_block_root;
                    let forks = &self.config.get_config().forks;

                    verify_bootstrap(&bootstrap, checkpoint_root, forks).unwrap();
                    apply_bootstrap(&mut self.store, &bootstrap);
                    self.is_bootstrapped = true;
                }

                LightClientUpdatePayload::Update(update) => {
                    apply_update_payload(&mut self.store, update);
                }

                _ => {}
            }
        }
    }

    pub fn get_latest_block_hash(&self) -> String {
        if !self.is_bootstrapped {
            self.config
                .get_config()
                .checkpoint
                .as_ref()
                .unwrap()
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
}
