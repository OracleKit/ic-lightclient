use anyhow::{anyhow, Context, Ok, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChainState {
    pub version: u64,
    pub state: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CanisterState {
    pub version: u64,
    pub states: HashMap<u16, ChainState>,
}

pub struct StatePayloadParser {
    state: CanisterState,
}

impl StatePayloadParser {
    pub fn new(data: Vec<u8>) -> Result<Self> {
        let state: CanisterState = serde_json::from_slice(data.as_slice()).context("Failed to parse State Payload")?;

        Ok(Self { state })
    }

    pub fn state<T: DeserializeOwned>(&self, uid: u16) -> Result<T> {
        let raw_state = self.state.states.get(&uid).ok_or(anyhow!("No state for chain uid: {}", uid))?;

        let state = serde_json::from_slice(raw_state.state.as_slice()).context("Failed to parse state.")?;

        Ok(state)
    }
}

pub struct StatePayloadMarshaller {
    state: CanisterState,
}

impl StatePayloadMarshaller {
    pub fn new() -> Self {
        Self { state: CanisterState { version: 1, states: HashMap::new() } }
    }

    pub fn state<T: Serialize>(&mut self, uid: u16, state: T) -> Result<()> {
        let marshalled_state = serde_json::to_vec(&state).context("Failed to marshal chain state")?;

        self.state
            .states
            .insert(uid, ChainState { version: 1, state: marshalled_state });

        Ok(())
    }

    pub fn build(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self.state).context("Failed to marshal canister state")
    }
}
