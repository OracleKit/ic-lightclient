use std::{cell::RefCell, ops::Index, rc::Rc};
use crate::chain::Chain;

thread_local! {
    static CHAINS: RefCell<Vec<Rc<RefCell<Chain>>>> = RefCell::default();
}

pub struct GlobalState;

impl GlobalState {
    pub fn num_chains() -> usize {
        CHAINS.with(|chains| chains.borrow().len())
    }

    pub fn chain(index: usize) -> Rc<RefCell<Chain>> {
        CHAINS.with(|chains| {
            chains.borrow().index(index).clone()
        })
    }

    pub fn add_chain(chain: Chain) {
        CHAINS.with(|chains| {
            chains.borrow_mut().push(Rc::new(RefCell::new(chain)));
        });
    }
}