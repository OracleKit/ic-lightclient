use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static CONFIG: RefCell<HashMap<u16, String>> = RefCell::new(HashMap::new());
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn get(chain: u16) -> Option<String> {
        CONFIG.with_borrow(|map| map.get(&chain).map(|v| v.to_string()))
    }

    pub fn set(chain: u16, config: String) {
        CONFIG.with_borrow_mut(|map: &mut HashMap<u16, String>| {
            map.insert(chain, config);
        });
    }

    pub fn list() -> Vec<u16> {
        CONFIG.with_borrow(|map| map.keys().map(|k| k.clone()).collect())
    }
}
