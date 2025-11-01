use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait ConfigManager {
    type Config: Debug + Serialize + DeserializeOwned + 'static;

    fn new(config: String) -> impl std::future::Future<Output = Result<Self>>
    where
        Self: Sized;
    fn get_config(&self) -> &Self::Config;
}

pub trait ConfigManagerDyn {
    type Config: Debug + Serialize + DeserializeOwned + 'static;

    fn get_config(&self) -> &Self::Config;
}

impl<T: ConfigManager> ConfigManagerDyn for T {
    type Config = <Self as ConfigManager>::Config;

    fn get_config(&self) -> &Self::Config {
        self.get_config()
    }
}
