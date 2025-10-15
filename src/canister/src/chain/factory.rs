use crate::{
    chain::{Chain, GenericChain, GenericChainBlueprint},
    config::ConfigManager,
};

pub struct GenericChainFactory;

impl GenericChainFactory {
    pub async fn build<B: GenericChainBlueprint + 'static>() -> Box<dyn Chain> {
        let config = ConfigManager::get(B::CHAIN_UID).unwrap();
        let chain = GenericChain::<B>::new(config).await;
        Box::new(chain)
    }
}
