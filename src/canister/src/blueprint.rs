use crate::ethereum::{config::EthereumConfigManager, GenericChainBlueprint};
use ic_lightclient_ethereum::{helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

#[derive(Debug)]
pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    const CHAIN_UID: u16 = 1;
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec>;
}
