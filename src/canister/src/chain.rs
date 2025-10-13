use async_trait::async_trait;
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser};

#[async_trait(?Send)]
pub trait Chain {
    async fn init(&mut self);
    fn get_state(&self, marshaller: &mut StatePayloadMarshaller);
    fn update_state(&mut self, updates: &UpdatePayloadParser);
    fn get_latest_block_hash(&self) -> String;
}
