//!
//!

use reqwest::header;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{error::Error, Result};

mod endpoints;
mod signature;

use self::signature::{get_signature, nonce};

const KRAKEN_BASE_URL: &str = "https://api.kraken.com/";
const FORM_URL_ENCODED: &str = "application/x-www-form-urlencoded; charset=utf-8";
const HEADER_API_KEY: &str = "API-Key";
const HEADER_API_SIGN: &str = "API-Sign";
const ENV_VAR_API_KEY: &str = "KRAKEN_API_KEY";
const ENV_VAR_API_SECRET: &str = "KRAKEN_API_SECRET";

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
    api_key: Option<String>,
    api_secret: Option<String>,
}

impl KrakenClient {
    fn new(
        base_url: &str,
        api_version: u8,
        api_key: Option<String>,
        api_secret: Option<String>,
    ) -> Self {
        Self {
            api_version: api_version.to_owned(),
            base_url: base_url.to_owned(),
            http_client: reqwest::Client::new(),
            api_key,
            api_secret,
        }
    }

    fn new_with_secret(api_version: u8) -> Self {
        let api_key = std::env::var(ENV_VAR_API_KEY).ok();
        let api_secret = std::env::var(ENV_VAR_API_SECRET).ok();
        KrakenClient::new(KRAKEN_BASE_URL, api_version, api_key, api_secret)
    }

    pub fn new_v0() -> Self {
        KrakenClient::new_with_secret(0)
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

    pub async fn get_private<T>(&self, path_query: &str) -> Result<KrakenResponse<T>>
    where
        T: DeserializeOwned,
    {
        if let (Some(api_key), Some(api_secret)) = (&self.api_key, &self.api_secret) {
            let nonce = nonce()?;

            let data = format!("nonce={nonce}");

            let uri = format!("/{}/private/{}", self.api_version, path_query);

            self.http_client
                .post(format!("{}{}", self.base_url, &uri))
                .header(HEADER_API_KEY, api_key)
                .header(
                    HEADER_API_SIGN,
                    get_signature(&uri, &data, nonce, api_secret)?,
                )
                .header(header::CONTENT_TYPE, FORM_URL_ENCODED)
                .body(data.into_bytes())
                .send()
                .await?
                .json::<KrakenResponse<T>>()
                .await
                .map_err(|e| e.into())
        } else {
            Err(Error::Unauthorized)
        }
    }
}
