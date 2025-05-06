use ic_cdk::api::{canister_balance128, time};
use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: Vec<u8>,
}

pub fn serve_metrics() -> HttpResponse {
    let timestamp = time() / 1000000;

    let body = vec![
        "# HELP cycle_balance Current cycle balance of canister".to_string(),
        "# TYPE cycle_balance counter".to_string(),
        format!(
            "cycle_balance {} {}",
            canister_balance128(),
            timestamp
        )
    ].join("\n");

    let body = body.as_bytes().to_vec();

    HttpResponse {
        status_code: 200,
        headers: vec![
            HeaderField("Content-Length".to_string(), format!("{}", body.len())),
            HeaderField("Cache-Control".to_string(), format!("max-age={}", 300)),
            HeaderField("Content-Type".to_string(), "text/plain".to_string())
        ],
        body: body
    }
}