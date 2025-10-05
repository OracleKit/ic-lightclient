use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static CONFIG: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn get(chain: &str) -> Option<String> {
        CONFIG.with_borrow(|map| map.get(chain).map(|v| v.to_string()))
    }

    pub fn set(chain: String, config: String) {
        CONFIG.with_borrow_mut(|map: &mut HashMap<String, String>| {
            map.insert(chain, config);
        });
    }

    pub fn list() -> Vec<String> {
        CONFIG.with_borrow(|map| map.keys().map(|k| k.to_string()).collect())
    }
}
