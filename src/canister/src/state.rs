use crate::{config::ConfigManager, ethereum::EthereumChain};
use std::{
    cell::{OnceCell, RefCell},
    rc::Rc,
};

thread_local! {
    static CHAINS: OnceCell<Rc<RefCell<ChainState>>> = OnceCell::new();
}

#[derive(Debug)]
pub struct ChainState {
    pub ethereum: EthereumChain,
}

pub struct GlobalState;

impl GlobalState {
    pub async fn init() {
        let mut ethereum = EthereumChain::new(ConfigManager::get("ethereum").unwrap());
        ethereum.init().await;

        CHAINS.with(|chains| {
            chains.set(Rc::new(RefCell::new(ChainState { ethereum }))).unwrap();
        });
    }

    pub fn chains() -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| chains.get().unwrap().clone())
    }
}
