use alloy_primitives::{b256, fixed_bytes, B256};
use serde::Deserialize;
use crate::helios::types::{Fork, Forks};

#[derive(Deserialize, Debug, Default)]
pub struct EthereumConfig {
    pub consensus_api: String,
    pub execution_api: String,
    pub checkpoint_block_root: B256,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
}

pub fn mainnet() -> EthereumConfig {
    EthereumConfig {
        consensus_api: "https://ethereum.operationsolarstorm.org".into(),
        execution_api: "https://ethereum-rpc.publicnode.com".into(),
        checkpoint_block_root: b256!("00203619f27722af905f1464b6959c2c96f1c8a4fd564e17b61819ba407c518e"),
        genesis_validator_root: b256!("4b363db94e286120d76eb905340fdd4e54bfe9f06bf33ff6cf5ad27f511bfe95"),
        genesis_time: 1606824023,
        forks: Forks {
            genesis: Fork {
                epoch: 0,
                fork_version: fixed_bytes!("00000000")
            },
            altair: Fork {
                epoch: 74240,
                fork_version: fixed_bytes!("01000000")
            },
            bellatrix: Fork {
                epoch: 144896,
                fork_version: fixed_bytes!("02000000")
            },
            capella: Fork {
                epoch: 194048,
                fork_version: fixed_bytes!("03000000")
            },
            deneb: Fork {
                epoch: 269568,
                fork_version: fixed_bytes!("04000000")
            },
            electra: Fork {
                epoch: 364032,
                fork_version: fixed_bytes!("05000000")
            }
        }
    }
}