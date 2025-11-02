use anyhow::Result;
use ic_lightclient_wire::outcalls::{Block, Config};
use crate::chain::StateManager;

pub struct OutcallsStateManager {
    _config: Config,
    state: Block
}

impl StateManager for OutcallsStateManager {
    type Config = Config;
    type StatePayload = Block;
    type UpdatePayload = Block;

    fn new(config: Config) -> Self {
        Self { _config: config, state: Block::default() }
    }

    fn get_state(&self) -> Result<Block> {
        Ok(self.state.clone())
    }

    fn update_state(&mut self, updates: Vec<Block>) -> Result<()> {
        let mut updates = updates;
        let block = updates.pop();

        if let Some(block) = block {
            self.state = block;
        }

        Ok(())
    }

    fn get_latest_block_hash(&self) -> String {
        self.state.block_hash.clone()
    }

    fn get_base_gas_fee(&self) -> u128 {
        self.state.base_gas_fee
    }

    fn get_max_priority_fee(&self) -> u128 {
        self.state.max_priority_fee
    }
}