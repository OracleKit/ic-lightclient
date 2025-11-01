use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait ConfigManager {
    type Config: Clone + Debug + Serialize + DeserializeOwned + 'static;

    fn new(config: String) -> impl std::future::Future<Output = Result<Self>>
    where
        Self: Sized;
    fn get_config(&self) -> &Self::Config;
}
