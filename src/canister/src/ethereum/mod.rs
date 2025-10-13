pub mod config;

use crate::chain::Chain;
use async_trait::async_trait;
use ic_lightclient_types::traits::{self, ConfigManager, ConsensusManager};
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser};
use std::{fmt::Debug, marker::PhantomData};

pub trait GenericChainBlueprint: Debug {
    const CHAIN_UID: u16;
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

    fn get_state(&self, marshaller: &mut StatePayloadMarshaller) {
        let state = self.consensus.get_state();
        marshaller.state(Blueprint::CHAIN_UID, state).unwrap();
    }

    // pub fn are_updates_valid(&self, _: ChainUpdates) -> bool {
    //     // Implement Ethereum-specific logic to validate updates
    //     true
    // }

    fn update_state(&mut self, updates: &UpdatePayloadParser) {

        // TODO: Add timer checks
        // TODO: Add check for conflicts

        let updates = updates.updates(Blueprint::CHAIN_UID).unwrap();
        self.consensus.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.get_latest_block_hash()
    }
}
