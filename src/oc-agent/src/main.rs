mod blueprint;
mod chain;
mod cli;
mod config;
mod ethereum;
mod http;
mod icp;
mod util;

use crate::{blueprint::build_chain_from_uid, cli::Cli, config::Config};
use anyhow::Result;
use chain::ChainManager;
use ic_lightclient_wire::{StatePayloadParser, UpdatePayloadMarshaller};
use icp::ICP;
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};

#[tokio::main]
async fn main() -> Result<()> {
    Cli::init()?;

    let config_file = Cli::config_file();
    Config::init(&config_file)?;

    ICP::init().await;

    let mut chain_manager = ChainManager::new();
    let configured_chains = ICP::list_chain_uids().await;

    for uid in configured_chains {
        let config = ICP::get_canister_config(uid).await;
        let chain = build_chain_from_uid(uid);
        chain_manager.set(uid, chain.clone());

        let mut chain = chain.lock().await;
        chain.init(config).await.unwrap();
    }

    loop {
        let state = ICP::get_canister_state().await;

        let state = Arc::new(StatePayloadParser::new(state).unwrap());
        let updates = Arc::new(Mutex::new(UpdatePayloadMarshaller::new()));
        let uids = chain_manager.list();
        let mut join_set = JoinSet::new();

        for uid in uids {
            let chain = chain_manager.get(&uid);
            let state = state.clone();
            let updates = updates.clone();

            join_set.spawn(async move {
                let mut chain = chain.lock().await;
                let mut updates = updates.lock().await;
                chain.get_updates(&state, &mut updates).await.unwrap();
            });
        }

        join_set.join_all().await;

        let updates = updates.lock().await;
        if updates.has_updates() {
            ICP::update_canister_state(updates.build().unwrap()).await;
        }

        sleep(Duration::from_secs(1)).await;
    }
}
