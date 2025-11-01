use anyhow::Result;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait StateMachine: Send {
    type Config: DeserializeOwned + 'static;
    type CanisterStatePayload: DeserializeOwned + 'static;
    type CanisterUpdatePayload: Serialize + 'static;

    fn new() -> Self;
    async fn init(&mut self, config: Self::Config) -> Result<()>;
    async fn get_updates(
        &mut self,
        canister_state: Self::CanisterStatePayload,
    ) -> Result<Vec<Self::CanisterUpdatePayload>>;
}
