use crate::helios::spec::ConsensusSpec;
use alloy_primitives::{Address, FixedBytes, B256, U256};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, FixedVector};
use superstruct::superstruct;
use tree_hash_derive::TreeHash;

use self::{
    bls::{PublicKey, Signature},
    bytes::{ByteList, ByteVector},
};

pub mod bls;
mod bytes;
mod serde_utils;

pub type LogsBloom = ByteVector<typenum::U256>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LightClientStore<S: ConsensusSpec> {
    pub finalized_header: LightClientHeader,
    pub current_sync_committee: SyncCommittee<S>,
    pub next_sync_committee: Option<SyncCommittee<S>>,
    pub optimistic_header: LightClientHeader,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
    pub best_valid_update: Option<GenericUpdate<S>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Encode, Decode, TreeHash, Clone, PartialEq)]
pub struct BeaconBlockHeader {
    #[serde(with = "serde_utils::u64")]
    pub slot: u64,
    #[serde(with = "serde_utils::u64")]
    pub proposer_index: u64,
    pub parent_root: B256,
    pub state_root: B256,
    pub body_root: B256,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Encode, Decode, TreeHash, PartialEq)]
pub struct ExecutionPayloadHeader {
    pub parent_hash: B256,
    pub fee_recipient: Address,
    pub state_root: B256,
    pub receipts_root: B256,
    pub logs_bloom: LogsBloom,
    pub prev_randao: B256,
    #[serde(with = "serde_utils::u64")]
    pub block_number: u64,
    #[serde(with = "serde_utils::u64")]
    pub gas_limit: u64,
    #[serde(with = "serde_utils::u64")]
    pub gas_used: u64,
    #[serde(with = "serde_utils::u64")]
    pub timestamp: u64,
    pub extra_data: ByteList<typenum::U32>,
    #[serde(with = "serde_utils::u256")]
    pub base_fee_per_gas: U256,
    pub block_hash: B256,
    pub transactions_root: B256,
    pub withdrawals_root: B256,
    #[serde(with = "serde_utils::u64")]
    pub blob_gas_used: u64,
    #[serde(with = "serde_utils::u64")]
    pub excess_blob_gas: u64,
}

#[superstruct(
    variants(Deneb, Electra),
    variant_attributes(
        derive(Serialize, Deserialize, Debug, Decode, Clone),
        serde(deny_unknown_fields),
        serde(bound = "S: ConsensusSpec"),
    )
)]
#[derive(Serialize, Deserialize, Debug, Decode, Clone)]
#[serde(untagged)]
#[serde(bound = "S: ConsensusSpec")]
#[ssz(enum_behaviour = "transparent")]
pub struct Bootstrap<S: ConsensusSpec> {
    pub header: LightClientHeader,
    pub current_sync_committee: SyncCommittee<S>,
    #[superstruct(
        only(Deneb),
        partial_getter(rename = "current_sync_committee_branch_deneb")
    )]
    pub current_sync_committee_branch: FixedVector<B256, typenum::U5>,
    #[superstruct(
        only(Electra),
        partial_getter(rename = "current_sync_committee_branch_electra")
    )]
    pub current_sync_committee_branch: FixedVector<B256, typenum::U6>,
}

impl<S: ConsensusSpec> Bootstrap<S> {
    pub fn current_sync_committee_branch(&self) -> &[B256] {
        match self {
            Bootstrap::Deneb(inner) => &inner.current_sync_committee_branch,
            Bootstrap::Electra(inner) => &inner.current_sync_committee_branch,
        }
    }
}

#[superstruct(
    variants(Deneb, Electra),
    variant_attributes(
        derive(Serialize, Deserialize, Debug, Clone, Decode,),
        serde(deny_unknown_fields),
        serde(bound = "S: ConsensusSpec"),
    )
)]
#[derive(Serialize, Deserialize, Debug, Clone, Decode)]
#[serde(untagged)]
#[serde(bound = "S: ConsensusSpec")]
#[ssz(enum_behaviour = "transparent")]
pub struct Update<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub next_sync_committee: SyncCommittee<S>,
    #[superstruct(
        only(Deneb),
        partial_getter(rename = "next_sync_committee_branch_deneb")
    )]
    pub next_sync_committee_branch: FixedVector<B256, typenum::U5>,
    #[superstruct(
        only(Electra),
        partial_getter(rename = "next_sync_committee_branch_electra")
    )]
    pub next_sync_committee_branch: FixedVector<B256, typenum::U6>,
    pub finalized_header: LightClientHeader,
    #[superstruct(only(Deneb), partial_getter(rename = "finality_branch_deneb"))]
    pub finality_branch: FixedVector<B256, typenum::U6>,
    #[superstruct(only(Electra), partial_getter(rename = "finality_branch_electra"))]
    pub finality_branch: FixedVector<B256, typenum::U7>,
    pub sync_aggregate: SyncAggregate<S>,
    #[serde(with = "serde_utils::u64")]
    pub signature_slot: u64,
}

impl<S: ConsensusSpec> Update<S> {
    pub fn next_sync_committee_branch(&self) -> &[B256] {
        match self {
            Update::Deneb(inner) => &inner.next_sync_committee_branch,
            Update::Electra(inner) => &inner.next_sync_committee_branch,
        }
    }

