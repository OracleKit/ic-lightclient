use ic_lightclient_ethereum::{config::EthereumConfigPopulated, helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};
use crate::ethereum::{config::EthereumConfigManager, GenericChainBlueprint};

#[derive(Debug)]
pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    type Config = EthereumConfigPopulated;
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec, EthereumConfigManager>;
}
