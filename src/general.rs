use serde_json::Value;

use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    /// Test connectivity
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let conf = Config::default().set_rest_api_endpoint(DATA_REST_ENDPOINT);
    /// let general: General = Binance::new_with_env(&conf);
    /// let pong = tokio_test::block_on(general.ping());
    /// assert!(pong.is_ok(), "{:?}", pong);
    /// assert_eq!(pong.unwrap(), "pong");
    /// ```
    pub async fn ping(&self) -> Result<&'static str> {
        let _: Value = self.client.get("/api/v3/ping", None).await?;

        Ok("pong")
    }

    /// Check server time
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let conf = Config::default().set_rest_api_endpoint(DATA_REST_ENDPOINT);
    /// let general: General = Binance::new_with_env(&conf);
    /// let server_time = tokio_test::block_on(general.get_server_time());
    /// assert!(server_time.is_ok(), "{:?}", server_time);
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.client.get("/api/v3/time", None).await
    }

    /// Obtain exchange information (rate limits, symbol metadata etc)
    /// # Examples
    /// ```rust
    /// use binance::{api::*, general::*, config::*};
    /// let conf = Config::default().set_rest_api_endpoint(DATA_REST_ENDPOINT);
    /// let general: General = Binance::new_with_env(&conf);
    /// let exchange_info = tokio_test::block_on(general.exchange_info());
    /// assert!(exchange_info.is_ok(), "{:?}", exchange_info);
    /// ```
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get("/api/v3/exchangeInfo", None).await
    }
}
