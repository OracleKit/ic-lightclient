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
