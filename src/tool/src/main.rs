use ic_lightclient_ethereum::config::EthereumConfig;
use ic_lightclient_oc_utils::{IcpAgent, IcpConfig};
use ic_lightclient_wire::ethereum::outcalls;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    icp: IcpConfig,
    ethereum: EthereumConfig,
    ethereum_holesky: outcalls::Config,
}

#[tokio::main]
async fn main() {
    let config_file = "oraclekit.toml";
    let config = read_to_string(config_file).unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    IcpAgent::init(config.icp).await.unwrap();
    IcpAgent::set_config(1, serde_json::to_string(&config.ethereum).unwrap())
        .await
        .unwrap();

    IcpAgent::set_config(17000, serde_json::to_string(&config.ethereum_holesky).unwrap())
        .await
        .unwrap();

    println!("Set config successfully.");
}
