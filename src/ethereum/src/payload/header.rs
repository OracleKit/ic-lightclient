use alloy_consensus::Sealed;
use alloy_primitives::{B256, U256};
use alloy_rpc_types_eth::Header;
use bincode::error::DecodeError;
use ssz_types::FixedVector;
use crate::helios::types::BeaconBlockHeader;

pub struct ConsensusBlockHeaderPayload {
    execution: ExecutionBlockHeaderPayload,
    beacon: BeaconBlockHeader,
    prev_randao: B256,
    execution_branch: FixedVector<>
}

pub struct ExecutionBlockHeaderPayload {
    inner: Header
}

impl ExecutionBlockHeaderPayload {
    pub fn verify(&self) -> bool {
        self.inner.hash_slow() == self.inner.hash
    }
}

impl<Context> bincode::Decode<Context> for ExecutionBlockHeaderPayload {
    fn decode<D: bincode::de::Decoder<Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let block_hash: [u8; 32] = bincode::Decode::decode(decoder)?;
        let parent_hash: [u8; 32] = bincode::Decode::decode(decoder)?;
        let ommers_hash: [u8; 32] = bincode::Decode::decode(decoder)?;
        let beneficiary: [u8; 20] = bincode::Decode::decode(decoder)?;
        let state_root: [u8; 32] = bincode::Decode::decode(decoder)?;
        let transaction_root: [u8; 32] = bincode::Decode::decode(decoder)?;
        let receipts_root: [u8; 32] = bincode::Decode::decode(decoder)?;
        let logs_bloom: [u8; 256] = bincode::Decode::decode(decoder)?;
        let difficulty: [u64; 4] = bincode::Decode::decode(decoder)?;
        let number: u64 = bincode::Decode::decode(decoder)?;
        let gas_limit: u64 = bincode::Decode::decode(decoder)?;
        let gas_used: u64 = bincode::Decode::decode(decoder)?;
        let timestamp: u64 = bincode::Decode::decode(decoder)?;
        let extra_data: Vec<u8> = bincode::Decode::decode(decoder)?;
        let mix_hash: [u8; 32] = bincode::Decode::decode(decoder)?;
        let nonce: [u8; 8] = bincode::Decode::decode(decoder)?;

        let consensus_header = alloy_consensus::Header {
            parent_hash: parent_hash.into(),
            ommers_hash: ommers_hash.into(),
            beneficiary: beneficiary.into(),
            state_root: state_root.into(),
            transactions_root: transaction_root.into(),
            receipts_root: receipts_root.into(),
            logs_bloom: logs_bloom.into(),
            difficulty: U256::from_limbs(difficulty),
            number,
            gas_limit,
            gas_used,
            timestamp,
            extra_data: extra_data.into(),
            mix_hash: mix_hash.into(),
            nonce: nonce.into(),
            base_fee_per_gas: None,
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
            parent_beacon_block_root: None,
            requests_hash: None
        };

        let header: Header = Header::from_consensus(
            Sealed::new_unchecked(
                consensus_header,
                block_hash.into()
            ),
            None,
            None
        );

        Ok(ExecutionBlockHeaderPayload { inner: header })
    }
}

impl bincode::Encode for ExecutionBlockHeaderPayload {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> core::result::Result<(), bincode::error::EncodeError> {
        let header = &self.inner;
        let consensus_header = &header.inner;

        header.hash.as_slice().encode(encoder)?;
        consensus_header.parent_hash.as_slice().encode(encoder)?;
        consensus_header.ommers_hash.as_slice().encode(encoder)?;
        consensus_header.beneficiary.as_slice().encode(encoder)?;
        consensus_header.state_root.as_slice().encode(encoder)?;
        consensus_header.transactions_root.as_slice().encode(encoder)?;
        consensus_header.receipts_root.as_slice().encode(encoder)?;
        consensus_header.logs_bloom.as_slice().encode(encoder)?;
        consensus_header.difficulty.into_limbs().encode(encoder)?;
        consensus_header.number.encode(encoder)?;
        consensus_header.gas_limit.encode(encoder)?;
        consensus_header.gas_used.encode(encoder)?;
        consensus_header.timestamp.encode(encoder)?;
        consensus_header.extra_data.as_ref().encode(encoder)?;
        consensus_header.mix_hash.as_slice().encode(encoder)?;
        consensus_header.nonce.as_slice().encode(encoder)?;

        Ok(())
    }
}

