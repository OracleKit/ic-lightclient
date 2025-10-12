use crate::ethereum::{config::EthereumConfigManager, GenericChainBlueprint};
use ic_lightclient_ethereum::{helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

#[derive(Debug)]
pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec>;
}
