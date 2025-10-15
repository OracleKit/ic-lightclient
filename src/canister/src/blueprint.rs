use crate::{
    chain::{Chain, GenericChainBlueprint, GenericChainFactory},
    ethereum::EthereumConfigManager,
};
use ic_lightclient_ethereum::{helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    const CHAIN_UID: u16 = 1;
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec>;
}

pub async fn build_chain_from_uid(uid: u16) -> Box<dyn Chain> {
    match uid {
        EthereumChainBlueprint::CHAIN_UID => GenericChainFactory::build::<EthereumChainBlueprint>().await,
        _ => panic!("Invalid"),
    }
}
