use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_lightclient_ethereum::{
    checkpoint::{parse_checkpointz_output_to_config, EthereumCheckpoint},
    config::EthereumConfig,
};

pub struct EthereumCheckpointManager;

impl EthereumCheckpointManager {
    pub async fn new(config: &EthereumConfig) -> EthereumCheckpoint {
        // fetch
        let url = config.checkpoint_sync_host.clone();
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

        parse_checkpointz_output_to_config(res.body)
    }
}
