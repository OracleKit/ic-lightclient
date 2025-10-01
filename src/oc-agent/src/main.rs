mod chain;
mod cli;
mod config;
mod ethereum;
mod http;
mod icp;
mod util;

use anyhow::Result;
use chain::ChainManager;
use ic_lightclient_types::CanisterUpdates;
use icp::ICP;
use std::time::Duration;
use tokio::time::sleep;

use crate::{cli::Cli, config::Config};

#[tokio::main]
async fn main() -> Result<()> {
    Cli::init()?;

    let config_file = Cli::config_file();
    Config::init(&config_file)?;

    ICP::init().await;
    let state = ICP::get_canister_state().await;

    let chain_manager = ChainManager::new();
    chain_manager.ethereum.try_lock().unwrap().init(state.ethereum).await;

    loop {
        let state = ICP::get_canister_state().await;
        let chain_manager = chain_manager.clone();

        tokio::spawn(async move {
            let Ok(mut ethereum) = chain_manager.ethereum.try_lock() else {
                return;
            };
            let updates = ethereum.get_updates(state.ethereum).await;

            if let Some(updates) = updates {
                ICP::update_canister_state(CanisterUpdates { version: 1, ethereum: updates }).await;
            }
        });

        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
