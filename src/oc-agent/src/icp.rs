use ic_lightclient_types::{CanisterState, CanisterUpdates};

pub struct ICP;

impl ICP {
    pub async fn get_canister_state() -> CanisterState {
        CanisterState::default()
    }

    pub async fn update_canister_state(updates: CanisterUpdates) {
        
    }
}