use alloy_primitives::{B256, Address, U256, FixedBytes};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;
use ssz_types::{FixedVector, BitVector};

use self::{
    bytes::{ByteList, ByteVector},
    bls::{PublicKey, Signature}
};

mod serde_utils;
mod bytes;
mod bls;

pub type LogsBloom = ByteVector<typenum::U256>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LightClientStore {
    pub finalized_header: LightClientHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: Option<SyncCommittee>,
    pub optimistic_header: LightClientHeader,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
    pub best_valid_update: Option<GenericUpdate>,
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

#[derive(Deserialize, Debug, Decode)]
pub struct Bootstrap {
    pub header: LightClientHeader,
    pub current_sync_committee: SyncCommittee,
    pub current_sync_committee_branch: FixedVector<B256, typenum::U6> // assuming Electra
}

#[derive(Serialize, Deserialize, Debug, Clone, Decode)]
pub struct Update {
    pub attested_header: LightClientHeader,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: FixedVector<B256, typenum::U6>,
    pub finalized_header: LightClientHeader,
    pub finality_branch: FixedVector<B256, typenum::U7>,
    pub sync_aggregate: SyncAggregate,
    #[serde(with = "serde_utils::u64")]
    pub signature_slot: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Decode)]
pub struct FinalityUpdate {
    pub attested_header: LightClientHeader,
    pub finalized_header: LightClientHeader,
    pub finality_branch: FixedVector<B256, typenum::U7>,
    pub sync_aggregate: SyncAggregate,
    #[serde(with = "serde_utils::u64")]
    pub signature_slot: u64,
}

#[derive(Serialize, Deserialize, Debug, Decode)]
pub struct OptimisticUpdate {
    pub attested_header: LightClientHeader,
    pub sync_aggregate: SyncAggregate,
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
pub struct SyncCommittee {
    pub pubkeys: FixedVector<PublicKey, typenum::U512>,
    pub aggregate_pubkey: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Encode, Decode, TreeHash)]
pub struct SyncAggregate {
    pub sync_committee_bits: BitVector<typenum::U512>,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GenericUpdate {
    pub attested_header: LightClientHeader,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: u64,
    pub next_sync_committee: Option<SyncCommittee>,
    pub next_sync_committee_branch: Option<Vec<B256>>,
    pub finalized_header: Option<LightClientHeader>,
    pub finality_branch: Option<Vec<B256>>,
}

impl From<&Update> for GenericUpdate {
    fn from(update: &Update) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot,
            next_sync_committee: default_to_none(update.next_sync_committee.clone()),
            next_sync_committee_branch: default_branch_to_none(&update.next_sync_committee_branch),
            finalized_header: default_header_to_none(update.finalized_header.clone()),
            finality_branch: default_branch_to_none(&update.finality_branch),
        }
    }
}

impl From<&FinalityUpdate> for GenericUpdate {
    fn from(update: &FinalityUpdate) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot,
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: default_header_to_none(update.finalized_header.clone()),
            finality_branch: default_branch_to_none(&update.finality_branch),
        }
    }
}

impl From<&OptimisticUpdate> for GenericUpdate {
    fn from(update: &OptimisticUpdate) -> Self {
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
    if value.beacon == BeaconBlockHeader::default() &&
       value.execution == ExecutionPayloadHeader::default() {
        None
    } else {
        Some(value)
    }
}