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

    let chain_manager = ChainManager::new();
    let configured_chains = ICP::list_chain_uids().await;
    println!("Received chains for configuration: {:?}", &configured_chains);

    for uid in configured_chains {
        println!("Initializing chain: {}", uid);
        let config = ICP::get_canister_config(uid).await;
        let chain = build_chain_from_uid(uid);
        let mut chain = chain.lock().await;
        chain.init(config).await.unwrap();
        println!("Initialized chain: {}", uid);
    }

    loop {
        let state = ICP::get_canister_state().await;
        println!("Received canister state.");

        let state = Arc::new(StatePayloadParser::new(state).unwrap());
        let updates = Arc::new(Mutex::new(UpdatePayloadMarshaller::new()));
        let uids = chain_manager.list();
        let mut join_set = JoinSet::new();

        for uid in uids {
            println!("Beginning sync for chain: {}", uid);
            let chain = chain_manager.get(&uid);
            let state = state.clone();
            let updates = updates.clone();

            join_set.spawn(async move {
                println!("Spawned sync job for chain: {}", uid);
                let mut chain = chain.lock().await;
                let mut updates = updates.lock().await;
                println!("Obtained locks for sync job for chain: {}", uid);
                chain.get_updates(&state, &mut updates).await.unwrap();
                println!("Ran updates for chain: {}", uid);
            });
        }

        println!("Joining all spawned tasks.");
        join_set.join_all().await;
        println!("Done.");

        let updates = updates.lock().await;
        if updates.has_updates() {
            ICP::update_canister_state(updates.build().unwrap()).await;
        }

        sleep(Duration::from_secs(1)).await;
    }
}
