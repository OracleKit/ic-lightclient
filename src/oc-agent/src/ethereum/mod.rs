mod api;
mod diff;

use crate::config::Config;
use alloy_primitives::B256;
use api::{ConsensusApi, ExecutionApi};
use diff::EthereumStateDiff;
use ic_lightclient_ethereum::{
    checkpoint::EthereumCheckpoint,
    helios::{
        consensus::{calc_sync_period, expected_current_slot},
        spec::MainnetConsensusSpec,
        types::{Forks, GenericUpdate, Update},
    },
    EthereumLightClientConsensus,
};
use ic_lightclient_wire::{Block, LightClientStatePayload, LightClientUpdatePayload};
use std::time::SystemTime;

const MAX_REQUEST_LIGHT_CLIENT_UPDATES: u8 = 128;

#[derive(Default)]
pub struct EthereumChain {
    light_client_store: EthereumLightClientConsensus<MainnetConsensusSpec>,
    genesis_time: u64,
    genesis_validator_root: B256,
    forks: Forks,
    last_updated_time_sec: u64,
    state_differ: EthereumStateDiff<MainnetConsensusSpec>,
}

impl EthereumChain {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn init(&mut self, canister_state: LightClientStatePayload<MainnetConsensusSpec>) {
        // TODO: fix
        let LightClientStatePayload::Bootstrap(checkpoint) = canister_state else {
            panic!("Canister not in bootstrap state.");
        };

        // TODO supposed to change. config should be fetched from canister.
        let config = Config::ethereum();
        let config = config.populate(EthereumCheckpoint { checkpoint_block_root: checkpoint });

        ExecutionApi::init(config.execution_api.clone());
        ConsensusApi::init(config.consensus_api.clone());

        self.genesis_time = config.genesis_time;
        self.genesis_validator_root = config.genesis_validator_root;
        self.forks = config.forks.clone();

        let bootstrap = ConsensusApi::bootstrap(checkpoint).await;
        self.light_client_store = EthereumLightClientConsensus::new(config);
        self.light_client_store.bootstrap(&bootstrap).unwrap();
        self.state_differ.add_bootstrap(bootstrap);
        println!("Ethereum light client initialized with bootstrap data.");
    }

    pub async fn get_updates(
        &mut self,
        canister_state: LightClientStatePayload<MainnetConsensusSpec>,
    ) -> Option<Vec<LightClientUpdatePayload<MainnetConsensusSpec>>> {
        self.check_and_sync().await;

        // check for next sync committee

        let mut updates = self.state_differ.get_diff_updates(&canister_state, &self.light_client_store);

        if updates.len() > 0 {
            let block = self.get_latest_block_update().await;
            updates.push(block);

            Some(updates)
        } else {
            None
        }
    }

    async fn get_latest_block_update(&self) -> LightClientUpdatePayload<MainnetConsensusSpec> {
        let base_gas_fee = ExecutionApi::base_gas_fee().await.try_into().unwrap();
        let max_priority_fee = ExecutionApi::max_priority_fee().await.try_into().unwrap();

        LightClientUpdatePayload::Block(Block { base_gas_fee, max_priority_fee })
    }

    async fn check_and_sync(&mut self) {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_time_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let genesis_time = self.genesis_time;
        let current_slot = expected_current_slot(current_time_ns.try_into().unwrap(), genesis_time);

        if current_time_sec - self.last_updated_time_sec < 12 {
            return;
        }

        let optimistic_slot = self.light_client_store.get_optimistic_slot();
        let finalized_slot = self.light_client_store.get_finalized_slot();
        let is_next_sync_committee_known = self.light_client_store.is_next_sync_committee_known();

        let current_period = calc_sync_period::<MainnetConsensusSpec>(current_slot);
        let optimistic_period = calc_sync_period::<MainnetConsensusSpec>(optimistic_slot);
        let finalized_period = calc_sync_period::<MainnetConsensusSpec>(finalized_slot);

        if finalized_period == optimistic_period && is_next_sync_committee_known == false {
            let update = ConsensusApi::updates(finalized_period, 1).await;

            if update.len() == 1 {
                self.verify_and_apply_generic_update((&update[0]).into());
            }
        } else if finalized_period + 1 < current_period {
            self.sync(current_period, finalized_period).await;
        } else {
            self.sync_head().await;
        }

        let new_optimistic_slot = self.light_client_store.get_optimistic_slot();

        if new_optimistic_slot != optimistic_slot {
            self.last_updated_time_sec = current_time_sec;
        }
    }

    async fn sync(&mut self, current_period: u64, mut finalized_period: u64) {
        println!("Syncing...");
        let mut updates: Vec<Update<MainnetConsensusSpec>> = vec![];
        if current_period - finalized_period >= 128 {
            while finalized_period < current_period {
                let batch_size =
                    std::cmp::min(current_period - finalized_period, MAX_REQUEST_LIGHT_CLIENT_UPDATES.into());

                let update = ConsensusApi::updates(finalized_period, batch_size).await;
                updates.extend(update);

                finalized_period += batch_size;
            }
        }

        let update = ConsensusApi::updates(finalized_period, MAX_REQUEST_LIGHT_CLIENT_UPDATES.into()).await;
        updates.extend(update);

        for update in updates {
            self.verify_and_apply_generic_update((&update).into());
        }

        self.sync_head().await;
    }

    async fn sync_head(&mut self) {
        let optimistic_update = tokio::spawn(async { ConsensusApi::optimistic_update().await });
        let finality_update = tokio::spawn(async { ConsensusApi::finality_update().await });

        let optimistic_update = optimistic_update.await.unwrap();
        let finality_update = finality_update.await.unwrap();

        self.verify_and_apply_generic_update((&optimistic_update).into());
        self.verify_and_apply_generic_update((&finality_update).into());
    }

    fn verify_and_apply_generic_update(&mut self, update: GenericUpdate<MainnetConsensusSpec>) {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_time_ns = current_time_ns.try_into().unwrap();
        let result = self.light_client_store.update(&update, current_time_ns);

        if !result.is_ok() {
            println!("Result is err: {:?}", result.err().unwrap());
        }
    }
}
