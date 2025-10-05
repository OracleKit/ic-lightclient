mod checkpoint;

use std::rc::Rc;
use crate::{chain::Chain, ethereum::checkpoint::EthereumCheckpointManager};
use async_trait::async_trait;
use ic_lightclient_ethereum::{
    config::EthereumConfig,
    helios::spec::MainnetConsensusSpec,
    payload::LightClientUpdatePayload,
    EthereumLightClientConsensus,
};
use ic_lightclient_types::{ChainState, ChainUpdates};

#[derive(Debug)]
pub struct EthereumChain {
    consensus: Option<EthereumLightClientConsensus<MainnetConsensusSpec>>,
    config: Rc<EthereumConfig>,
}

impl EthereumChain {
    pub fn new(config: String) -> Self {
        let config: EthereumConfig = serde_json::from_str(&config).unwrap();

        Self {
            consensus: None,
            config: Rc::new(config),
        }
    }
}

#[async_trait(?Send)]
impl Chain for EthereumChain {
    async fn init(&mut self) {
        let checkpoint = EthereumCheckpointManager::new(&self.config).await;
        self.consensus = Some(EthereumLightClientConsensus::new(checkpoint.checkpoint_block_root, self.config.clone()));
    }

    fn get_state(&self) -> ChainState {
        let state = self.consensus.as_ref().unwrap().get_state();
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

        self.consensus.as_mut().unwrap().update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.as_ref().unwrap().get_latest_block_hash()
    }
}
