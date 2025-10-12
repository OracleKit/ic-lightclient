use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait ConfigManager {
    type Config: Debug + 'static;

    fn new(config: String) -> impl std::future::Future<Output = Self>;
    fn get_config(&self) -> &Self::Config;
}

pub trait ConfigManagerDyn {
    type Config: Debug + 'static;

    fn get_config(&self) -> &Self::Config;
}

impl<T: ConfigManager> ConfigManagerDyn for T {
    type Config = <Self as ConfigManager>::Config;

    fn get_config(&self) -> &Self::Config {
        self.get_config()
    }
}

pub trait ConsensusManager {
    type Config: Debug;
    type StatePayload: Serialize + Debug;
    type UpdatePayload: DeserializeOwned + Debug;

    fn new(config: Box<dyn ConfigManagerDyn<Config = Self::Config>>) -> Self;
    fn get_state(&self) -> Self::StatePayload;
    fn update_state(&mut self, updates: Vec<Self::UpdatePayload>);
    fn get_latest_block_hash(&self) -> String;
}
