use anyhow::{anyhow, Context, Result};
use ic_agent::export::Principal;
use serde::Deserialize;
use std::{fs::read_to_string, sync::OnceLock};

static INNER: OnceLock<ConfigSchema> = OnceLock::new();

#[derive(Deserialize, Clone)]
pub struct ICPConfig {
    pub canister_id: Principal,
    pub agent_url: String,
}

#[derive(Deserialize, Clone)]
pub struct EthereumConfig {
    pub consensus_api: String,
    pub execution_api: String,    
}

#[derive(Deserialize)]
struct ConfigSchema {
    icp: ICPConfig,
    ethereum: EthereumConfig
}

pub struct Config {}

impl Config {
    pub fn init(file_path: &str) -> Result<()> {
        let config = read_to_string(file_path).context(format!("Config file not found: {}", file_path))?;
        let config: ConfigSchema = toml::from_str(&config).context("Error while parsing config file")?;
        INNER.set(config).map_err(|_| anyhow!("Attempting to reinitialize config."))?;

        Ok(())
    }

    pub fn icp() -> ICPConfig {
        INNER.get().unwrap().icp.clone()
    }

    pub fn ethereum() -> EthereumConfig {
        INNER.get().unwrap().ethereum.clone()
    }
}

// fn icp_config_local() -> ICPConfig {
//     ICPConfig {
//         canister_id: Principal::from_str("uxrrr-q7777-77774-qaaaq-cai").unwrap(),
//         agent_url: "http://127.0.0.1:4943".into(),
//     }
// }

// fn icp_config_prod() -> ICPConfig {
//     ICPConfig {
//         canister_id: Principal::from_str("mawej-zyaaa-aaaah-qqbqa-cai").unwrap(),
//         agent_url: "https://icp-api.io".into(),
//     }
// }

// pub fn load_config() -> Config {
//     Config {
//         ethereum: mainnet(),
//         icp: if env::var("OKLC_PROD").is_err() {
//             icp_config_local()
//         } else {
//             icp_config_prod()
//         },
//     }
// }
