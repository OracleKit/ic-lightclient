use serde::Deserialize;
use alloy_primitives::B256;

#[derive(Deserialize)]
pub struct EthereumConfig {
    pub consensus_rpc: String,
    pub execution_rpc: String,
    pub checkpoint_block_root: B256,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
}

#[derive(Deserialize)]
pub struct Config {
    pub ethereum: EthereumConfig
}

pub fn load_config() -> Config {
    let config_file = "config.toml";
    let config_file_contents = std::fs::read_to_string(config_file)
        .expect("Failed to read config file");

    let config: Config = toml::from_str(&config_file_contents.as_str())
        .expect("Invalid config file");

    config
}