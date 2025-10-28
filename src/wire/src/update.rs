use anyhow::{anyhow, Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::WireProtocol;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChainUpdates {
    pub version: u64,
    pub updates: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CanisterUpdates {
    pub version: u64,
    pub updates: HashMap<u16, ChainUpdates>,
}

pub struct UpdatePayloadParser {
    updates: CanisterUpdates,
}

impl UpdatePayloadParser {
    pub fn new(data: Vec<u8>) -> Result<Self> {
        let updates: CanisterUpdates =
            serde_json::from_slice(data.as_slice()).context("Failed to parse Update Payload")?;

        Ok(Self { updates })
    }

    pub fn updates<W: WireProtocol>(&self, uid: u16) -> Result<Vec<W::UpdatePayload>> {
        let raw_updates = self
            .updates
            .updates
            .get(&uid)
            .ok_or(anyhow!("No updates for chain uid: {}", uid))?;

        let updates = raw_updates
            .updates
            .iter()
            .map(|raw_update|
                serde_json::from_slice(raw_update.as_slice())
                    .context("Failed to parse update."))
            .collect::<Result<Vec<W::UpdatePayload>>>()?;

        Ok(updates)
    }
}

pub struct UpdatePayloadMarshaller {
    updates: CanisterUpdates,
}

impl UpdatePayloadMarshaller {
    pub fn new() -> Self {
        Self { updates: CanisterUpdates { version: 1, updates: HashMap::new() } }
    }

    pub fn updates<W: WireProtocol>(&mut self, uid: u16, updates: Vec<W::UpdatePayload>) -> Result<()> {
        let marshalled_updates = updates
            .into_iter()
            .map(|update| serde_json::to_vec(&update).context("Failed to marshal chain update"))
            .collect::<Result<Vec<Vec<u8>>>>()?;

        self.updates
            .updates
            .insert(uid, ChainUpdates { version: 1, updates: marshalled_updates });

        Ok(())
    }

    pub fn build(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self.updates).context("Failed to marshal canister update")
    }
}
