mod config;

use std::rc::Rc;
use crate::{chain::Chain, ethereum::config::EthereumConfigManager};
use async_trait::async_trait;
use ic_lightclient_ethereum::{
    helios::spec::MainnetConsensusSpec,
    payload::LightClientUpdatePayload,
    EthereumLightClientConsensus,
};
use ic_lightclient_types::{ChainState, ChainUpdates};

#[derive(Debug)]
pub struct EthereumChain {
    consensus: EthereumLightClientConsensus<MainnetConsensusSpec, EthereumConfigManager>,
}

impl EthereumChain {
    pub async fn new(config: String) -> Self {
        let mut config = EthereumConfigManager::new(config);
        config.init().await;

        let config = Rc::new(config);

        Self {
            consensus: EthereumLightClientConsensus::new(config.clone())
        }
    }
}

#[async_trait(?Send)]
impl Chain for EthereumChain {
    async fn init(&mut self) {
    }

    fn get_state(&self) -> ChainState {
        let state = self.consensus.get_state();
        let state = serde_json::to_vec(&state).unwrap();

        ChainState { version: 1, state, tasks: vec![] }
    }

    // pub fn are_updates_valid(&self, _: ChainUpdates) -> bool {
    //     // Implement Ethereum-specific logic to validate updates
    //     true
    // }

    fn update_state(&mut self, updates: ChainUpdates) {
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

        self.consensus.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.get_latest_block_hash()
    }
}
