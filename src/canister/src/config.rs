use std::rc::Rc;
use ic_lightclient_types::Config;
use std::cell::LazyCell;
use std::include_str;

thread_local! {
    static CONFIG: LazyCell<Rc<Config>> = LazyCell::new(|| {
        let config = toml::de::from_str(include_str!("../../../config.toml"))
            .expect("Failed to parse config");

        Rc::new(config)
    });
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn get() -> Rc<Config> {
        CONFIG.with(|config| {
            let config = LazyCell::force(config);
            config.clone()
        })
    }
}