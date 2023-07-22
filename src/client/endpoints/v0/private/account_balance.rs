//!
//!

use std::collections::HashMap;

use crate::{
    client::{KrakenClient, KrakenResponse},
    Result,
};

const BALANCE_PATH: &str = "Balance";

pub type AccountBalanceResponse = HashMap<String, String>;

impl KrakenClient {
    pub async fn get_account_balance(&self) -> Result<KrakenResponse<AccountBalanceResponse>> {
        self.get_private::<AccountBalanceResponse>(BALANCE_PATH)
            .await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_private_get_account_balance() {
        let client = super::KrakenClient::new_v0();

        let response = client.get_account_balance().await;

        println!("{:#?}", response);

        assert!(response.is_ok());
        let response = response.unwrap();

        println!("{:#?}", response.result);

        assert!(response.success());

        let _balance = response.result.as_ref().unwrap();
        println!("{:#?}", _balance);
    }
}
