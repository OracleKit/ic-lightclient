use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_lightclient_ethereum::{
    checkpoint::{parse_checkpointz_output_to_config},
    config::{EthereumConfig, EthereumConfigPopulated}, consensus::{TConfigManager},
};

#[derive(Debug)]
pub struct EthereumConfigManager {
    config: EthereumConfigPopulated,
}

impl TConfigManager<EthereumConfigPopulated> for EthereumConfigManager {
    fn new(config: String) -> Self {
        let config: EthereumConfig = serde_json::from_str(&config).unwrap();
        Self { 
            config: config.into()
        }
    }

    async fn init(&mut self) {
        let url = self.config.checkpoint_sync_host.clone();
        let url = format!("{}/checkpointz/v1/beacon/slots", url);

        let res = http_request(
            CanisterHttpRequestArgument {
                url,
                max_response_bytes: Some(10_000),
                method: HttpMethod::GET,
                headers: vec![HttpHeader { name: "Accept-Encoding".into(), value: "application/json".into() }],
                body: None,
                transform: None,
            },
            200_000_000,
        )
        .await
        .unwrap()
        .0;

        let checkpoint = parse_checkpointz_output_to_config(res.body);
        self.config.checkpoint = Some(checkpoint);
    }
    
    fn get_config(&self) -> &EthereumConfigPopulated {
        &self.config
    }
}
