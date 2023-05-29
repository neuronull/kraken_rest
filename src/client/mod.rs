//!
//!

use serde::{de::DeserializeOwned, Deserialize};

use super::Result;

mod endpoints;

const KRAKEN_BASE_URL: &str = "https://api.kraken.com/";

#[derive(Debug, Deserialize)]
pub struct KrakenResponse<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

impl<T> KrakenResponse<T> {
    pub fn success(&self) -> bool {
        self.error.is_empty() && self.result.is_some()
    }
}

pub struct KrakenClient {
    api_version: u8,
    base_url: String,
    http_client: reqwest::Client,
}

impl KrakenClient {
    fn new(base_url: &str, api_version: u8) -> Self {
        Self {
            api_version: api_version.to_owned(),
            base_url: base_url.to_owned(),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn new_v0() -> Self {
        KrakenClient::new(KRAKEN_BASE_URL, 0)
    }

    pub async fn get_public<T>(&self, path_query: &str) -> Result<KrakenResponse<T>>
    where
        T: DeserializeOwned,
    {
        self.http_client
            .get(format!(
                "{}/{}/public/{}",
                self.base_url, self.api_version, path_query
            ))
            .send()
            .await?
            .json::<KrakenResponse<T>>()
            .await
            .map_err(|e| e.into())
    }
}
