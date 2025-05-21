use crate::ethereum::EthereumChain;
use std::{cell::RefCell, rc::Rc};

thread_local! {
    static CHAINS: Rc<RefCell<ChainState>> = Rc::default();
}

#[derive(Default)]
pub struct ChainState {
    pub ethereum: EthereumChain,
}

pub struct GlobalState;

impl GlobalState {
    pub fn chains() -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| chains.clone())
    }
}
