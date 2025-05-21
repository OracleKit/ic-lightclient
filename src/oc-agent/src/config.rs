use ic_agent::export::Principal;
use ic_lightclient_ethereum::config::mainnet;
use ic_lightclient_types::{Config, ICPConfig};
use std::{env, str::FromStr};

// pub fn load_config() -> Config {
//     let config_file = "config.toml";
//     let config_file_contents = std::fs::read_to_string(config_file)
//         .expect("Failed to read config file");

//     let config: Config = toml::from_str(&config_file_contents.as_str())
//         .expect("Invalid config file");

//     config
// }

fn icp_config_local() -> ICPConfig {
    ICPConfig {
        canister_id: Principal::from_str("uxrrr-q7777-77774-qaaaq-cai").unwrap(),
        agent_url: "http://127.0.0.1:4943".into(),
    }
}

fn icp_config_prod() -> ICPConfig {
    ICPConfig {
        canister_id: Principal::from_str("mawej-zyaaa-aaaah-qqbqa-cai").unwrap(),
        agent_url: "https://icp-api.io".into(),
    }
}

pub fn load_config() -> Config {
    Config {
        ethereum: mainnet(),
        icp: if env::var("OKLC_PROD").is_err() {
            icp_config_local()
        } else {
            icp_config_prod()
        },
    }
}
