use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::http::HttpClient;
use alloy_primitives::U256;
use alloy_rpc_types_eth::Header;
use anyhow::{Result, anyhow};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::value::RawValue;

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

    pub async fn batch_request(&self, batch: ExecutionRequestBatch) -> Result<()> {
        let url = &self.url;
        
        let response = HttpClient::post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .json(&batch.collect_requests())
            .send()
            .await?;

        let response = response.json().await?;
        batch.process_response(response)?;

        Ok(())
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

struct InnerReceipt {
    method: String,
    params: Box<RawValue>,
    result: Rc<RefCell<Option<Box<RawValue>>>>
}

pub struct ExecutionRequestBatch {
    receipts: Vec<InnerReceipt>
}

impl ExecutionRequestBatch {
    pub fn new() -> Self {
        Self {
            receipts: vec![]
        }
    }

    fn request<T, R>(&mut self, method: &str, params: T) -> Result<ExecutionRequestReceipt<R>>
        where
            T : Serialize,
            R: DeserializeOwned
    {
        let method = method.to_string();
        let params = serde_json::to_string(&params)?;
        let params = RawValue::from_string(params)?;
        let result = Rc::new(RefCell::new(None));
        let inner_receipt = InnerReceipt { method, params, result: result.clone() };
        self.receipts.push(inner_receipt);

        Ok(ExecutionRequestReceipt { result, _r: PhantomData })
    }

    fn collect_requests(&self) -> Vec<JsonRpcRequestWrapper<Box<RawValue>>> {
        let requests = self.receipts.iter()
            .enumerate()
            .map(|(i, receipt)| {
                JsonRpcRequestWrapper {
                    id: i as u32,
                    jsonrpc: "2.0".into(),
                    method: receipt.method.clone(),
                    params: receipt.params.clone()
                }
            }).collect();

        requests
    }

    fn process_response(&self, responses: Vec<JsonRpcResponseWrapper<Box<RawValue>>>) -> Result<()> {
        for response in responses {
            let i = response.id;
            let Some(receipt) = self.receipts.get(i as usize) else {
                return Err(anyhow!("Invalid id received in response."));
            };

            receipt.result.replace(Some(response.result));
        }

        Ok(())
    }

    pub fn max_priority_fee(&mut self) -> Result<ExecutionRequestReceipt<U256>> {
        self.request("eth_maxPriorityFeePerGas", ())
    }
}

pub struct ExecutionRequestReceipt<T: DeserializeOwned> {
    result: Rc<RefCell<Option<Box<RawValue>>>>,
    _r: PhantomData<T>
}

impl<T: DeserializeOwned> ExecutionRequestReceipt<T> {
    pub fn get(&mut self) -> Result<T> {
        let Some(raw) = self.result.take() else {
            return Err(anyhow!("Result not populated."));
        };

        let parsed = serde_json::from_str(raw.get())?;
        Ok(parsed)
    }
}