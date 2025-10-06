use ic_lightclient_ethereum::{consensus::{TConfigManager, TConsensusManager}, helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

use crate::{chain::Chain, config::ConfigManager, ethereum::{config::EthereumConfigManager, GenericChain}};
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

thread_local! {
    static CHAINS: OnceCell<Rc<RefCell<ChainState>>> = OnceCell::new();
}

#[derive(Debug)]
pub struct ChainState {
    pub ethereum: Box<dyn Chain>,
}

type EthereumChain = GenericChain<EthereumLightClientConsensus<MainnetConsensusSpec, EthereumConfigManager>>;

async fn generate_ethereum_chain(config: String) -> EthereumChain {
    let mut config = EthereumConfigManager::new(config);
    config.init().await;
    let config = Rc::new(config);

    let consensus = EthereumLightClientConsensus::new(config);

    EthereumChain::new(consensus)
}
  
pub struct GlobalState;

impl GlobalState {
    pub async fn init() {
        let mut ethereum = generate_ethereum_chain(ConfigManager::get("ethereum").unwrap()).await;
        ethereum.init().await;

        CHAINS.with(|chains| {
            chains
                .set(Rc::new(RefCell::new(ChainState { ethereum: Box::new(ethereum) })))
                .unwrap();
        });
    }

    pub fn chains() -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| chains.get().unwrap().clone())
    }
}
