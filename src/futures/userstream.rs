use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

static LISTEN_KEY: &str = "/fapi/v1/listenKey";

#[derive(Clone)]
pub struct FuturesUserStream {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesUserStream {
    pub async fn start(&self) -> Result<UserDataStream> { self.client.post(LISTEN_KEY, None).await }

    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        self.client.put(LISTEN_KEY, listen_key, None).await
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        self.client.delete(LISTEN_KEY, listen_key, None).await
    }
}
