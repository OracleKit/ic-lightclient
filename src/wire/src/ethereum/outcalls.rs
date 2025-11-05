use crate::WireProtocol;
use serde::{Deserialize, Serialize};

pub use crate::ethereum::common::Block;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub execution_apis: Vec<String>,
}

pub struct OutcallsWireProtocol;

impl WireProtocol for OutcallsWireProtocol {
    type StatePayload = Block;
    type UpdatePayload = Block;
    type Config = Config;
}
