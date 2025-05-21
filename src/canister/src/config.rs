use ic_lightclient_ethereum::config::mainnet;
use ic_lightclient_types::{Config, ICPConfig};
use std::cell::LazyCell;
use std::rc::Rc;

thread_local! {
    static CONFIG: LazyCell<Rc<Config>> = LazyCell::new(|| {
        let config = Config {
            ethereum: mainnet(),
            icp: ICPConfig::default()
        };

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
