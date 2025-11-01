use crate::http::HttpClient;
use alloy_primitives::B256;
use anyhow::{anyhow, Result};
use ic_lightclient_ethereum::helios::{
    spec::MainnetConsensusSpec,
    types::{Bootstrap, FinalityUpdate, OptimisticUpdate, Update},
};
use serde::{de::DeserializeOwned, Deserialize};
use std::sync::OnceLock;

static INNER: OnceLock<Inner> = OnceLock::new();

#[derive(Debug)]
struct Inner {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ResponseWrapper<T> {
    #[allow(dead_code)]
    version: String,
    data: T,
}

pub struct ConsensusApi;

impl ConsensusApi {
    pub fn init(url: String) -> Result<()> {
        INNER.set(Inner { url }).map_err(|_| anyhow!("ConsensusApi already init."))
    }

    async fn request<Response: DeserializeOwned>(path: &str, query: &[(&str, &str)]) -> Result<Response> {
        let inner = INNER.get().ok_or(anyhow!("ConsensusApi called before init."))?;
        let url = &inner.url;
        let url = format!("{}{}", url, path);

        let response = HttpClient::get(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&query)
            .send()
            .await?;

        let response = response.json().await?;
        Ok(response)
    }

    pub async fn bootstrap(block_root: B256) -> Result<Bootstrap<MainnetConsensusSpec>> {
        let response: ResponseWrapper<Bootstrap<MainnetConsensusSpec>> =
            Self::request(&format!("/eth/v1/beacon/light_client/bootstrap/{}", block_root), &[]).await?;

        Ok(response.data)
    }

    pub async fn updates(start_period: u64, count: u64) -> Result<Vec<Update<MainnetConsensusSpec>>> {
        let response: Vec<ResponseWrapper<Update<MainnetConsensusSpec>>> = Self::request(
            "/eth/v1/beacon/light_client/updates",
            &[("start_period", &start_period.to_string()), ("count", &count.to_string())],
        )
        .await?;

        let response = response.into_iter().map(|r| r.data).collect();
        Ok(response)
    }

    pub async fn optimistic_update() -> Result<OptimisticUpdate<MainnetConsensusSpec>> {
        let response: ResponseWrapper<OptimisticUpdate<MainnetConsensusSpec>> =
            Self::request("/eth/v1/beacon/light_client/optimistic_update", &[]).await?;

        Ok(response.data)
    }

    pub async fn finality_update() -> Result<FinalityUpdate<MainnetConsensusSpec>> {
        let response: ResponseWrapper<FinalityUpdate<MainnetConsensusSpec>> =
            Self::request("/eth/v1/beacon/light_client/finality_update", &[]).await?;

        Ok(response.data)
    }
}
