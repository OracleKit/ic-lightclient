use ic_lightclient_ethereum::{helios::{consensus::calc_sync_period, spec::MainnetConsensusSpec, types::{Bootstrap, FinalityUpdate, OptimisticUpdate, Update}}, payload::{LightClientStatePayload, LightClientUpdatePayload}};

const MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF: usize = 20;

#[derive(Debug, Clone)]
enum SlotUpdate {
    WithSyncCommittee(Update<MainnetConsensusSpec>),
    WithoutSyncCommittee(
        OptimisticUpdate<MainnetConsensusSpec>,
        FinalityUpdate<MainnetConsensusSpec>
    )
}

impl SlotUpdate {
    fn slot(&self) -> u64 {
        match self {
            SlotUpdate::WithSyncCommittee(update) => update.attested_header.beacon.slot,
            SlotUpdate::WithoutSyncCommittee(update, _) => update.attested_header.beacon.slot
        }
    }
}

pub struct EthereumStateDiff {
    head: usize,
    updates: Vec<Option<SlotUpdate>>,
    bootstrap: Option<Bootstrap<MainnetConsensusSpec>>
}

impl Default for EthereumStateDiff {
    fn default() -> Self {
        let mut updates = Vec::new();
        updates.resize(MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF, None);

        Self {
            head: 0,
            updates,
            bootstrap: None
        }
    }
}

impl EthereumStateDiff {
    fn get_index_for_period(
        &self,
        period: u64
    ) -> Option<usize> {
        let tail_index = (self.head + MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF - 1) % MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF;
        let Some(tail_update) = &self.updates[tail_index] else {
            return Some(self.head);
        };

        let tail_slot = tail_update.slot();
        let tail_period = calc_sync_period::<MainnetConsensusSpec>(tail_slot);

        if period <= tail_period {
            let period_diff = (tail_period - period) as usize;
            if period_diff >= MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF {
                return None;
            }

            let index = (tail_index + MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF - period_diff) % MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF;
            Some(index)
        } else {
            Some(self.head)
        }
    }

    fn increment_head(&mut self) {
        self.head = (self.head + 1) % MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF;
    }

    fn add_generic_update(
        &mut self,
        update: SlotUpdate
    ) {
        let update_slot = update.slot();
        let update_period = calc_sync_period::<MainnetConsensusSpec>(update_slot);
        let Some(update_index) = self.get_index_for_period(update_period) else { return; };

        if let Some(existing_update) = &self.updates[update_index] {
            if existing_update.slot() > update_slot {
                return;
            }
        }

        self.updates[update_index] = Some(update);
        if update_index == self.head {
            self.increment_head();
        }
    }

    pub fn add_update_with_sync_committee(
        &mut self,
        update: Update<MainnetConsensusSpec>
    ) {
        self.add_generic_update(SlotUpdate::WithSyncCommittee(update));
    }

    pub fn add_update_without_sync_committee(
        &mut self,
        optimistic_update: OptimisticUpdate<MainnetConsensusSpec>,
        finality_update: FinalityUpdate<MainnetConsensusSpec>
    ) {
        self.add_generic_update(SlotUpdate::WithoutSyncCommittee(optimistic_update, finality_update));
    }

    pub fn add_bootstrap(
        &mut self,
        bootstrap: Bootstrap<MainnetConsensusSpec>
    ) {
        self.bootstrap = Some(bootstrap);
    }

    pub fn get_diff_updates(&self, state: &LightClientStatePayload<MainnetConsensusSpec>) -> Vec<LightClientUpdatePayload<MainnetConsensusSpec>> {
        let slot: u64;
        let mut updates = vec![];

        match &state {
            LightClientStatePayload::Bootstrap(_state) => {
                let bootstrap_update = self.bootstrap.as_ref().expect("Bootstrap update not found");

                // if state.block_hash != bootstrap_update.header.beacon.tree_hash_root() {
                //     panic!("Bootstrap block hash mismatch, {:?}", bootstrap_update.header);
                // }
    
                updates.push(LightClientUpdatePayload::Bootstrap(bootstrap_update.clone().into()));
                slot = bootstrap_update.header.beacon.slot;
                println!("Received request for bootstrap!");
            }
            LightClientStatePayload::Active(state) => {
                slot = state.optimistic_header.beacon.slot;
                println!("Received request for slot: {}!", slot);
            }
        }

        let current_period = calc_sync_period::<MainnetConsensusSpec>(slot);
        let Some(mut current_index) = self.get_index_for_period(current_period) else {
            panic!("No index found for current_period");
        };

        let mut slot_updates = vec![];
        let Some(current_update) = &self.updates[current_index] else {
            panic!("No update found for current period");
        };

        if current_update.slot() > slot {
            slot_updates.push(current_update);
        }

        current_index = (current_index + 1) % MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF;
        while current_index != self.head {
            let Some(update) = &self.updates[current_index] else {
                panic!("No update found for current period");
            };

            slot_updates.push(update);
            current_index = (current_index + 1) % MAX_LIGHT_CLIENT_SLOTS_FOR_DIFF;
        }

        for update in slot_updates {
            match update {
                SlotUpdate::WithSyncCommittee(update) => {
                    updates.push(LightClientUpdatePayload::Update(update.clone().into()));
                }
                SlotUpdate::WithoutSyncCommittee(optimistic_update, finality_update) => {
                    updates.push(LightClientUpdatePayload::OptimisticUpdate(optimistic_update.clone().into()));
                    updates.push(LightClientUpdatePayload::FinalityUpdate(finality_update.clone().into()));
                }
            }
        }
        
        updates
    }
}