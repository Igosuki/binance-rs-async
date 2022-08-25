use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::*;
use crate::rest_model::ServerTime;

#[derive(Clone)]
pub struct FuturesGeneral {
    pub client: Client,
}

impl FuturesGeneral {
    // Test connectivity
    pub async fn ping(&self) -> Result<&'static str> {
        self.client.get("/fapi/v1/ping", None).await?;
        Ok("pong")
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> { self.client.get_p("/fapi/v1/time", None).await }

    // Obtain exchange information
    // - Current exchange trading rules and symbol information
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.client.get_p("/fapi/v1/exchangeInfo", None).await
    }

    // Get Symbol information
    pub async fn get_symbol_info<S>(&self, symbol: S) -> Result<Symbol>
    where
        S: AsRef<str>,
    {
        let upper_symbol = symbol.as_ref().to_uppercase();
        self.exchange_info().await.and_then(|info| {
            info.symbols
                .into_iter()
                .find(|s| s.symbol == upper_symbol)
                .ok_or_else(|| Error::UnknownSymbol(symbol.as_ref().to_string()))
        })
    }
}
