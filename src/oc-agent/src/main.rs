mod chain;
mod cli;
mod config;
mod ethereum;
mod http;
mod icp;
mod util;

use anyhow::Result;
use chain::ChainManager;
use ic_lightclient_ethereum::{config::EthereumConfigPopulated, helios::spec::MainnetConsensusSpec};
use ic_lightclient_wire::{EthereumWireProtocol, StatePayloadParser, UpdatePayloadMarshaller};
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

    let config = ICP::get_canister_config().await;
    let config: EthereumConfigPopulated = serde_json::from_slice(config.as_slice()).unwrap();

    let chain_manager = ChainManager::new();
    chain_manager.ethereum.try_lock().unwrap().init(config).await;

    loop {
        let state = ICP::get_canister_state().await;
        let state = StatePayloadParser::new(state).unwrap();
        let chain_manager = chain_manager.clone();

        tokio::spawn(async move {
            let Ok(mut ethereum) = chain_manager.ethereum.try_lock() else {
                return;
            };
            let state = state.state::<EthereumWireProtocol<MainnetConsensusSpec>>(1).unwrap();
            let updates = ethereum.get_updates(state).await;

            if let Some(updates) = updates {
                let mut marshaller = UpdatePayloadMarshaller::new();
                marshaller.updates::<EthereumWireProtocol<MainnetConsensusSpec>>(1, updates).unwrap();

                ICP::update_canister_state(marshaller.build().unwrap()).await;
            }
        });

        sleep(Duration::from_secs(1)).await;
    }
}
