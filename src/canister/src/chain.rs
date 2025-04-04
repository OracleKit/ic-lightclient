use enum_dispatch::enum_dispatch;
use ic_lightclient_types::{ChainState, ChainUpdates};
use crate::ethereum::EthereumChain;

#[enum_dispatch]
pub trait ChainInterface {
    fn get_state(&self) -> ChainState;
    fn are_updates_valid(&self, updates: ChainUpdates) -> bool;
    fn update_state(&mut self, updates: ChainUpdates);
}

#[enum_dispatch(ChainInterface)]
pub enum Chain {
    Ethereum(EthereumChain)
}