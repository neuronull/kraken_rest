//!
//!

use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    client::{KrakenClient, KrakenResponse},
    Result,
};

const ASSETS_PATH: &str = "Assets";

#[derive(Debug, Deserialize)]
pub struct AssetInfo {
    pub aclass: String,
    pub altname: String,
    pub decimals: f32,
    pub display_decimals: f32,
    pub collateral_value: Option<f32>,
    pub status: String,
}

pub type AssetInfoResponse = HashMap<String, AssetInfo>;

impl KrakenClient {
    pub async fn get_all_asset_info(&self) -> Result<KrakenResponse<AssetInfoResponse>> {
        self.get_public::<AssetInfoResponse>(ASSETS_PATH).await
    }

    pub async fn get_asset_info(
        &self,
        assets: &[&str],
    ) -> Result<KrakenResponse<AssetInfoResponse>> {
        self.get_public::<AssetInfoResponse>(&format!("{}?asset={}", ASSETS_PATH, assets.join(",")))
            .await
    }

    pub async fn get_asset_class_info(
        &self,
        class: &str,
    ) -> Result<KrakenResponse<AssetInfoResponse>> {
        self.get_public::<AssetInfoResponse>(&format!("{}?aclass={}", ASSETS_PATH, class))
            .await
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_public_get_all_asset_info() {
        let client = super::KrakenClient::new_v0();

        let response = client.get_all_asset_info().await;

        assert!(response.is_ok());
        let response = response.unwrap();

        assert!(response.success());

        let assets = response.result.as_ref().unwrap();
        assert!(assets.len() > 1);
    }

    #[tokio::test]
    async fn test_public_get_single_asset_info() {
        let client = super::KrakenClient::new_v0();

        let response = client.get_asset_info(&vec!["ETH"]).await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.success());

        let assets = response.result.as_ref().unwrap();
        assert!(assets.len() == 1);
        assert!(assets.contains_key("XETH"));
    }

    #[tokio::test]
    async fn test_public_get_asset_class_info() {
        let client = super::KrakenClient::new_v0();

        let response = client.get_asset_class_info("currency").await;

        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(response.success());

        let assets = response.result.as_ref().unwrap();
        assert!(assets.len() > 1);
    }
}
