use ic_agent::{export::Principal, Agent};
use ic_lightclient_types::{config::IcpConfig, CanisterState, CanisterUpdates};
use ic_utils::{call::SyncCall, Canister};
use std::sync::OnceLock;

static INNER: OnceLock<Inner> = OnceLock::new();

#[derive(Debug)]
struct Inner {
    agent: Agent,
    canister_id: Principal,
}

pub struct IcpAgent;

impl IcpAgent {
    pub async fn init(config: IcpConfig) {
        let agent = Agent::builder()
            .with_url(config.agent_url.clone())
            .build()
            .expect("Failed to create agent");

        agent.fetch_root_key().await.unwrap();
        INNER.set(Inner { agent, canister_id: config.canister_id.clone() }).unwrap();
    }

    fn canister<'a>() -> Canister<'a> {
        let inner = INNER.get().unwrap();
        Canister::builder()
            .with_agent(&inner.agent)
            .with_canister_id(inner.canister_id)
            .build()
            .expect("Failed to create canister")
    }

    pub async fn get_canister_state() -> CanisterState {
        let canister = IcpAgent::canister();
        let (state,) = canister
            .query("get_state")
            .build()
            .call()
            .await
            .expect("Failed to get canister state");

        state
    }

    pub async fn update_canister_state(updates: CanisterUpdates) {
        let canister = IcpAgent::canister();
        let _: () = canister
            .update("update_state")
            .with_arg(updates)
            .build()
            .call_and_wait()
            .await
            .expect("Failed to update canister state");
    }

    pub async fn set_parameter(name: String, value: String) {
        let canister = IcpAgent::canister();
        let _: () = canister
            .update("set_parameter")
            .with_args((name, value))
            .build()
            .call_and_wait()
            .await
            .expect("Failed to set parameter");
    }
}
