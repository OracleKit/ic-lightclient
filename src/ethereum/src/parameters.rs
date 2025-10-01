use crate::helios::types::{Fork, Forks};
use alloy_primitives::{b256, fixed_bytes, B256};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct EthereumParameters {
    pub checkpoint_sync_url: String,
    pub genesis_validator_root: B256,
    pub genesis_time: u64,
    pub forks: Forks,
}

pub fn mainnet() -> EthereumParameters {
    EthereumParameters {
        checkpoint_sync_url: "https://sync-mainnet.beaconcha.in".into(),
        genesis_validator_root: b256!("4b363db94e286120d76eb905340fdd4e54bfe9f06bf33ff6cf5ad27f511bfe95"),
        genesis_time: 1606824023,
        forks: Forks {
            genesis: Fork { epoch: 0, fork_version: fixed_bytes!("00000000") },
            altair: Fork { epoch: 74240, fork_version: fixed_bytes!("01000000") },
            bellatrix: Fork { epoch: 144896, fork_version: fixed_bytes!("02000000") },
            capella: Fork { epoch: 194048, fork_version: fixed_bytes!("03000000") },
            deneb: Fork { epoch: 269568, fork_version: fixed_bytes!("04000000") },
            electra: Fork { epoch: 364032, fork_version: fixed_bytes!("05000000") },
        },
    }
}
