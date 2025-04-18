use std::sync::OnceLock;
use alloy_primitives::B256;
use serde::{de::DeserializeOwned, Deserialize};
use crate::http::HttpClient;
use ic_lightclient_ethereum::helios::{spec::MainnetConsensusSpec, types::Bootstrap};

static INNER: OnceLock<Inner> = OnceLock::new();

#[derive(Debug)]
struct Inner {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ResponseWrapper<T> {
    version: String,
    data: T
}

pub struct ConsensusApi;

impl ConsensusApi {
    pub fn init(url: String) {
        INNER.set(Inner { url }).unwrap();
    }

    async fn request<Response: DeserializeOwned>(
        path: &str,
        query: &[(&str, &str)],
    ) -> Response {
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

        let response: ResponseWrapper<Response> = response.json().await.expect("Failed to parse response");
        response.data
    }

    pub async fn bootstrap(block_root: B256) -> Bootstrap<MainnetConsensusSpec> {
        Self::request(
            &format!("/eth/v1/beacon/light_client/bootstrap/{}", block_root),
            &[]
        ).await
    }
}