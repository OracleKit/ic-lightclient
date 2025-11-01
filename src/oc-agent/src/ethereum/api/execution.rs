use crate::http::HttpClient;
use alloy_primitives::U256;
use alloy_rpc_types_eth::Header;
use anyhow::{anyhow, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::OnceLock;

static INNER: OnceLock<Inner> = OnceLock::new();

#[derive(Debug)]
struct Inner {
    url: String,
}

#[derive(Serialize)]
struct JsonRpcRequestWrapper<T> {
    id: u32,
    jsonrpc: String,
    method: String,
    params: T,
}

#[derive(Deserialize)]
struct JsonRpcResponseWrapper<T> {
    #[allow(dead_code)]
    id: u32,
    #[allow(dead_code)]
    jsonrpc: String,
    result: T,
}

pub struct ExecutionApi;

impl ExecutionApi {
    pub fn init(url: String) -> Result<()> {
        INNER.set(Inner { url }).map_err(|_| anyhow!("ExecutionApi already init."))
    }

    async fn request<Request: Serialize, Response: DeserializeOwned>(
        method: &str,
        request: Request,
    ) -> Result<Response> {
        let inner = INNER.get().ok_or(anyhow!("ExecutionApi request() called before init()"))?;
        let url = &inner.url;

        let response = HttpClient::post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&JsonRpcRequestWrapper {
                id: 1,
                jsonrpc: "2.0".to_string(),
                method: method.to_string(),
                params: request,
            })
            .send()
            .await?;

        let response: JsonRpcResponseWrapper<Response> = response.json().await?;
        Ok(response.result)
    }

    #[allow(dead_code)]
    pub async fn latest_block_number() -> Result<U256> {
        Self::request("eth_blockNumber", ()).await
    }

    #[allow(dead_code)]
    pub async fn block_header_by_number(block_number: U256) -> Result<Header> {
        Self::request("eth_getBlockByNumber", (block_number, false)).await
    }

    pub async fn base_gas_fee() -> Result<U256> {
        Self::request("eth_gasPrice", ()).await
    }

    pub async fn max_priority_fee() -> Result<U256> {
        Self::request("eth_maxPriorityFeePerGas", ()).await
    }
}
