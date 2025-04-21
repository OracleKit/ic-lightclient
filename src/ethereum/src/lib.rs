pub mod helios;
pub mod payload;
pub mod config;

pub struct EthereumConsensus;

impl EthereumConsensus {
    // takes bootstrap and returns lightclientstore
    pub fn initialize_light_client_store() {}

    // takes lightclientstore mut & and update
    pub fn process_light_client_update() {}

    // later
    pub fn process_light_client_force_update() {}
}