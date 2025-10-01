mod config;

use ic_lightclient_ethereum::{
    config::EthereumConfig,
    helios::{
        consensus::{apply_bootstrap, verify_bootstrap},
        spec::MainnetConsensusSpec,
        types::LightClientStore,
    },
    parameters::EthereumParameters,
    payload::{
        apply_update_payload, LightClientStateActive, LightClientStateBootstrap, LightClientStatePayload,
        LightClientUpdatePayload,
    },
};
use ic_lightclient_types::{ChainState, ChainUpdates};

use crate::ethereum::config::EthereumConfigManager;

#[derive(Debug)]
pub struct EthereumChain {
    is_bootstrapped: bool,
    store: LightClientStore<MainnetConsensusSpec>,
    config: EthereumConfig,
    parameters: EthereumParameters,
}

impl EthereumChain {
    pub async fn new(parameters: EthereumParameters) -> Self {
        Self {
            is_bootstrapped: false,
            store: LightClientStore::<MainnetConsensusSpec>::default(),
            config: EthereumConfigManager::new(&parameters).await,
            parameters,
        }
    }

    pub fn get_state(&self) -> ChainState {
        let state = if !self.is_bootstrapped {
            let checkpoint_root = self.config.checkpoint_block_root.clone();
            let state = LightClientStateBootstrap { block_hash: checkpoint_root };
            let state = serde_json::to_vec(&LightClientStatePayload::<MainnetConsensusSpec>::Bootstrap(state))
                .expect("Failed to serialize state");

            state
        } else {
            let state = LightClientStateActive { store: self.store.clone() };
            let state = serde_json::to_vec(&LightClientStatePayload::<MainnetConsensusSpec>::Active(state))
                .expect("Failed to serialize state");

            state
        };

        ChainState { version: 1, state, tasks: vec![] }
    }

    // pub fn are_updates_valid(&self, _: ChainUpdates) -> bool {
    //     // Implement Ethereum-specific logic to validate updates
    //     true
    // }

    pub fn update_state(&mut self, updates: ChainUpdates) {
        let updates = updates.updates;

        // TODO: Add timer checks

        let updates: Vec<LightClientUpdatePayload<MainnetConsensusSpec>> = updates
            .into_iter()
            .map(|update| {
                let update: LightClientUpdatePayload<MainnetConsensusSpec> =
                    serde_json::from_slice(&update).expect("Failed to parse update");
                update
            })
            .collect();

        // TODO: Add check for conflicts

        for update in updates {
            match update {
                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    if self.is_bootstrapped {
                        panic!("Received bootstrap update after being bootstrapped");
                    }

                    let checkpoint_root = self.config.checkpoint_block_root.clone();
                    let forks = &self.parameters.forks;

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
            self.config.checkpoint_block_root.to_string()
        } else {
            format!(
                "Slot: {}, hash: {}",
                self.store.optimistic_header.beacon.slot,
                self.store.optimistic_header.beacon.state_root.to_string()
            )
        }
    }
}
