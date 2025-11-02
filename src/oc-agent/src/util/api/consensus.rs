use crate::http::HttpClient;
use alloy_primitives::B256;
use anyhow::Result;
use ic_lightclient_ethereum::helios::{
    spec::MainnetConsensusSpec,
    types::{Bootstrap, FinalityUpdate, OptimisticUpdate, Update},
};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Deserialize)]
struct ResponseWrapper<T> {
    #[allow(dead_code)]
    version: String,
    data: T,
}

#[derive(Default, Clone)]
pub struct ConsensusApi {
    url: String,
}

impl ConsensusApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    async fn request<Response: DeserializeOwned>(&self, path: &str, query: &[(&str, &str)]) -> Result<Response> {
        let url = &self.url;
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

    pub async fn bootstrap(&self, block_root: B256) -> Result<Bootstrap<MainnetConsensusSpec>> {
        let response: ResponseWrapper<Bootstrap<MainnetConsensusSpec>> = self
            .request(&format!("/eth/v1/beacon/light_client/bootstrap/{}", block_root), &[])
            .await?;

        Ok(response.data)
    }

    pub async fn updates(&self, start_period: u64, count: u64) -> Result<Vec<Update<MainnetConsensusSpec>>> {
        let response: Vec<ResponseWrapper<Update<MainnetConsensusSpec>>> = self
            .request(
                "/eth/v1/beacon/light_client/updates",
                &[("start_period", &start_period.to_string()), ("count", &count.to_string())],
            )
            .await?;

        let response = response.into_iter().map(|r| r.data).collect();
        Ok(response)
    }

    pub async fn optimistic_update(&self) -> Result<OptimisticUpdate<MainnetConsensusSpec>> {
        let response: ResponseWrapper<OptimisticUpdate<MainnetConsensusSpec>> =
            self.request("/eth/v1/beacon/light_client/optimistic_update", &[]).await?;

        Ok(response.data)
    }

    pub async fn finality_update(&self) -> Result<FinalityUpdate<MainnetConsensusSpec>> {
        let response: ResponseWrapper<FinalityUpdate<MainnetConsensusSpec>> =
            self.request("/eth/v1/beacon/light_client/finality_update", &[]).await?;

        Ok(response.data)
    }
}
