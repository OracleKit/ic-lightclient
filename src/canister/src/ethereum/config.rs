use crate::chain::ConfigManager;
use anyhow::{anyhow, Ok, Result};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_lightclient_ethereum::{
    checkpoint::parse_checkpointz_output_to_config,
    config::{EthereumConfig, EthereumConfigPopulated},
};

pub struct EthereumConfigManager;

impl ConfigManager for EthereumConfigManager {
    type Config = EthereumConfigPopulated;

    async fn process(config: String) -> Result<Self::Config> {
        let config: EthereumConfig = serde_json::from_str(&config)?;
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
        .map_err(|e| anyhow!("HTTP Request failed: {:?} {}", e.0, e.1))?
        .0;

        let checkpoint = parse_checkpointz_output_to_config(res.body)?;
        let populated_config = config.populate(checkpoint);

        Ok(populated_config)
    }
}
