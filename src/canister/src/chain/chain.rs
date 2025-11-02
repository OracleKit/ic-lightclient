use crate::chain::{state::StateManager, ConfigManager};
use anyhow::Result;
use async_trait::async_trait;
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser, WireProtocol};
use std::marker::PhantomData;

#[async_trait(?Send)]
pub trait Chain {
    async fn init(&mut self);
    fn get_state(&self, marshaller: &mut StatePayloadMarshaller) -> Result<()>;
    fn update_state(&mut self, updates: &UpdatePayloadParser) -> Result<()>;
    fn get_latest_block_hash(&self) -> String;
    fn get_base_gas_fee(&self) -> u128;
    fn get_max_priority_fee(&self) -> u128;
    fn get_config(&self) -> Result<Vec<u8>>;
}

pub trait GenericChainBlueprint {
    const CHAIN_UID: u16;
    type ConfigManager: ConfigManager<
            Config = <Self::Protocol as WireProtocol>::Config
        > + 'static;
    type Protocol: WireProtocol;
    type StateManager: StateManager<
            Config = <Self::Protocol as WireProtocol>::Config,
            UpdatePayload = <Self::Protocol as WireProtocol>::UpdatePayload,
            StatePayload = <Self::Protocol as WireProtocol>::StatePayload,
        > + 'static;
}

type ExtractConfig<B> = <<B as GenericChainBlueprint>::Protocol as WireProtocol>::Config;

pub struct GenericChain<Blueprint: GenericChainBlueprint> {
    state: Blueprint::StateManager,
    config: ExtractConfig<Blueprint>,
    blueprint: PhantomData<Blueprint>,
}

impl<Blueprint: GenericChainBlueprint> GenericChain<Blueprint> {
    pub async fn new(config: String) -> Result<Self> {
        let config = Blueprint::ConfigManager::process(config).await?;
        let state = Blueprint::StateManager::new(config.clone());

        Ok(Self { state, config, blueprint: PhantomData })
    }
}

#[async_trait(?Send)]
impl<Blueprint: GenericChainBlueprint> Chain for GenericChain<Blueprint> {
    async fn init(&mut self) {}

    fn get_state(&self, marshaller: &mut StatePayloadMarshaller) -> Result<()> {
        let state = self.state.get_state()?;
        marshaller.state::<Blueprint::Protocol>(Blueprint::CHAIN_UID, state)?;
        Ok(())
    }

    // pub fn are_updates_valid(&self, _: ChainUpdates) -> bool {
    //     // Implement Ethereum-specific logic to validate updates
    //     true
    // }

    fn update_state(&mut self, updates: &UpdatePayloadParser) -> Result<()> {
        // TODO: Add timer checks
        // TODO: Add check for conflicts

        let updates = updates.updates::<Blueprint::Protocol>(Blueprint::CHAIN_UID)?;
        self.state.update_state(updates)?;
        Ok(())
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

    fn get_config(&self) -> Result<Vec<u8>> {
        let serialized = serde_json::to_vec(&self.config)?;
        Ok(serialized)
    }
}
