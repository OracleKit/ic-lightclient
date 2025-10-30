use crate::{
    chain::{Chain, GenericChain, GenericChainBlueprint},
    ethereum::EthereumChain,
};
use ic_lightclient_ethereum::helios::spec::MainnetConsensusSpec;
use ic_lightclient_wire::EthereumWireProtocol;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct EthereumMainnetBlueprint;

impl GenericChainBlueprint for EthereumMainnetBlueprint {
    const CHAIN_UID: u16 = 1;
    type WireProtocol = EthereumWireProtocol<MainnetConsensusSpec>;
    type StateMachine = EthereumChain;
}

fn build_chain<B: GenericChainBlueprint + 'static>() -> Arc<Mutex<dyn Chain + Send>> {
    Arc::new(Mutex::new(GenericChain::<B>::new()))
}

pub fn build_chain_from_uid(uid: u16) -> Arc<Mutex<dyn Chain + Send>> {
    match uid {
        EthereumMainnetBlueprint::CHAIN_UID => build_chain::<EthereumMainnetBlueprint>(),
        _ => panic!("invalid chain uid received"),
    }
}
