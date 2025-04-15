use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use typenum::Unsigned;

pub trait ConsensusSpec: 'static + Default + Sync + Send + Clone + Debug + PartialEq {
    type MaxValidatorsPerCommittee: Unsigned + Default + Debug + Sync + Send + Clone + PartialEq;
    type SlotsPerEpoch: Unsigned + Default + Debug + Sync + Send + Clone + PartialEq;
    type EpochsPerSyncCommitteePeriod: Unsigned + Default + Debug + Sync + Send + Clone + PartialEq;
    type SyncCommitteeSize: Unsigned + Default + Debug + Sync + Send + Clone + PartialEq;

    fn slots_per_epoch() -> u64 {
        Self::SlotsPerEpoch::to_u64()
    }

    fn epochs_per_sync_committee_period() -> u64 {
        Self::EpochsPerSyncCommitteePeriod::to_u64()
    }

    fn slots_per_sync_committee_period() -> u64 {
        Self::slots_per_epoch() * Self::epochs_per_sync_committee_period()
    }

    fn sync_committee_size() -> u64 {
        Self::SyncCommitteeSize::to_u64()
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct MainnetConsensusSpec;

impl ConsensusSpec for MainnetConsensusSpec {
    type MaxValidatorsPerCommittee = typenum::U2048;
    type SlotsPerEpoch = typenum::U32;
    type EpochsPerSyncCommitteePeriod = typenum::U256;
    type SyncCommitteeSize = typenum::U512;
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
pub struct MinimalConsensusSpec;

impl ConsensusSpec for MinimalConsensusSpec {
    type MaxValidatorsPerCommittee = typenum::U2048;
    type SlotsPerEpoch = typenum::U8;
    type EpochsPerSyncCommitteePeriod = typenum::U8;
    type SyncCommitteeSize = typenum::U32;
}