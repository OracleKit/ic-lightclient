use crate::chain::chain::Chain;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ChainManager {
    pub chains: HashMap<u16, Arc<Mutex<dyn Chain + Send>>>,
}

impl ChainManager {
    pub fn new() -> Self {
        let chains = HashMap::new();
        Self { chains }
    }

    pub fn get(&self, uid: &u16) -> Arc<Mutex<dyn Chain + Send>> {
        self.chains.get(uid).unwrap().clone()
    }

    pub fn set(&mut self, uid: u16, chain: Arc<Mutex<dyn Chain + Send>>) {
        self.chains.insert(uid, chain);
    }

    pub fn list(&self) -> Vec<u16> {
        self.chains.keys().map(|k| k.clone()).collect()
    }
}
