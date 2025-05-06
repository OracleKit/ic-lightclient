use std::rc::Rc;

use crate::{config::ConfigManager, ChainInterface};
use ic_lightclient_ethereum::{helios::{consensus::{apply_bootstrap, apply_finality_update, apply_optimistic_update, apply_update, verify_bootstrap}, spec::MainnetConsensusSpec, types::LightClientStore}, payload::{LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload, LightClientUpdatePayload}};
use ic_lightclient_types::{ChainState, ChainUpdates, Config};

pub struct EthereumChain {
    is_bootstrapped: bool,
    store: LightClientStore<MainnetConsensusSpec>,
    awaiting_challenge: Vec<LightClientUpdatePayload<MainnetConsensusSpec>>,
    config: Rc<Config>
}

impl Default for EthereumChain {
    fn default() -> Self {
        Self {
            is_bootstrapped: false,
            store: LightClientStore::<MainnetConsensusSpec>::default(),
            awaiting_challenge: vec![],
            config: ConfigManager::get()
        }
    }
}

impl ChainInterface for EthereumChain {
    fn get_state(&self) -> ChainState {
        let state = if !self.is_bootstrapped {
            let checkpoint_root = self.config.ethereum.checkpoint_block_root.clone();
            let state = LightClientStateBootstrap { block_hash: checkpoint_root };
            let state = serde_json::to_vec(&LightClientStatePayload::<MainnetConsensusSpec>::Bootstrap(state)).expect("Failed to serialize state");
            
            state
        } else {
            let optimistic_header = self.store.optimistic_header.clone();
            let finalized_header = self.store.finalized_header.clone();
            let has_next_sync_committee = self.store.next_sync_committee.is_some();
            let awaiting_challenge = self.awaiting_challenge.clone();
            let state = LightClientStateActive {
                optimistic_header,
                finalized_header,
                has_next_sync_committee,
                awaiting_challenge,
            };
            let state = serde_json::to_vec(&LightClientStatePayload::<MainnetConsensusSpec>::Active(state)).expect("Failed to serialize state");
            
            state
        };

        ChainState { version: 1, state, tasks: vec![] }
    }

    fn are_updates_valid(&self, updates: ChainUpdates) -> bool {
        // Implement Ethereum-specific logic to validate updates
        true
    }

    fn update_state(&mut self, updates: ChainUpdates) {
        let updates = updates.updates;
        let config = &self.config.ethereum;

        // TODO: Add timer checks

        let updates: Vec<LightClientUpdatePayload<MainnetConsensusSpec>> = updates.into_iter()
            .map(|update| {
                let update: LightClientUpdatePayload<MainnetConsensusSpec> = serde_json::from_slice(&update)
                    .expect("Failed to parse update");
                update
            })
            .collect();

        // TODO: Add check for conflicts

        for update in self.awaiting_challenge.iter() {
            match update {
                LightClientUpdatePayload::FinalityUpdate(update) => {
                    apply_finality_update(&mut self.store, &update.update);
                }
                LightClientUpdatePayload::OptimisticUpdate(update) => {
                    apply_optimistic_update(&mut self.store, &update.update);
                }
                LightClientUpdatePayload::Update(update) => {
                    apply_update(&mut self.store, &update.update);
                }
                _ => unreachable!()
            }
        }

        self.awaiting_challenge.clear();

        for update in updates {           
            match update {
                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    if self.is_bootstrapped {
                        panic!("Received bootstrap update after being bootstrapped");
                    }

                    let bootstrap = bootstrap.bootstrap;
                    let checkpoint_root = config.checkpoint_block_root.clone();
                    let forks = &config.forks;

                    verify_bootstrap(&bootstrap, checkpoint_root, forks).unwrap();
                    apply_bootstrap(&mut self.store, &bootstrap);
                    self.is_bootstrapped = true;
                }

                _ => {
                    self.awaiting_challenge.push(update);
                }
            }
        }
    }
}

impl EthereumChain {
    pub fn get_latest_block_hash(&self) -> String {
        if !self.is_bootstrapped {
            self.config.ethereum.checkpoint_block_root.to_string()
        } else {
            self.store.optimistic_header.beacon.state_root.to_string()
        }
    }
}