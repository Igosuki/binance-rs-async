use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;
use serde_json::from_str;

static USER_DATA_STREAM: &str = "/api/v3/userDataStream";

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    /// Get a listen key for the stream
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// assert!(start.unwrap().listen_key.len() > 0)
    /// ```
    pub async fn start(&self) -> Result<UserDataStream> {
        let data = self.client.post(USER_DATA_STREAM).await?;
        let user_data_stream: UserDataStream = from_str(data.as_str())?;

        Ok(user_data_stream)
    }

    /// Keep the connection alive, as the listen key becomes invalid after 60mn
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let keep_alive = tokio_test::block_on(userstream.keep_alive(&start.unwrap().listen_key));
    /// assert!(keep_alive.is_ok())
    /// ```
    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.put(USER_DATA_STREAM, listen_key).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }

    /// Invalidate the listen key
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, userstream::*, config::*};
    /// let userstream: UserStream = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(userstream.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let close = tokio_test::block_on(userstream.close(&start.unwrap().listen_key));
    /// assert!(close.is_ok())
    /// ```
    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.delete(USER_DATA_STREAM, listen_key).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }
}
