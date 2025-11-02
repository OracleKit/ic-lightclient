use crate::{
    chain::{Chain, GenericChain, GenericChainBlueprint},
    ethereum::EthereumChain, outcalls::OutcallsChain,
};
use ic_lightclient_ethereum::helios::spec::MainnetConsensusSpec;
use ic_lightclient_wire::{ethereum, outcalls};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EthereumMainnetBlueprint;

impl GenericChainBlueprint for EthereumMainnetBlueprint {
    const CHAIN_UID: u16 = 1;
    type WireProtocol = ethereum::EthereumWireProtocol<MainnetConsensusSpec>;
    type StateMachine = EthereumChain;
}

pub struct EthereumHoleskyBlueprint;

impl GenericChainBlueprint for EthereumHoleskyBlueprint {
    const CHAIN_UID: u16 = 17000;
    type WireProtocol = outcalls::OutcallsWireProtocol;
    type StateMachine = OutcallsChain;
}

fn build_chain<B: GenericChainBlueprint + 'static>() -> Arc<Mutex<dyn Chain + Send>> {
    Arc::new(Mutex::new(GenericChain::<B>::new()))
}

pub fn build_chain_from_uid(uid: u16) -> Arc<Mutex<dyn Chain + Send>> {
    match uid {
        EthereumMainnetBlueprint::CHAIN_UID => build_chain::<EthereumMainnetBlueprint>(),
        EthereumHoleskyBlueprint::CHAIN_UID => build_chain::<EthereumHoleskyBlueprint>(),
        _ => panic!("invalid chain uid received"),
    }
}
