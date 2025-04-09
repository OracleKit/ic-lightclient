pub mod types;
pub mod spec;
pub mod proof;
pub mod utils;
pub mod consensus;
pub mod errors;

pub struct EthereumConsensus;

impl EthereumConsensus {
    // takes bootstrap and returns lightclientstore
    pub fn initialize_light_client_store() {}

    // takes lightclientstore mut & and update
    pub fn process_light_client_update() {}

    // later
    pub fn process_light_client_force_update() {}
}