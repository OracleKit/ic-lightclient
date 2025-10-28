use async_trait::async_trait;
use ic_lightclient_types::traits::{self, ConfigManager};
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser, WireProtocol};
use std::{marker::PhantomData, rc::Rc};

use crate::chain::state::StateManager;

#[async_trait(?Send)]
pub trait Chain {
    async fn init(&mut self);
    fn get_state(&self, marshaller: &mut StatePayloadMarshaller);
    fn update_state(&mut self, updates: &UpdatePayloadParser);
    fn get_latest_block_hash(&self) -> String;
    fn get_base_gas_fee(&self) -> u128;
    fn get_max_priority_fee(&self) -> u128;
    fn get_config(&self) -> Vec<u8>;
}

pub trait GenericChainBlueprint {
    const CHAIN_UID: u16;
    type ConfigManager: traits::ConfigManager + 'static;
    type Protocol: WireProtocol;
    type StateManager: StateManager<
            Config = <Self::ConfigManager as ConfigManager>::Config,
            UpdatePayload = <Self::Protocol as WireProtocol>::UpdatePayload,
            StatePayload = <Self::Protocol as WireProtocol>::StatePayload,
        > + 'static;
}

pub struct GenericChain<Blueprint: GenericChainBlueprint> {
    state: Blueprint::StateManager,
    config: Rc<Blueprint::ConfigManager>,
    blueprint: PhantomData<Blueprint>,
}

impl<Blueprint: GenericChainBlueprint> GenericChain<Blueprint> {
    pub async fn new(config: String) -> Self {
        let config_manager = Blueprint::ConfigManager::new(config).await;
        let config_manager = Rc::new(config_manager);
        let state = Blueprint::StateManager::new(config_manager.clone());

        Self { state, config: config_manager, blueprint: PhantomData }
    }
}

#[async_trait(?Send)]
impl<Blueprint: GenericChainBlueprint> Chain for GenericChain<Blueprint> {
    async fn init(&mut self) {}

    fn get_state(&self, marshaller: &mut StatePayloadMarshaller) {
        let state = self.state.get_state();
        marshaller.state::<Blueprint::Protocol>(Blueprint::CHAIN_UID, state).unwrap();
    }

    // pub fn are_updates_valid(&self, _: ChainUpdates) -> bool {
    //     // Implement Ethereum-specific logic to validate updates
    //     true
    // }

    fn update_state(&mut self, updates: &UpdatePayloadParser) {
        // TODO: Add timer checks
        // TODO: Add check for conflicts

        let updates = updates.updates::<Blueprint::Protocol>(Blueprint::CHAIN_UID).unwrap();
        self.state.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.state.get_latest_block_hash()
    }

    fn get_base_gas_fee(&self) -> u128 {
        self.state.get_base_gas_fee()
    }

    fn get_max_priority_fee(&self) -> u128 {
        self.state.get_max_priority_fee()
    }

    fn get_config(&self) -> Vec<u8> {
        let config = self.config.get_config();
        serde_json::to_vec(config).unwrap()
    }
}
