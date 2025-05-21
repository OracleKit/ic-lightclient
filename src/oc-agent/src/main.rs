mod chain;
mod config;
mod ethereum;
mod http;
mod icp;
mod util;

use chain::{Chain, ChainManager};
use config::load_config;
use ic_lightclient_types::CanisterUpdates;
use icp::ICP;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let config = load_config();
    ICP::init(config.icp).await;

    let chain_manager = ChainManager::new();
    chain_manager
        .ethereum
        .try_lock()
        .unwrap()
        .init(config.ethereum)
        .await;

    loop {
        let state = ICP::get_canister_state().await;
        let chain_manager = chain_manager.clone();

        tokio::spawn(async move {
            let Ok(mut ethereum) = chain_manager.ethereum.try_lock() else {
                return;
            };
            let updates = ethereum.get_updates(state.ethereum).await;

            if let Some(updates) = updates {
                ICP::update_canister_state(CanisterUpdates {
                    version: 1,
                    ethereum: updates,
                })
                .await;
            }
        });

        sleep(Duration::from_secs(1)).await;
    }
}
