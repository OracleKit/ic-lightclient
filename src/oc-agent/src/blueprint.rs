use ic_lightclient_ethereum::helios::spec::MainnetConsensusSpec;
use ic_lightclient_wire::EthereumWireProtocol;

use crate::{chain::GenericChainBlueprint, ethereum::EthereumChain};

pub struct EthereumMainnetBlueprint;

impl GenericChainBlueprint for EthereumMainnetBlueprint {
    const CHAIN_UID: u16 = 1;
    type WireProtocol = EthereumWireProtocol<MainnetConsensusSpec>;
    type StateMachine = EthereumChain;
}
