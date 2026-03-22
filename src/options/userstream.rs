use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

static USER_DATA_STREAM: &str = "/eapi/v1/listenKey";

#[derive(Clone)]
pub struct UserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl UserStream {
    /// Get a listen key for the stream
    pub async fn start(&self) -> Result<UserDataStream> { self.client.post(USER_DATA_STREAM, None).await }

    /// Keep the connection alive, as the listen key becomes invalid after 60mn
    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(USER_DATA_STREAM, listen_key, None).await
    }

    /// Invalidate the listen key
    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client.delete(USER_DATA_STREAM, listen_key, None).await
    }
}
