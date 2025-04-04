mod chain;
mod ethereum;
mod state;

use ic_lightclient_types::{CanisterState, CanisterUpdates};
pub use crate::chain::ChainInterface;

#[ic_cdk::query]
fn get_state() -> CanisterState {
    CanisterState::default()
}

#[ic_cdk::update]
fn update_state(updates: CanisterUpdates) {
}

ic_cdk::export_candid!();