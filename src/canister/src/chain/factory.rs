use crate::{
    chain::{Chain, GenericChain, GenericChainBlueprint},
    config::ConfigManager,
};
use anyhow::{anyhow, Result};

pub struct GenericChainFactory;

impl GenericChainFactory {
    pub async fn build<B: GenericChainBlueprint + 'static>() -> Result<Box<dyn Chain>> {
        let config = ConfigManager::get(B::CHAIN_UID).ok_or(anyhow!("Chain config not found."))?;
        let chain = GenericChain::<B>::new(config).await?;
        Ok(Box::new(chain))
    }
}
