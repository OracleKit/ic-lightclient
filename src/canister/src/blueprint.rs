use crate::{
    chain::{Chain, GenericChainBlueprint, GenericChainFactory},
    ethereum::{EthereumConfigManager, EthereumStateManager},
    outcalls::{OutcallsConfigManager, OutcallsStateManager},
};
use anyhow::{anyhow, Result};
use ic_lightclient_ethereum::helios::spec::MainnetConsensusSpec;
use ic_lightclient_wire::ethereum::{lightclient, outcalls};

struct EthereumMainnetBlueprint;

impl GenericChainBlueprint for EthereumMainnetBlueprint {
    const CHAIN_UID: u16 = 1;
    type ConfigManager = EthereumConfigManager;
    type StateManager = EthereumStateManager<MainnetConsensusSpec>;
    type Protocol = lightclient::EthereumWireProtocol<MainnetConsensusSpec>;
}

struct EthereumHoleskyBlueprint;

impl GenericChainBlueprint for EthereumHoleskyBlueprint {
    const CHAIN_UID: u16 = 17000;
    type ConfigManager = OutcallsConfigManager;
    type StateManager = OutcallsStateManager;
    type Protocol = outcalls::OutcallsWireProtocol;
}

pub async fn build_chain_from_uid(uid: u16) -> Result<Box<dyn Chain>> {
    match uid {
        EthereumMainnetBlueprint::CHAIN_UID => GenericChainFactory::build::<EthereumMainnetBlueprint>().await,
        EthereumHoleskyBlueprint::CHAIN_UID => GenericChainFactory::build::<EthereumHoleskyBlueprint>().await,
        _ => Err(anyhow!("Invalid uid")),
    }
}
