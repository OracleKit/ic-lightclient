mod api;

use std::{sync::{Mutex, OnceLock}, time::SystemTime};
use alloy_primitives::B256;
use api::{ConsensusApi, ExecutionApi};
use ic_lightclient_ethereum::helios::{consensus::{apply_bootstrap, apply_generic_update, calc_sync_period, expected_current_slot, verify_generic_update}, spec::MainnetConsensusSpec, types::{FinalityUpdate, Forks, GenericUpdate, LightClientStore, OptimisticUpdate, Update}};
use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::chain::Chain;
use ic_lightclient_ethereum::config::EthereumConfig;

const MAX_REQUEST_LIGHT_CLIENT_UPDATES: u8 = 128;

pub struct EthereumChain {
    light_client_store: Mutex<LightClientStore<MainnetConsensusSpec>>,
    genesis_time: OnceLock<u64>,
    genesis_validator_root: OnceLock<B256>,
    forks: OnceLock<Forks>,
    last_updated_time_sec: Mutex<u64>,
}

impl Chain for EthereumChain {
    type ConfigType = EthereumConfig;

    fn new() -> Self {
        Self {
            light_client_store: Mutex::default(),
            genesis_time: OnceLock::new(),
            genesis_validator_root: OnceLock::new(),
            forks: OnceLock::new(),
            last_updated_time_sec: Mutex::new(0),
        }
    }

    async fn init(&self, config: EthereumConfig) {
        ExecutionApi::init(config.execution_api.clone());
        ConsensusApi::init(config.consensus_api.clone());

        self.genesis_time.set(config.genesis_time).unwrap();
        self.genesis_validator_root.set(config.genesis_validator_root).unwrap();
        self.forks.set(config.forks).unwrap();

        let bootstrap = ConsensusApi::bootstrap(config.checkpoint_block_root).await;
        let mut store = self.light_client_store.lock().unwrap();
        apply_bootstrap(&mut store, &bootstrap);
        println!("Ethereum light client initialized with bootstrap data.");
    }

    async fn get_updates(&self, state: ChainState) -> Option<ChainUpdates> {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_time_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let genesis_time = self.genesis_time.get().unwrap().clone();
        let current_slot = expected_current_slot(current_time_ns.try_into().unwrap(), genesis_time);

        let store = self.light_client_store.lock().unwrap();
        let last_updated_time_sec = *self.last_updated_time_sec.lock().unwrap();

        if current_time_sec - last_updated_time_sec < 12 {
            return None;
        }

        let optimistic_slot = store.optimistic_header.beacon.slot;
        let finalized_slot = store.finalized_header.beacon.slot;
        let is_next_sync_committee_known = store.next_sync_committee.is_some();
        drop(store);

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

        let store = self.light_client_store.lock().unwrap();
        let new_optimistic_slot = store.optimistic_header.beacon.slot;

        if new_optimistic_slot != optimistic_slot {
            let mut last_updated_time_sec = self.last_updated_time_sec.lock().unwrap();
            *last_updated_time_sec = current_time_sec;
        }

        None
    }
}

impl EthereumChain {
    async fn sync(&self, current_period: u64, mut finalized_period: u64) {
        println!("Syncing...");
        let mut updates: Vec<Update<MainnetConsensusSpec>> = vec![];
        if current_period - finalized_period >= 128 {
            while finalized_period < current_period {
                let batch_size = std::cmp::min(
                    current_period - finalized_period,
                    MAX_REQUEST_LIGHT_CLIENT_UPDATES.into(),
                );
                
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

    async fn sync_head(&self) {
        let optimistic_update = tokio::spawn(async { ConsensusApi::optimistic_update().await });
        let finality_update = tokio::spawn(async { ConsensusApi::finality_update().await });

        let optimistic_update = optimistic_update.await.unwrap();
        let finality_update = finality_update.await.unwrap();
        
        self.verify_and_apply_optimistic_update(&optimistic_update);
        self.verify_and_apply_finality_update(&finality_update);
    }

    fn verify_and_apply_finality_update(
        &self,
        update: &FinalityUpdate<MainnetConsensusSpec>
    ) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_optimistic_update(
        &self,
        update: &OptimisticUpdate<MainnetConsensusSpec>
    ) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_update(
        &self,
        update: &Update<MainnetConsensusSpec>
    ) {
        let update = GenericUpdate::from(update);
        self.verify_and_apply_generic_update(&update);
    }

    fn verify_and_apply_generic_update(
        &self,
        update: &GenericUpdate<MainnetConsensusSpec>
    ) {
        let mut store = self.light_client_store.lock().unwrap();
        let genesis_root = self.genesis_validator_root.get().unwrap();
        let genesis_time = self.genesis_time.get().unwrap();
        let forks = self.forks.get().unwrap();
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        let current_slot = expected_current_slot(current_time_ns.try_into().unwrap(), *genesis_time);

        verify_generic_update(update, current_slot, &store, *genesis_root, forks).unwrap();
        apply_generic_update(&mut store, update);
    }
}