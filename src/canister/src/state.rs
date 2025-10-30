use crate::{blueprint::build_chain_from_uid, chain::Chain};
use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    rc::Rc,
};

thread_local! {
    static CHAINS: OnceCell<Rc<RefCell<ChainState>>> = OnceCell::new();
}

pub struct ChainState {
    pub chains: HashMap<u16, Box<dyn Chain>>,
}

pub struct GlobalState;

impl GlobalState {
    pub async fn init(uids: Vec<u16>) {
        let mut chains = HashMap::new();

        for uid in uids {
            let mut chain = build_chain_from_uid(uid).await;
            chain.init().await;

            chains.insert(uid, chain);
        }

        let chains = ChainState { chains };
        let chains = Rc::new(RefCell::new(chains));

        CHAINS.with(|state| {
            state.set(chains).unwrap_or_else(|_| panic!("GlobalState already initialized"));
        });
    }

    pub fn state() -> Rc<RefCell<ChainState>> {
        CHAINS.with(|chains| chains.get().unwrap().clone())
    }

    pub fn chain_uids() -> Vec<u16> {
        CHAINS.with(|state| {
            let state = state.get().unwrap().borrow();
            state.chains.keys().map(|k| { k.clone() }).collect()
        })
    }
}
