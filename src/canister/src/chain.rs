use ic_lightclient_types::{ChainState, ChainUpdates};

pub trait Chain {
    fn get_state() -> ChainState;
    fn are_updates_valid(updates: ChainUpdates) -> bool;
    fn update_state(updates: ChainUpdates);
}