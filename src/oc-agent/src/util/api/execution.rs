use crate::http::HttpClient;
use alloy_primitives::U256;
use alloy_rpc_types_eth::Header;
use anyhow::Result;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

#[derive(Default, Clone)]
pub struct ExecutionApi {
    url: String,
}

impl ExecutionApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    async fn request<Request: Serialize, Response: DeserializeOwned>(
        &self,
        method: &str,
        request: Request,
    ) -> Result<Response> {
        let url = &self.url;

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
    pub async fn latest_block_number(&self) -> Result<U256> {
        self.request("eth_blockNumber", ()).await
    }

    #[allow(dead_code)]
    pub async fn block_header_by_number(&self, block_number: U256) -> Result<Header> {
        self.request("eth_getBlockByNumber", (block_number, false)).await
    }

    pub async fn base_gas_fee(&self) -> Result<U256> {
        self.request("eth_gasPrice", ()).await
    }

    pub async fn max_priority_fee(&self) -> Result<U256> {
        self.request("eth_maxPriorityFeePerGas", ()).await
    }
}
