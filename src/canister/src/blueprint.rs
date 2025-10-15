use crate::{chain::Chain, config::ConfigManager, ethereum::{config::EthereumConfigManager, GenericChain, GenericChainBlueprint}};
use ic_lightclient_ethereum::{helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

#[derive(Debug)]
pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    const CHAIN_UID: u16 = 1;
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec>;
}

pub async fn build_chain_from_uid(uid: u16) -> Box<dyn Chain> {
    let chain = match uid {
        EthereumChainBlueprint::CHAIN_UID => {
            let config = ConfigManager::get(EthereumChainBlueprint::CHAIN_UID).unwrap();
            GenericChain::<EthereumChainBlueprint>::new(config).await
        }
        _ => panic!("Invalid")
    };

    Box::new(chain)
}