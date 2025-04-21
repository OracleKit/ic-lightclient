use reqwest::Client;
use std::sync::LazyLock;

static INNER: LazyLock<Inner> = LazyLock::new(|| {
    Inner {
        client: Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client"),
    }
});

struct Inner {
    client: Client,
}

pub struct HttpClient;

impl HttpClient {
    pub fn get(url: &str) -> reqwest::RequestBuilder {
        let client = &INNER.client;
        client.get(url)
    }

    pub fn post(url: &str) -> reqwest::RequestBuilder {
        let client = &INNER.client;
        client.post(url)
    }
}