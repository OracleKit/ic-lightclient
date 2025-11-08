use anyhow::{anyhow, Result};
use ic_lightclient_utils::CircularQueue;
use ic_lightclient_wire::ethereum::outcalls::Block;

pub struct EthereumBlockStore {
    store: CircularQueue<Block>,
}

impl EthereumBlockStore {
    pub fn new(size: usize) -> Self {
        Self { store: CircularQueue::new(size) }
    }

    pub fn store(&mut self, block: Block) -> Result<()> {
        let tail = self.store.tail();
        if let Some(tail) = tail {
            if tail.block_num + 1 != block.block_num {
                return Err(anyhow!("Received non-consecutive blocks to store."));
            }
        }

        self.store.queue(block);
        Ok(())
    }

    pub fn get_updates(&mut self, block_num: u128) -> Vec<Block> {
        let Some(head) = self.store.head() else {
            return vec![];
        };
        let size = self.store.size() as u128;

        let block_num = if block_num < head.block_num { head.block_num } else { block_num };

        let mut offset = block_num - head.block_num;
        if offset >= size {
            return vec![];
        };

        let mut updates = vec![];
        while offset < size {
            let Some(block) = self.store.at_index(offset as usize) else {
                return vec![];
            };
            updates.push(block.clone());
            offset += 1;
        }

        updates
    }

    pub fn last_block_num(&self) -> Option<u128> {
        match self.store.tail() {
            Some(tail) => Some(tail.block_num),
            None => None,
        }
    }
}
