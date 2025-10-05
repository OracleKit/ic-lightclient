use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static PARAMETERS: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub struct ParameterManager;

impl ParameterManager {
    pub fn get(chain: &str) -> Option<String> {
        PARAMETERS.with_borrow(|map| {
            map.get(chain).map(|v| v.to_string())
        })
    }

    pub fn set(chain: String, param: String) {
        PARAMETERS.with_borrow_mut(|map| {
            map.insert(chain, param);
        });
    }

    pub fn list() -> Vec<String> {
        PARAMETERS.with_borrow(|map| {
            map.keys().map(|k| k.to_string()).collect()
        })
    }
}
