use crate::model::*;
use crate::client::*;
use crate::errors::*;

use serde_json::from_str;

#[derive(Clone)]
pub struct General {
    pub client: Client,
}

impl General {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get("/api/v3/ping", "").await?;

        Ok("pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        let data: String = self.client.get("/api/v3/time", "").await?;

        let server_time: ServerTime = from_str(data.as_str())?;

        Ok(server_time)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        let data: String = self.client.get("/api/v3/exchangeInfo", "").await?;

        let info: ExchangeInformation = from_str(data.as_str())?;

        Ok(info)
    }
}
