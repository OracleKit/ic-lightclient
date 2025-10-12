pub mod config;

use crate::chain::Chain;
use async_trait::async_trait;
use ic_lightclient_types::{
    traits::{self, ConfigManager, ConsensusManager},
    ChainState, ChainUpdates,
};
use std::{fmt::Debug, marker::PhantomData};

pub trait GenericChainBlueprint: Debug {
    type ConfigManager: traits::ConfigManager + 'static;
    type ConsensusManager: traits::ConsensusManager<Config = <Self::ConfigManager as ConfigManager>::Config> + 'static;
}

pub struct GenericChain<Blueprint: GenericChainBlueprint> {
    consensus: Blueprint::ConsensusManager,
    blueprint: PhantomData<Blueprint>,
}

impl<Blueprint: GenericChainBlueprint> GenericChain<Blueprint> {
    pub async fn new(config: String) -> Self {
        let config_manager = Blueprint::ConfigManager::new(config).await;
        let consensus = Blueprint::ConsensusManager::new(Box::new(config_manager));

        Self { consensus, blueprint: PhantomData }
    }
}

#[async_trait(?Send)]
impl<Blueprint: GenericChainBlueprint> Chain for GenericChain<Blueprint> {
    async fn init(&mut self) {}

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

        let updates = updates
            .into_iter()
            .map(|update| serde_json::from_slice(&update).expect("Failed to parse update"))
            .collect();

        // TODO: Add check for conflicts

        self.consensus.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.get_latest_block_hash()
    }
}
