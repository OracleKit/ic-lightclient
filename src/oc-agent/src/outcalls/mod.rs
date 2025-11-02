use std::time::SystemTime;
use anyhow::Result;
use async_trait::async_trait;
use ic_lightclient_wire::outcalls::{Block, Config};
use crate::{chain::StateMachine, util::ExecutionApi};

const BLOCK_TIME_SEC: u64 = 12;

#[derive(Default)]
pub struct OutcallsChain {
    execution_apis: Vec<ExecutionApi>,
    last_updated_time_sec: u64
}

#[async_trait]
impl StateMachine for OutcallsChain {
    type Config = Config;
    type CanisterStatePayload = Block;
    type CanisterUpdatePayload = Block;

    fn new() -> Self {
        Self::default()
    }

    async fn init(&mut self, config: Config) -> Result<()> {
        let execution_apis = config.execution_apis
            .into_iter()
            .map(|url| ExecutionApi::new(url))
            .collect();

        self.execution_apis = execution_apis;

        Ok(())
    }

    async fn get_updates(&mut self, canister_state: Block) -> Result<Vec<Block>> {
        let block = self.sync().await?;
        let Some(block) = block else { return Ok(vec![]) };

        let mut updates = vec![];
        if block.block_num > canister_state.block_num {
            updates.push(block);
        }

        Ok(updates)
    }
}

impl OutcallsChain {
    async fn sync(&mut self) -> Result<Option<Block>> {
        let current_time_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let current_time_sec: u64 = current_time_sec.as_secs().try_into()?;

        if current_time_sec - self.last_updated_time_sec < BLOCK_TIME_SEC {
            return Ok(None);
        }

        self.last_updated_time_sec = current_time_sec;

        let latest_block_num = self.execution_apis[0].latest_block_number().await?;
        let block_hash = self.execution_apis[0].block_header_by_number(latest_block_num).await?;
        let block_hash = block_hash.hash.to_string();
        let latest_block_num = latest_block_num.try_into()?;
        let base_fee = self.execution_apis[0].base_gas_fee().await?;
        let base_fee = base_fee.try_into()?;
        let max_priority_fee = self.execution_apis[0].max_priority_fee().await?;
        let max_priority_fee = max_priority_fee.try_into()?;

        let block = Block {
            block_num: latest_block_num,
            block_hash,
            base_gas_fee: base_fee,
            max_priority_fee
        };

        Ok(Some(block))
    }
}