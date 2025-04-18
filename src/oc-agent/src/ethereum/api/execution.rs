use std::sync::OnceLock;
use alloy_primitives::U256;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::http::HttpClient;
use alloy_rpc_types_eth::Header;

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
    id: u32,
    jsonrpc: String,
    result: T,
}

pub struct ExecutionApi;

impl ExecutionApi {
    pub fn init(url: String) {
        INNER.set(Inner { url }).unwrap();
    }

    async fn request<Request: Serialize, Response: DeserializeOwned>(
        method: &str,
        request: Request,
    ) -> Response {
        let inner = INNER.get().unwrap();
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
            .await
            .expect("Failed to send request");

        let response: JsonRpcResponseWrapper<Response> = response
            .json()
            .await
            .expect("Failed to parse response");

        response.result
    }

    pub async fn latest_block_number() -> U256 {
        Self::request("eth_blockNumber", ()).await
    }

    pub async fn get_block_header_by_number(
        block_number: U256,
    ) -> Header {
        Self::request(
            "eth_getBlockByNumber",
            (block_number, false),
        )
        .await
    }
}