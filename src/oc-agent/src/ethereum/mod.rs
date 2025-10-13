mod api;
mod diff;

use crate::config::Config;
use alloy_primitives::B256;
use api::{ConsensusApi, ExecutionApi};
use diff::EthereumStateDiff;
use ic_lightclient_ethereum::{
    helios::{
        consensus::{
            apply_bootstrap, apply_generic_update, calc_sync_period, expected_current_slot, verify_generic_update,
        },
        spec::MainnetConsensusSpec,
        types::{FinalityUpdate, Forks, GenericUpdate, LightClientStore, OptimisticUpdate, Update},
    },
    payload::{LightClientStatePayload, LightClientUpdatePayload},
};
use std::time::SystemTime;

const MAX_REQUEST_LIGHT_CLIENT_UPDATES: u8 = 128;

#[derive(Default)]
pub struct EthereumChain {
    light_client_store: LightClientStore<MainnetConsensusSpec>,
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
        // TODO supposed to change. config should be fetched from canister.
        let config = Config::ethereum();

        ExecutionApi::init(config.execution_api);
        ConsensusApi::init(config.consensus_api);

        self.genesis_time = config.genesis_time;
        self.genesis_validator_root = config.genesis_validator_root;
        self.forks = config.forks;

        let LightClientStatePayload::Bootstrap(canister_state) = canister_state else {
            panic!("Canister not in bootstrap state.");
        };

        let bootstrap = ConsensusApi::bootstrap(canister_state.block_hash).await;
        apply_bootstrap(&mut self.light_client_store, &bootstrap);

        self.state_differ.add_bootstrap(bootstrap);
        println!("Ethereum light client initialized with bootstrap data.");
    }

    pub async fn get_updates(
        &mut self,
        canister_state: LightClientStatePayload<MainnetConsensusSpec>,
    ) -> Option<Vec<LightClientUpdatePayload<MainnetConsensusSpec>>> {
        self.check_and_sync().await;

        // check for next sync committee

        let updates = self.state_differ.get_diff_updates(&canister_state, &self.light_client_store);

        if updates.len() > 0 {
            Some(updates)
        } else {
            None
        }
    }

    async fn check_and_sync(&mut self) {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_time_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let genesis_time = self.genesis_time;
        let current_slot = expected_current_slot(current_time_ns.try_into().unwrap(), genesis_time);

        if current_time_sec - self.last_updated_time_sec < 12 {
            return;
        }

        let optimistic_slot = self.light_client_store.optimistic_header.beacon.slot;
        let finalized_slot = self.light_client_store.finalized_header.beacon.slot;
        let is_next_sync_committee_known = self.light_client_store.next_sync_committee.is_some();

        let current_period = calc_sync_period::<MainnetConsensusSpec>(current_slot);
        let optimistic_period = calc_sync_period::<MainnetConsensusSpec>(optimistic_slot);
        let finalized_period = calc_sync_period::<MainnetConsensusSpec>(finalized_slot);

        if finalized_period == optimistic_period && is_next_sync_committee_known == false {
            let update = ConsensusApi::updates(finalized_period, 1).await;

            if update.len() == 1 {
                self.verify_and_apply_update(&update[0]);
            }
        } else if finalized_period + 1 < current_period {
            self.sync(current_period, finalized_period).await;
        } else {
            self.sync_head().await;
        }

        let new_optimistic_slot = self.light_client_store.optimistic_header.beacon.slot;

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
            self.verify_and_apply_update(&update);
        }

        self.sync_head().await;
    }

    async fn sync_head(&mut self) {
        let optimistic_update = tokio::spawn(async { ConsensusApi::optimistic_update().await });
        let finality_update = tokio::spawn(async { ConsensusApi::finality_update().await });

        let optimistic_update = optimistic_update.await.unwrap();
        let finality_update = finality_update.await.unwrap();

        self.verify_and_apply_optimistic_update(&optimistic_update);
        self.verify_and_apply_finality_update(&finality_update);
    }

    fn verify_and_apply_finality_update(&mut self, update: &FinalityUpdate<MainnetConsensusSpec>) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_optimistic_update(&mut self, update: &OptimisticUpdate<MainnetConsensusSpec>) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_update(&mut self, update: &Update<MainnetConsensusSpec>) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_generic_update(&mut self, update: &GenericUpdate<MainnetConsensusSpec>) {
        let genesis_root = self.genesis_validator_root;
        let genesis_time = self.genesis_time;
        let forks = &self.forks;
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_slot = expected_current_slot(current_time_ns.try_into().unwrap(), genesis_time);

        let result = verify_generic_update(update, current_slot, &mut self.light_client_store, genesis_root, forks);
        if result.is_ok() {
            apply_generic_update(&mut self.light_client_store, update);
        } else {
            println!("Result is err: {:?}", result.err().unwrap());
        }
    }
}
