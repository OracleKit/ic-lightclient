mod blueprint;
mod chain;
mod config;
mod ethereum;
mod metrics;
mod state;

use crate::config::ConfigManager;
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser};
use metrics::{serve_metrics, HttpRequest, HttpResponse};
use state::GlobalState;

#[ic_cdk::query]
fn get_latest_block_hash() -> String {
    let state = GlobalState::state();
    let state = state.borrow();
    let ethereum = state.chains.get(&1).unwrap();
    ethereum.get_latest_block_hash()
}

#[ic_cdk::query]
fn get_base_gas_fee() -> u128 {
    let state = GlobalState::state();
    let state = state.borrow();
    let ethereum = state.chains.get(&1).unwrap();
    ethereum.get_base_gas_fee()
}

#[ic_cdk::query]
fn get_max_priority_fee() -> u128 {
    let state = GlobalState::state();
    let state = state.borrow();
    let ethereum = state.chains.get(&1).unwrap();
    ethereum.get_max_priority_fee()
}

#[ic_cdk::query]
fn get_state() -> Vec<u8> {
    let state = GlobalState::state();
    let state = state.borrow();
    let mut marshaller = StatePayloadMarshaller::new();

    for chain in state.chains.values() {
        chain.get_state(&mut marshaller);
    }

    marshaller.build().unwrap()
}

#[ic_cdk::query]
fn get_chain_config() -> Vec<u8> {
    let state = GlobalState::state();
    let state = state.borrow();
    let ethereum = state.chains.get(&1).unwrap();
    ethereum.get_config()
}

#[ic_cdk::update]
fn update_state(updates: Vec<u8>) {
    let start = ic_cdk::api::performance_counter(0);

    let parser = UpdatePayloadParser::new(updates).unwrap();
    let state = GlobalState::state();
    let mut state = state.borrow_mut();

    for chain in state.chains.values_mut() {
        chain.update_state(&parser);
    }

    let end = ic_cdk::api::performance_counter(0);
    let cycles = ic_cdk::api::canister_balance();
    ic_cdk::println!("Instructions: {}, cycles: {}", end - start, cycles);
}

#[ic_cdk::query]
fn http_request(_: HttpRequest) -> HttpResponse {
    serve_metrics()
}

#[ic_cdk::update]
async fn init(chains: Vec<u16>) {
    GlobalState::init(chains).await;
}

#[ic_cdk::update]
fn set_config(chain: u16, config: String) {
    ConfigManager::set(chain, config);
}

#[ic_cdk::query]
fn list_configs() -> Vec<u16> {
    ConfigManager::list()
}

#[ic_cdk::query]
fn get_config(chain: u16) -> Option<String> {
    ConfigManager::get(chain)
}

ic_cdk::export_candid!();
