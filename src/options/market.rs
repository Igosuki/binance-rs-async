use crate::client::*;
use crate::errors::*;
use crate::options::rest_model::*;
use crate::rest_model::PairQuery;

#[derive(Clone)]
pub struct OptionsMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl OptionsMarket {
    /// Get order book depth for a symbol
    pub async fn get_depth(&self, symbol: impl Into<String>) -> Result<OrderBook> {
        self.client
            .get_d("/eapi/v1/depth", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get recent trades for a symbol
    pub async fn get_trades(&self, symbol: impl Into<String>) -> Result<Vec<OptionTrade>> {
        self.client
            .get_d("/eapi/v1/trades", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get 24hr ticker for a symbol
    pub async fn get_ticker(&self, symbol: impl Into<String>) -> Result<OptionTicker> {
        self.client
            .get_d("/eapi/v1/ticker", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get mark price for a symbol
    pub async fn get_mark_price(&self, symbol: impl Into<String>) -> Result<Vec<OptionMarkPrice>> {
        self.client
            .get_d("/eapi/v1/mark", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get kline/candlestick data for a symbol
    pub async fn get_klines(
        &self,
        symbol: impl Into<String>,
        interval: impl Into<String>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Vec<Vec<serde_json::Value>>> {
        let query = OptionKlineQuery {
            symbol: symbol.into(),
            interval: interval.into(),
            start_time,
            end_time,
            limit,
        };
        self.client.get_d("/eapi/v1/klines", Some(query)).await
    }

    /// Get open interest for an underlying asset
    pub async fn get_open_interest(&self, underlying: impl Into<String>) -> Result<Vec<OptionOpenInterest>> {
        let query = UnderlyingQuery {
            underlying: underlying.into(),
        };
        self.client.get_d("/eapi/v1/openInterest", Some(query)).await
    }
}
