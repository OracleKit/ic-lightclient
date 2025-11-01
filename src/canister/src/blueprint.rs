use crate::{
    chain::{Chain, GenericChainBlueprint, GenericChainFactory},
    ethereum::{EthereumConfigManager, EthereumStateManager},
};
use anyhow::{anyhow, Result};
use ic_lightclient_ethereum::helios::spec::MainnetConsensusSpec;
use ic_lightclient_wire::EthereumWireProtocol;

pub struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    const CHAIN_UID: u16 = 1;
    type ConfigManager = EthereumConfigManager;
    type StateManager = EthereumStateManager<MainnetConsensusSpec>;
    type Protocol = EthereumWireProtocol<MainnetConsensusSpec>;
}

pub async fn build_chain_from_uid(uid: u16) -> Result<Box<dyn Chain>> {
    match uid {
        EthereumChainBlueprint::CHAIN_UID => GenericChainFactory::build::<EthereumChainBlueprint>().await,
        _ => Err(anyhow!("Invalid uid")),
    }
}
