use anyhow::{anyhow, Context, Result};
use ic_agent::{export::Principal, Agent};
use ic_lightclient_types::config::IcpConfig;
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
    pub async fn init(config: IcpConfig) -> Result<()> {
        let agent = Agent::builder()
            .with_url(config.agent_url.clone())
            .build()
            .context("Failed to create agent")?;

        agent.fetch_root_key().await?;
        INNER
            .set(Inner { agent, canister_id: config.canister_id.clone() })
            .map_err(|_| anyhow!("IcpAgent already initialized."))?;

        Ok(())
    }

    fn canister<'a>() -> Result<Canister<'a>> {
        let inner = INNER.get().ok_or(anyhow!("IcpAgent canister() called before init()"))?;

        Canister::builder()
            .with_agent(&inner.agent)
            .with_canister_id(inner.canister_id)
            .build()
            .map_err(|e| anyhow!("Error while building Canister: {:?}", e))
    }

    pub async fn get_canister_state() -> Result<Vec<u8>> {
        let canister = IcpAgent::canister()?;
        let (state,) = canister
            .query("get_state")
            .build()
            .call()
            .await
            .context("Failed to get canister state")?;

        Ok(state)
    }

    pub async fn list_chain_uids() -> Result<Vec<u16>> {
        let canister = IcpAgent::canister()?;
        let (uids,) = canister
            .query("list_chain_uids")
            .build()
            .call()
            .await
            .context("Failed to get configured chain uids")?;

        Ok(uids)
    }

    pub async fn get_canister_config(uid: u16) -> Result<Vec<u8>> {
        let canister = IcpAgent::canister()?;
        let (state,) = canister
            .query("get_chain_config")
            .with_arg(uid)
            .build()
            .call()
            .await
            .context("Failed to get canister config")?;

        Ok(state)
    }

    pub async fn update_canister_state(updates: Vec<u8>) -> Result<()> {
        let canister = IcpAgent::canister()?;
        canister
            .update("update_state")
            .with_arg(updates)
            .build()
            .call_and_wait()
            .await
            .context("Failed to update canister state")
    }

    pub async fn set_config(name: u16, value: String) -> Result<()> {
        let canister = IcpAgent::canister()?;
        canister
            .update("set_config")
            .with_args((name, value))
            .build()
            .call_and_wait()
            .await
            .context("Failed to set config in canister")
    }
}
