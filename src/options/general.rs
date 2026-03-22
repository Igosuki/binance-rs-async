use crate::client::*;
use crate::errors::*;
use crate::options::rest_model::*;
use crate::rest_model::ServerTime;

#[derive(Clone)]
pub struct OptionsGeneral {
    pub client: Client,
}

impl OptionsGeneral {
    pub async fn ping(&self) -> Result<String> {
        self.client.get::<()>("/eapi/v1/ping", None).await?;
        Ok("pong".into())
    }

    pub async fn get_server_time(&self) -> Result<ServerTime> { self.client.get_p("/eapi/v1/time", None).await }

    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("/eapi/v1/exchangeInfo", None).await
    }
}
