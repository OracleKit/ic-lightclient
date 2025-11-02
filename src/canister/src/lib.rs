mod blueprint;
mod chain;
mod config;
mod ethereum;
mod metrics;
mod outcalls;
mod state;

use crate::config::ConfigManager;
use ic_lightclient_wire::{StatePayloadMarshaller, UpdatePayloadParser};
use metrics::{serve_metrics, HttpRequest, HttpResponse};
use state::GlobalState;

#[ic_cdk::query]
fn get_latest_block_hash(chain: u16) -> String {
    let state = GlobalState::state().unwrap();
    let state = state.borrow();
    let chain = state.chains.get(&chain).unwrap();
    chain.get_latest_block_hash()
}

#[ic_cdk::query]
fn get_base_gas_fee(chain: u16) -> u128 {
    let state = GlobalState::state().unwrap();
    let state = state.borrow();
    let chain = state.chains.get(&chain).unwrap();
    chain.get_base_gas_fee()
}

#[ic_cdk::query]
fn get_max_priority_fee(chain: u16) -> u128 {
    let state = GlobalState::state().unwrap();
    let state = state.borrow();
    let chain = state.chains.get(&chain).unwrap();
    chain.get_max_priority_fee()
}

#[ic_cdk::query]
fn get_state() -> Vec<u8> {
    let state = GlobalState::state().unwrap();
    let state = state.borrow();
    let mut marshaller = StatePayloadMarshaller::new();

    for chain in state.chains.values() {
        chain.get_state(&mut marshaller).unwrap();
    }

    marshaller.build().unwrap()
}

#[ic_cdk::query]
fn list_chain_uids() -> Vec<u16> {
    GlobalState::chain_uids().unwrap()
}

#[ic_cdk::query]
fn get_chain_config(uid: u16) -> Vec<u8> {
    let state = GlobalState::state().unwrap();
    let state = state.borrow();
    let chain = state.chains.get(&uid).unwrap();
    chain.get_config().unwrap()
}

#[ic_cdk::update]
fn update_state(updates: Vec<u8>) {
    let start = ic_cdk::api::performance_counter(0);

    let parser = UpdatePayloadParser::new(updates).unwrap();
    let state = GlobalState::state().unwrap();
    let mut state = state.borrow_mut();

    for chain in state.chains.values_mut() {
        chain.update_state(&parser).unwrap();
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
    GlobalState::init(chains).await.unwrap();
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
