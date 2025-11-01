use crate::{blueprint::build_chain_from_uid, chain::Chain};
use anyhow::{anyhow, Result};
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
    pub async fn init(uids: Vec<u16>) -> Result<()> {
        let mut chains = HashMap::new();

        for uid in uids {
            let mut chain = build_chain_from_uid(uid).await?;
            chain.init().await;

            chains.insert(uid, chain);
        }

        let chains = ChainState { chains };
        let chains = Rc::new(RefCell::new(chains));

        CHAINS.with(|state| -> Result<()> {
            state.set(chains).map_err(|_| anyhow!("Global state already initialized"))?;
            Ok(())
        })?;

        Ok(())
    }

    pub fn state() -> Result<Rc<RefCell<ChainState>>> {
        CHAINS.with(|chains| {
            let chains = chains.get().ok_or(anyhow!("Global state not initialized"))?;
            Ok(chains.clone())
        })
    }

    pub fn chain_uids() -> Result<Vec<u16>> {
        CHAINS.with(|state| {
            let state = state.get().ok_or(anyhow!("Global state not initialized"))?;

            let state = state.borrow();
            let chains = &state.chains;
            let uids = chains.keys().map(|k| k.clone()).collect();

            Ok(uids)
        })
    }
}
