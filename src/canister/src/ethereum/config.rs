use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod};
use ic_lightclient_ethereum::{config::{parse_checkpointz_output_to_config, EthereumConfig}, parameters::EthereumParameters};

pub struct EthereumConfigManager;

impl EthereumConfigManager {
    pub async fn new(parameters: &EthereumParameters) -> EthereumConfig {
        // fetch 
        let url = parameters.checkpoint_sync_url.clone();
        let url = format!("{}/checkpointz/v1/beacon/slots", url);

        let res = http_request(CanisterHttpRequestArgument {
            url,
            max_response_bytes: Some(10_000),
            method: HttpMethod::GET,
            headers: vec![HttpHeader { name: "Accept-Encoding".into(), value: "application/json".into() }],
            body: None,
            transform: None,
        }, 200_000_000).await.unwrap().0;

        parse_checkpointz_output_to_config(res.body)
    }
}