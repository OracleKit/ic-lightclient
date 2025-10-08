use ic_lightclient_ethereum::{config::EthereumConfigPopulated, helios::spec::MainnetConsensusSpec, EthereumLightClientConsensus};

use crate::{chain::Chain, config::ConfigManager, ethereum::{config::EthereumConfigManager, GenericChain, GenericChainBlueprint}};
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

#[derive(Debug)]
struct EthereumChainBlueprint;

impl GenericChainBlueprint for EthereumChainBlueprint {
    type Config = EthereumConfigPopulated;
    type ConfigManager = EthereumConfigManager;
    type ConsensusManager = EthereumLightClientConsensus<MainnetConsensusSpec, EthereumConfigManager>;
}
  
pub struct GlobalState;

impl GlobalState {
    pub async fn init() {
        let config = ConfigManager::get("ethereum").unwrap();
        let mut ethereum = GenericChain::<EthereumChainBlueprint>::new(config).await;
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
