mod chain;
mod ethereum;
mod state;
mod config;

use ic_lightclient_types::{CanisterState, CanisterUpdates};
use state::GlobalState;
pub use crate::chain::ChainInterface;

#[ic_cdk::query]
fn get_state() -> CanisterState {
    let chains = GlobalState::chains();
    let ethereum_state = chains.borrow().ethereum.get_state();

    CanisterState {
        version: 1,
        ethereum: ethereum_state
    }
}

#[ic_cdk::update]
fn update_state(updates: CanisterUpdates) {
    let chains = GlobalState::chains();
    chains.borrow_mut().ethereum.update_state(updates.ethereum);
}

ic_cdk::export_candid!();