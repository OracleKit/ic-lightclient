use std::fs::read_to_string;

use ic_lightclient_oc_utils::IcpAgent;
use ic_lightclient_types::config::{EthereumConfig, IcpConfig};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    icp: IcpConfig,
    ethereum: EthereumConfig
}

#[tokio::main]
async fn main() {
    let config_file = "oraclekit.toml";
    let config = read_to_string(config_file).unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    IcpAgent::init(config.icp).await;
    IcpAgent::set_config("ethereum".into(), serde_json::to_string(&config.ethereum).unwrap()).await;
    
    println!("Set config successfully.");
}