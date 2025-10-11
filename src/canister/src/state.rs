use crate::{blueprint::EthereumChainBlueprint, chain::Chain, config::ConfigManager, ethereum::GenericChain};
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

thread_local! {
    static CHAINS: OnceCell<Rc<RefCell<ChainState>>> = OnceCell::new();
}

pub struct ChainState {
    pub ethereum: Box<dyn Chain>,
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
                .unwrap_or_else(|_| panic!("GlobalState already initialized"));
        });
    }

    pub fn chains() -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| chains.get().unwrap().clone())
    }
}
