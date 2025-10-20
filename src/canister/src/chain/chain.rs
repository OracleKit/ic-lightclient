use async_trait::async_trait;
use ic_lightclient_types::traits::{self, ConfigManager};
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser};
use std::marker::PhantomData;

use crate::chain::state::StateManager;

#[async_trait(?Send)]
pub trait Chain {
    async fn init(&mut self);
    fn get_state(&self, marshaller: &mut StatePayloadMarshaller);
    fn update_state(&mut self, updates: &UpdatePayloadParser);
    fn get_latest_block_hash(&self) -> String;
}

pub trait GenericChainBlueprint {
    const CHAIN_UID: u16;
    type ConfigManager: traits::ConfigManager + 'static;
    type StateManager: StateManager<Config = <Self::ConfigManager as ConfigManager>::Config> + 'static;
}

pub struct GenericChain<Blueprint: GenericChainBlueprint> {
    state: Blueprint::StateManager,
    blueprint: PhantomData<Blueprint>,
}

impl<Blueprint: GenericChainBlueprint> GenericChain<Blueprint> {
    pub async fn new(config: String) -> Self {
        let config_manager = Blueprint::ConfigManager::new(config).await;
        let state = Blueprint::StateManager::new(Box::new(config_manager));

        Self { state, blueprint: PhantomData }
    }
}

#[async_trait(?Send)]
impl<Blueprint: GenericChainBlueprint> Chain for GenericChain<Blueprint> {
    async fn init(&mut self) {}

    fn get_state(&self, marshaller: &mut StatePayloadMarshaller) {
        let state = self.state.get_state();
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
        self.state.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.state.get_latest_block_hash()
    }
}
