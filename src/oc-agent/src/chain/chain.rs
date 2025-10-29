use anyhow::Result;
use async_trait::async_trait;
use ic_lightclient_wire::{StatePayloadParser, UpdatePayloadMarshaller, WireProtocol};
use crate::chain::traits::StateMachine;

#[async_trait]
pub trait Chain {
    async fn init(&mut self, config: Vec<u8>) -> Result<()>;
    async fn get_updates(&mut self, state_parser: &StatePayloadParser, updates_marshaller: &mut UpdatePayloadMarshaller) -> Result<()>;
}

pub trait GenericChainBlueprint {
    const CHAIN_UID: u16;
    type WireProtocol: WireProtocol;
    type StateMachine: StateMachine<
        CanisterStatePayload = <Self::WireProtocol as WireProtocol>::StatePayload,
        CanisterUpdatePayload = <Self::WireProtocol as WireProtocol>::UpdatePayload,
    >;
}

pub struct GenericChain<Blueprint: GenericChainBlueprint> {
    state_machine: Blueprint::StateMachine,
}

impl<Blueprint: GenericChainBlueprint> GenericChain<Blueprint> {
    pub fn new() -> Self {
        Self {
            state_machine: Blueprint::StateMachine::new()
        }
    }
}

#[async_trait]
impl<Blueprint: GenericChainBlueprint> Chain for GenericChain<Blueprint> {
    async fn init(&mut self, config: Vec<u8>) -> Result<()> {
        let config = serde_json::from_slice(config.as_slice())?;
        self.state_machine.init(config).await?;
        Ok(())
    }

    async fn get_updates(&mut self, state_parser: &StatePayloadParser, updates_marshaller: &mut UpdatePayloadMarshaller) -> Result<()> {
        let state = state_parser.state::<Blueprint::WireProtocol>(Blueprint::CHAIN_UID)?;
        let updates = self.state_machine.get_updates(state).await;
        if updates.len() > 0 {
            updates_marshaller.updates::<Blueprint::WireProtocol>(Blueprint::CHAIN_UID, updates)?;
        }

        Ok(())
    }
}