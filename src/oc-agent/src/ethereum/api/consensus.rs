use crate::http::HttpClient;
use alloy_primitives::B256;
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
    version: String,
    data: T,
}

pub struct ConsensusApi;

impl ConsensusApi {
    pub fn init(url: String) {
        INNER.set(Inner { url }).unwrap();
    }

    async fn request<Response: DeserializeOwned>(path: &str, query: &[(&str, &str)]) -> Response {
        let inner = INNER.get().unwrap();
        let url = &inner.url;
        let url = format!("{}{}", url, path);

        let response = HttpClient::get(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .query(&query)
            .send()
            .await
            .expect("Failed to send request");

        response.json().await.expect("Failed to parse response")
    }

    pub async fn bootstrap(block_root: B256) -> Bootstrap<MainnetConsensusSpec> {
        let response: ResponseWrapper<Bootstrap<MainnetConsensusSpec>> =
            Self::request(&format!("/eth/v1/beacon/light_client/bootstrap/{}", block_root), &[]).await;

        response.data
    }

    pub async fn updates(start_period: u64, count: u64) -> Vec<Update<MainnetConsensusSpec>> {
        let response: Vec<ResponseWrapper<Update<MainnetConsensusSpec>>> = Self::request(
            "/eth/v1/beacon/light_client/updates",
            &[("start_period", &start_period.to_string()), ("count", &count.to_string())],
        )
        .await;

        response.into_iter().map(|r| r.data).collect()
    }

    pub async fn optimistic_update() -> OptimisticUpdate<MainnetConsensusSpec> {
        let response: ResponseWrapper<OptimisticUpdate<MainnetConsensusSpec>> =
            Self::request("/eth/v1/beacon/light_client/optimistic_update", &[]).await;

        response.data
    }

    pub async fn finality_update() -> FinalityUpdate<MainnetConsensusSpec> {
        let response: ResponseWrapper<FinalityUpdate<MainnetConsensusSpec>> =
            Self::request("/eth/v1/beacon/light_client/finality_update", &[]).await;

        response.data
    }
}