    pub fn finality_branch(&self) -> &[B256] {
        match self {
            Update::Deneb(inner) => &inner.finality_branch,
            Update::Electra(inner) => &inner.finality_branch,
        }
    }
}

#[superstruct(
    variants(Deneb, Electra),
    variant_attributes(
        derive(Serialize, Deserialize, Debug, Clone, Decode,),
        serde(deny_unknown_fields),
        serde(bound = "S: ConsensusSpec"),
    )
)]
#[derive(Serialize, Deserialize, Debug, Clone, Decode)]
#[serde(untagged)]
#[serde(bound = "S: ConsensusSpec")]
#[ssz(enum_behaviour = "transparent")]
pub struct FinalityUpdate<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub finalized_header: LightClientHeader,
    #[superstruct(only(Deneb), partial_getter(rename = "finality_branch_deneb"))]
    pub finality_branch: FixedVector<B256, typenum::U6>,
    #[superstruct(only(Electra), partial_getter(rename = "finality_branch_electra"))]
    pub finality_branch: FixedVector<B256, typenum::U7>,
    pub sync_aggregate: SyncAggregate<S>,
    #[serde(with = "serde_utils::u64")]
    pub signature_slot: u64,
}

impl<S: ConsensusSpec> FinalityUpdate<S> {
    pub fn finality_branch(&self) -> &[B256] {
        match self {
            FinalityUpdate::Deneb(inner) => &inner.finality_branch,
            FinalityUpdate::Electra(inner) => &inner.finality_branch,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Decode)]
pub struct OptimisticUpdate<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub sync_aggregate: SyncAggregate<S>,
    #[serde(with = "serde_utils::u64")]
    pub signature_slot: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Decode, PartialEq)]
pub struct LightClientHeader {
    pub beacon: BeaconBlockHeader,
    pub execution: ExecutionPayloadHeader,
    pub execution_branch: FixedVector<B256, typenum::U4>,
}

#[derive(Debug, Clone, Default, Encode, TreeHash, Serialize, Deserialize, Decode, PartialEq)]
pub struct SyncCommittee<S: ConsensusSpec> {
    pub pubkeys: FixedVector<PublicKey, S::SyncCommitteeSize>,
    pub aggregate_pubkey: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Encode, Decode, TreeHash, PartialEq)]
pub struct SyncAggregate<S: ConsensusSpec> {
    pub sync_committee_bits: BitVector<S::SyncCommitteeSize>,
    pub sync_committee_signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Forks {
    pub genesis: Fork,
    pub altair: Fork,
    pub bellatrix: Fork,
    pub capella: Fork,
    pub deneb: Fork,
    pub electra: Fork,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Fork {
    pub epoch: u64,
    pub fork_version: FixedBytes<4>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenericUpdate<S: ConsensusSpec> {
    pub attested_header: LightClientHeader,
    pub sync_aggregate: SyncAggregate<S>,
    pub signature_slot: u64,
    pub next_sync_committee: Option<SyncCommittee<S>>,
    pub next_sync_committee_branch: Option<Vec<B256>>,
    pub finalized_header: Option<LightClientHeader>,
    pub finality_branch: Option<Vec<B256>>,
}

impl<S: ConsensusSpec> From<&Update<S>> for GenericUpdate<S> {
    fn from(update: &Update<S>) -> Self {
        Self {
            attested_header: update.attested_header().clone(),
            sync_aggregate: update.sync_aggregate().clone(),
            signature_slot: *update.signature_slot(),
            next_sync_committee: default_to_none(update.next_sync_committee().clone()),
            next_sync_committee_branch: default_branch_to_none(
                &update.next_sync_committee_branch(),
            ),
            finalized_header: default_header_to_none(update.finalized_header().clone()),
            finality_branch: default_branch_to_none(&update.finality_branch()),
        }
    }
}

impl<S: ConsensusSpec> From<&FinalityUpdate<S>> for GenericUpdate<S> {
    fn from(update: &FinalityUpdate<S>) -> Self {
        Self {
            attested_header: update.attested_header().clone(),
            sync_aggregate: update.sync_aggregate().clone(),
            signature_slot: *update.signature_slot(),
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: default_header_to_none(update.finalized_header().clone()),
            finality_branch: default_branch_to_none(&update.finality_branch()),
        }
    }
}

impl<S: ConsensusSpec> From<&OptimisticUpdate<S>> for GenericUpdate<S> {
    fn from(update: &OptimisticUpdate<S>) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot,
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: None,
            finality_branch: None,
        }
    }
}

fn default_to_none<T: Default + PartialEq>(value: T) -> Option<T> {
    if value == T::default() {
        None
    } else {
        Some(value)
    }
}

fn default_branch_to_none(value: &[B256]) -> Option<Vec<B256>> {
    for elem in value {
        if !elem.is_zero() {
            return Some(value.to_vec());
        }
    }

    None
}

fn default_header_to_none(value: LightClientHeader) -> Option<LightClientHeader> {
    if value.beacon == BeaconBlockHeader::default()
        && value.execution == ExecutionPayloadHeader::default()
    {
        None
    } else {
        Some(value)
    }
}
