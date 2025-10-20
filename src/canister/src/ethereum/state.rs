use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated,
    helios::spec::ConsensusSpec,
    payload::{LightClientStatePayload, LightClientUpdatePayload},
    EthereumLightClientConsensus,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::chain::StateManager;

pub struct EthereumStateManager<S: ConsensusSpec> {
    consensus: EthereumLightClientConsensus<S>,
}

impl<S: ConsensusSpec + Serialize + DeserializeOwned> StateManager for EthereumStateManager<S> {
    type Config = EthereumConfigPopulated;
    type StatePayload = LightClientStatePayload<S>;
    type UpdatePayload = LightClientUpdatePayload<S>;

    fn new(config: Box<dyn ic_lightclient_types::traits::ConfigManagerDyn<Config = Self::Config>>) -> Self {
        let consensus = EthereumLightClientConsensus::new(config);
        Self { consensus }
    }

    fn get_state(&self) -> Self::StatePayload {
        self.consensus.get_state()
    }

    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>) {
        self.consensus.update_state(updates);
    }

    fn get_latest_block_hash(&self) -> String {
        self.consensus.get_latest_block_hash()
    }
}
