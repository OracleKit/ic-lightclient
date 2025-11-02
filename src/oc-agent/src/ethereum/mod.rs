mod diff;

use crate::{
    chain::StateMachine,
    util::{ConsensusApi, ExecutionApi},
};
use alloy_primitives::B256;
use anyhow::Result;
use async_trait::async_trait;
use diff::EthereumStateDiff;
use ic_lightclient_ethereum::{
    config::EthereumConfigPopulated,
    helios::{
        consensus::{calc_sync_period, expected_current_slot},
        spec::MainnetConsensusSpec,
        types::{Forks, GenericUpdate, Update},
    },
    EthereumLightClientConsensus,
};
use ic_lightclient_wire::ethereum::{Block, LightClientStatePayload, LightClientUpdatePayload};
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
    consensus_api: ConsensusApi,
    execution_api: ExecutionApi,
}

#[async_trait]
impl StateMachine for EthereumChain {
    type Config = EthereumConfigPopulated;
    type CanisterStatePayload = LightClientStatePayload<MainnetConsensusSpec>;
    type CanisterUpdatePayload = LightClientUpdatePayload<MainnetConsensusSpec>;

    fn new() -> Self {
        Self::default()
    }

    async fn init(&mut self, config: EthereumConfigPopulated) -> Result<()> {
        self.execution_api = ExecutionApi::new(config.execution_api.clone());
        self.consensus_api = ConsensusApi::new(config.consensus_api.clone());

        self.genesis_time = config.genesis_time;
        self.genesis_validator_root = config.genesis_validator_root;
        self.forks = config.forks.clone();
        let checkpoint = config.checkpoint.checkpoint_block_root;

        let bootstrap = self.consensus_api.bootstrap(checkpoint).await?;
        self.light_client_store = EthereumLightClientConsensus::new(config);
        self.light_client_store.bootstrap(&bootstrap)?;
        self.state_differ.add_bootstrap(bootstrap);
        println!("Ethereum light client initialized with bootstrap data.");

        Ok(())
    }

    async fn get_updates(
        &mut self,
        canister_state: LightClientStatePayload<MainnetConsensusSpec>,
    ) -> Result<Vec<LightClientUpdatePayload<MainnetConsensusSpec>>> {
        self.check_and_sync().await?;

        // check for next sync committee

        let mut updates = self.state_differ.get_diff_updates(&canister_state, &self.light_client_store)?;

        if updates.len() > 0 {
            let block = self.get_latest_block_update().await?;
            updates.push(block);

            Ok(updates)
        } else {
            Ok(vec![])
        }
    }
}

impl EthereumChain {
    async fn get_latest_block_update(&self) -> Result<LightClientUpdatePayload<MainnetConsensusSpec>> {
        let base_gas_fee = self.execution_api.base_gas_fee().await?;
        let base_gas_fee = base_gas_fee.try_into()?;
        let max_priority_fee = self.execution_api.max_priority_fee().await?;
        let max_priority_fee = max_priority_fee.try_into()?;

        Ok(LightClientUpdatePayload::Block(Block { base_gas_fee, max_priority_fee }))
    }

    async fn check_and_sync(&mut self) -> Result<()> {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let current_time_ns = current_time_ns.as_nanos().try_into()?;
        let current_time_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let current_time_sec = current_time_sec.as_secs();
        let genesis_time = self.genesis_time;
        let current_slot = expected_current_slot(current_time_ns, genesis_time);

        if current_time_sec - self.last_updated_time_sec < 12 {
            return Ok(());
        }

        let optimistic_slot = self.light_client_store.get_optimistic_slot();
        let finalized_slot = self.light_client_store.get_finalized_slot();
        let is_next_sync_committee_known = self.light_client_store.is_next_sync_committee_known();

        let current_period = calc_sync_period::<MainnetConsensusSpec>(current_slot);
        let optimistic_period = calc_sync_period::<MainnetConsensusSpec>(optimistic_slot);
        let finalized_period = calc_sync_period::<MainnetConsensusSpec>(finalized_slot);

        if finalized_period == optimistic_period && is_next_sync_committee_known == false {
            let update = self.consensus_api.updates(finalized_period, 1).await?;

            if update.len() == 1 {
                self.verify_and_apply_generic_update((&update[0]).into())?;
            }
        } else if finalized_period + 1 < current_period {
            self.sync(current_period, finalized_period).await?;
        } else {
            self.sync_head().await?;
        }

        let new_optimistic_slot = self.light_client_store.get_optimistic_slot();

        if new_optimistic_slot != optimistic_slot {
            self.last_updated_time_sec = current_time_sec;
        }

        Ok(())
    }

    async fn sync(&mut self, current_period: u64, mut finalized_period: u64) -> Result<()> {
        println!("Syncing...");
        let mut updates: Vec<Update<MainnetConsensusSpec>> = vec![];
        if current_period - finalized_period >= 128 {
            while finalized_period < current_period {
                let batch_size =
                    std::cmp::min(current_period - finalized_period, MAX_REQUEST_LIGHT_CLIENT_UPDATES.into());

                let update = self.consensus_api.updates(finalized_period, batch_size).await?;
                updates.extend(update);

                finalized_period += batch_size;
            }
        }

        let update = self
            .consensus_api
            .updates(finalized_period, MAX_REQUEST_LIGHT_CLIENT_UPDATES.into())
            .await?;
        updates.extend(update);

        for update in updates {
            self.verify_and_apply_generic_update((&update).into())?;
        }

        self.sync_head().await
    }

    async fn sync_head(&mut self) -> Result<()> {
        let consensus_api = self.consensus_api.clone();
        let consensus_api_2 = self.consensus_api.clone();

        let optimistic_update = tokio::spawn(async move { consensus_api.optimistic_update().await });
        let finality_update = tokio::spawn(async move { consensus_api_2.finality_update().await });

        let optimistic_update = optimistic_update.await??;
        let finality_update = finality_update.await??;

        self.verify_and_apply_generic_update((&optimistic_update).into())?;
        self.verify_and_apply_generic_update((&finality_update).into())?;
        Ok(())
    }

    fn verify_and_apply_generic_update(&mut self, update: GenericUpdate<MainnetConsensusSpec>) -> Result<()> {
        let current_time_ns = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let current_time_ns = current_time_ns.as_nanos();
        let current_time_ns = current_time_ns.try_into()?;
        self.light_client_store.update(&update, current_time_ns)
    }
}
