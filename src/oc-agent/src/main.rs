mod icp;
mod chain;
mod ethereum;

use std::time::Duration;
use chain::{Chain, ChainManager};
use ic_lightclient_types::CanisterUpdates;
use tokio::time::sleep;
use icp::ICP;

#[tokio::main]
async fn main() {
    let chain_manager = ChainManager::new();
    chain_manager.ethereum.init().await;

    loop {
        let state = ICP::get_canister_state().await;
        let updates = chain_manager.ethereum.get_updates(state.ethereum).await;

        if let Some(updates) = updates {
            ICP::update_canister_state(
                CanisterUpdates {
                    version: 1,
                    ethereum: updates
                }
            ).await;
        }

        sleep(Duration::from_secs(1)).await;
    }
}