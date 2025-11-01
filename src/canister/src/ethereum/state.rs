use crate::chain::StateManager;
use anyhow::Result;
use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated, helios::spec::ConsensusSpec, EthereumLightClientConsensus,
};
use ic_lightclient_wire::{Block, LightClientStatePayload, LightClientUpdatePayload};
use serde::{de::DeserializeOwned, Serialize};

pub struct EthereumStateManager<S: ConsensusSpec> {
    consensus: EthereumLightClientConsensus<S>,
    block: Block,
}

impl<S: ConsensusSpec> EthereumStateManager<S> {
    fn update_block(&mut self, block: Block) {
        self.block = block;
    }
}

impl<S: ConsensusSpec + Serialize + DeserializeOwned> StateManager for EthereumStateManager<S> {
    type Config = EthereumConfigPopulated;
    type StatePayload = LightClientStatePayload<S>;
    type UpdatePayload = LightClientUpdatePayload<S>;

    fn new(config: Self::Config) -> Self {
        let consensus = EthereumLightClientConsensus::new(config);
        Self { consensus, block: Block::default() }
    }

    fn get_state(&self) -> Result<Self::StatePayload> {
        self.consensus.get_state()
    }

    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>) -> Result<()> {
        for update in updates {
            match update {
                LightClientUpdatePayload::Block(block) => {
                    self.update_block(block.clone());
                }

                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    self.consensus.bootstrap(&bootstrap)?;
                }

                LightClientUpdatePayload::Update(update) => {
                    self.consensus.patch(update);
                }
            }
        }

        Ok(())
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.get_latest_block_hash()
    }

    fn get_base_gas_fee(&self) -> u128 {
        self.block.base_gas_fee
    }

    fn get_max_priority_fee(&self) -> u128 {
        self.block.max_priority_fee
    }
}
