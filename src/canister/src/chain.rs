use ic_lightclient_types::{ChainState, ChainUpdates};

pub trait ChainInterface {
    fn get_state(&self) -> ChainState;
    fn are_updates_valid(&self, updates: ChainUpdates) -> bool;
    fn update_state(&mut self, updates: ChainUpdates);
}
