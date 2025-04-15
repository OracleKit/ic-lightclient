use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct FixedBytes<const N: usize>(pub [u8; N]);

#[derive(Encode, Decode, Debug)]
pub struct FixedUint<const N: usize>(pub [u64; N]);

pub type H256 = FixedBytes<32>;
pub type Address = FixedBytes<20>;
pub type Bloom = FixedBytes<256>;
pub type U256 = FixedUint<4>;