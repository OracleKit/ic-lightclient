use crate::chain::StateManager;
use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated,
    helios::spec::ConsensusSpec,
    EthereumLightClientConsensus,
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

    fn new(config: Box<dyn ic_lightclient_types::traits::ConfigManagerDyn<Config = Self::Config>>) -> Self {
        let config = config.get_config();
        let consensus = EthereumLightClientConsensus::new(config.clone());
        Self { consensus, block: Block::default() }
    }

    fn get_state(&self) -> Self::StatePayload {
        self.consensus.get_state().unwrap()
    }

    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>) {
        for update in updates {
            match update {
                LightClientUpdatePayload::Block(block) => {
                    self.update_block(block.clone());
                }

                LightClientUpdatePayload::Bootstrap(bootstrap) => {
                    self.consensus.bootstrap(&bootstrap).unwrap();
                }

                LightClientUpdatePayload::Update(update) => {
                    self.consensus.patch(update);
                }
            }
        }
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
