mod chain;
mod config;
mod ethereum;
mod metrics;
mod state;

pub use crate::chain::ChainInterface;
use ic_lightclient_types::{CanisterState, CanisterUpdates};
use metrics::{serve_metrics, HttpRequest, HttpResponse};
use state::GlobalState;

#[ic_cdk::query]
fn get_latest_block_hash() -> String {
    let chains = GlobalState::chains();
    let ethereum = &chains.borrow().ethereum;
    ethereum.get_latest_block_hash()
}

#[ic_cdk::query]
fn get_state() -> CanisterState {
    let chains = GlobalState::chains();
    let ethereum_state = chains.borrow().ethereum.get_state();

    CanisterState {
        version: 1,
        ethereum: ethereum_state,
    }
}

#[ic_cdk::update]
fn update_state(updates: CanisterUpdates) {
    let start = ic_cdk::api::performance_counter(0);

    let chains = GlobalState::chains();
    chains.borrow_mut().ethereum.update_state(updates.ethereum);

    let end = ic_cdk::api::performance_counter(0);
    let cycles = ic_cdk::api::canister_balance();
    ic_cdk::println!("Instructions: {}, cycles: {}", end - start, cycles);
}

#[ic_cdk::query]
fn http_request(_: HttpRequest) -> HttpResponse {
    serve_metrics()
}

ic_cdk::export_candid!();
