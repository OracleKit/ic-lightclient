mod blueprint;
mod chain;
mod cli;
mod config;
mod ethereum;
mod outcalls;
mod http;
mod util;

use crate::{blueprint::build_chain_from_uid, cli::Cli, config::Config};
use anyhow::{anyhow, Result};
use chain::ChainManager;
use ic_lightclient_oc_utils::IcpAgent;
use ic_lightclient_wire::{StatePayloadParser, UpdatePayloadMarshaller};
use std::{sync::Arc, time::Duration};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};

#[tokio::main]
async fn main() -> Result<()> {
    Cli::init()?;

    let config_file = Cli::config_file();
    Config::init(&config_file)?;

    IcpAgent::init(Config::icp()).await?;

    let mut chain_manager = ChainManager::new();
    let configured_chains = IcpAgent::list_chain_uids().await?;

    for uid in configured_chains {
        let config = IcpAgent::get_canister_config(uid).await?;
        let chain = build_chain_from_uid(uid);
        chain_manager.set(uid, chain.clone());

        let mut chain = chain.lock().await;
        chain.init(config).await?;
    }

    loop {
        let state = IcpAgent::get_canister_state().await?;

        let state = StatePayloadParser::new(state)?;
        let state = Arc::new(state);
        let updates = Arc::new(Mutex::new(UpdatePayloadMarshaller::new()));
        let uids = chain_manager.list();
        let mut join_set = JoinSet::new();

        for uid in uids {
            let chain = chain_manager.get(&uid).ok_or(anyhow!("Chain not found in ChainManager"))?;
            let state = state.clone();
            let updates = updates.clone();

            join_set.spawn(async move {
                let mut chain = chain.lock().await;
                let mut updates: tokio::sync::MutexGuard<'_, UpdatePayloadMarshaller> = updates.lock().await;
                chain.get_updates(&state, &mut updates).await.unwrap();
            });
        }

        join_set.join_all().await;

        let updates = updates.lock().await;
        if updates.has_updates() {
            IcpAgent::update_canister_state(updates.build().unwrap()).await?;
        }

        sleep(Duration::from_secs(1)).await;
    }
}
