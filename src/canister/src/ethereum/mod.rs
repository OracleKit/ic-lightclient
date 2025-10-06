pub mod config;

use std::{fmt::Debug, rc::Rc};
use crate::chain::Chain;
use async_trait::async_trait;
use ic_lightclient_ethereum::consensus::{TConsensusManager, TEthereumLightClientConfigManager};
use ic_lightclient_types::{ChainState, ChainUpdates};

#[derive(Debug)]
pub struct GenericChain<Consensus: TConsensusManager + Debug> {
    consensus: Consensus,
}

impl<Consensus: TConsensusManager + Debug> GenericChain<Consensus> {
    pub fn new(consensus: Consensus) -> Self {
        Self { consensus }
    }
}

#[async_trait(?Send)]
impl<Consensus: TConsensusManager + Debug> Chain for GenericChain<Consensus> {
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

        let updates: Vec<Consensus::UpdatePayload> = updates
            .into_iter()
            .map(|update| {
                let update: Consensus::UpdatePayload =
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
