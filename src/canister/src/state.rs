use std::{cell::RefCell, rc::Rc};
use crate::ethereum::EthereumChain;

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