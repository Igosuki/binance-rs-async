use crate::client::*;
use crate::errors::*;
use crate::futures::rest_model::*;
use crate::rest_model::{
    BookTickers, KlineSummaries, KlineSummary, PairAndWindowQuery, PairQuery, SymbolPrice, Tickers,
};
use crate::util::*;
use serde_json::Value;

//TODO : Validate intervals and start/end times in history queries
//TODO : find out the repartition of kline/candlestick columns in the future kline rows
//TODO : make limit optional where applicable

#[derive(Clone)]
pub struct FuturesMarket {
    pub client: Client,
    pub recv_window: u64,
}

impl FuturesMarket {
    /// Order book (Default 100; max 1000)
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderBook>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v1/depth", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get trades for a pair
    pub async fn get_trades<S>(&self, symbol: S) -> Result<Trades>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v1/trades", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get historical trades
    pub async fn get_historical_trades<S1, S2, S3>(&self, symbol: S1, from_id: S2, limit: S3) -> Result<Trades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/fapi/v1/historicalTrades",
                Some(HistoryQuery {
                    start_time: None,
                    end_time: None,
                    from_id: from_id.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get aggregated trades
    pub async fn get_agg_trades<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        from_id: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<AggTrades>
    where
        S1: Into<String>,
        S2: Into<Option<u64>>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/fapi/v1/aggTrades",
                Some(HistoryQuery {
                    start_time: start_time.into(),
                    end_time: end_time.into(),
                    from_id: from_id.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get funding rate history
    pub async fn get_funding_rate<S1, S3, S4, S5>(
        &self,
        symbol: S1,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<FundingRate>>
    where
        S1: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        self.client
            .get_signed_p(
                "/fapi/v1/fundingRate",
                Some(HistoryQuery {
                    start_time: start_time.into(),
                    end_time: end_time.into(),
                    limit: limit.into(),
                    symbol: symbol.into(),
                    from_id: None,
                    interval: None,
                    period: None,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get open interest history
    pub async fn get_open_interest_history<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<OpenInterestHistory>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            period: Some(period.into()),
            from_id: None,
            interval: None,
        };
        query.validate()?;
        self.client
            .get_signed_p("/futures/data/openInterestHist", Some(query), self.recv_window)
            .await
    }

    /// Get Top Trader Account Long/Short Ratio
    pub async fn get_trader_account_long_short_ratio<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            period: Some(period.into()),
            from_id: None,
            interval: None,
        };
        query.validate()?;
        self.client
            .get_signed_p("/futures/data/topLongShortAccountRatio", Some(query), self.recv_window)
            .await
    }

    /// Get Top Trader Position Long/Short Ratio
    pub async fn get_trader_position_long_short_ratio<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            period: Some(period.into()),
            from_id: None,
            interval: None,
        };
        query.validate()?;
        self.client
            .get_signed_p("/futures/data/topLongShortPositionRatio", Some(query), self.recv_window)
            .await
    }

    /// Get Long/Short Ratio
    pub async fn get_long_short_ratio<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            period: Some(period.into()),
            from_id: None,
            interval: None,
        };
        query.validate()?;
        self.client
            .get_signed_p(
                "/futures/data/globalLongShortAccountRatio",
                Some(query),
                self.recv_window,
            )
            .await
    }

    /// Get Taker Long/Short Ratio
    pub async fn get_taker_long_short_ratio<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        period: S2,
        start_time: S3,
        end_time: S4,
        limit: S5,
    ) -> Result<Vec<LongShortRatio>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<Option<u64>>,
        S4: Into<Option<u64>>,
        S5: Into<u16>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            period: Some(period.into()),
            from_id: None,
            interval: None,
        };
        query.validate()?;
        self.client
            .get_signed_p("/futures/data/takerlongshortRatio", Some(query), self.recv_window)
            .await
    }

    /// Returns up to 'limit' klines for given symbol and interval ("1m", "5m", ...)
    /// https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md#klinecandlestick-data
    pub async fn get_klines<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<KlineSummaries>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<u16>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            interval: Some(interval.into()),
            from_id: None,
            period: None,
        };
        let data: Vec<Vec<Value>> = self.client.get_d("/fapi/v1/klines", Some(query)).await?;

        let klines = KlineSummaries::AllKlineSummaries(
            data.iter()
                .map(|row| KlineSummary {
                    open_time: to_i64(&row[0]),
                    open: to_f64(&row[1]),
                    high: to_f64(&row[2]),
                    low: to_f64(&row[3]),
                    close: to_f64(&row[4]),
                    volume: to_f64(&row[5]),
                    close_time: to_i64(&row[6]),
                    quote_asset_volume: to_f64(&row[7]),
                    number_of_trades: to_i64(&row[8]),
                    taker_buy_base_asset_volume: to_f64(&row[9]),
                    taker_buy_quote_asset_volume: to_f64(&row[10]),
                })
                .collect(),
        );
        Ok(klines)
    }

    /// Returns up to 'limit' blvt klines for given symbol and interval ("1m", "5m", ...)
    /// Note that the symbol is not the traditional pair but rather {symbol}{UP|DOWN}
    /// https://binance-docs.github.io/apidocs/futures/en/#blvt-nav-kline-candlestick-streams
    /// As the vector fields are undocumented on binance futures you are un your own, follow
    /// KlineSummary for an example
    pub async fn get_blvt_klines_v<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<Vec<Value>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<u16>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            interval: Some(interval.into()),
            from_id: None,
            period: None,
        };
        let klines = self.client.get_d("/fapi/v1/lvtKlines", Some(query)).await?;

        Ok(klines)
    }

    /// Returns up to 'limit' mark price klines for given symbol and interval ("1m", "5m", ...)
    /// https://binance-docs.github.io/apidocs/futures/en/#mark-price-kline-candlestick-data
    /// As the vector fields are undocumented on binance futures you are un your own, follow
    /// KlineSummary for an example
    pub async fn get_mark_price_klines_v<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<Vec<Value>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<u16>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            interval: Some(interval.into()),
            from_id: None,
            period: None,
        };
        let klines = self.client.get_d("/fapi/v1/markPriceKlines", Some(query)).await?;

        Ok(klines)
    }

    /// Returns up to 'limit' index price klines for given symbol and interval ("1m", "5m", ...)
    /// https://binance-docs.github.io/apidocs/futures/en/#index-price-kline-candlestick-data
    /// As the vector fields are undocumented on binance futures you are un your own, follow
    /// KlineSummary for an example
    pub async fn get_index_price_klines_v<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<Vec<Value>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<u16>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let query = IndexQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            pair: symbol.into(),
            interval: Some(interval.into()),
        };

        let klines = self.client.get_d("/fapi/v1/indexPriceKlines", Some(query)).await?;

        Ok(klines)
    }

    /// Returns up to 'limit' continuous contract klines for given symbol and interval ("1m", "5m", ...)
    /// https://binance-docs.github.io/apidocs/futures/en/#continuous-contract-kline-candlestick-data
    /// As the vector fields are undocumented on binance futures you are un your own, follow
    /// KlineSummary for an example
    pub async fn get_continuous_contract_klines_v<S1, S2, S3, S4, S5>(
        &self,
        symbol: S1,
        interval: S2,
        limit: S3,
        start_time: S4,
        end_time: S5,
    ) -> Result<Vec<Vec<Value>>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<u16>,
        S4: Into<Option<u64>>,
        S5: Into<Option<u64>>,
    {
        let query = HistoryQuery {
            start_time: start_time.into(),
            end_time: end_time.into(),
            limit: limit.into(),
            symbol: symbol.into(),
            interval: Some(interval.into()),
            from_id: None,
            period: None,
        };
        let klines = self.client.get_d("/fapi/v1/continuousKlines", Some(query)).await?;

        Ok(klines)
    }

    /// https://binance-docs.github.io/apidocs/futures/en/#notional-and-leverage-brackets-user_data
    pub async fn get_notional_leverage_brackets<S>(&self, symbol: S) -> Result<Vec<SymbolBrackets>>
    where
        S: Into<String>,
    {
        let p = PairAndWindowQuery {
            symbol: symbol.into(),
            recv_window: self.recv_window,
        };
        self.client
            .get_signed_p("/fapi/v1/leverageBracket", Some(p), self.recv_window)
            .await
    }

    /// https://binance-docs.github.io/apidocs/futures/en/#composite-index-symbol-information
    /// Only for composite symbols (ex: DEFIUSDT)
    pub async fn get_index_info<S>(&self, symbol: Option<S>) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        let p = symbol.map(|s| PairQuery { symbol: s.into() });
        self.client.get_d("/fapi/v1/indexInfo", p).await
    }

    /// 24hr ticker price change statistics
    pub async fn get_24h_price_stats<S>(&self, symbol: S) -> Result<PriceStats>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v1/ticker/24hr", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// 24hr ticker price change statistics for all symbols
    pub async fn get_all_24h_price_stats(&self) -> Result<Vec<PriceStats>> {
        self.client.get_p("/fapi/v1/ticker/24hr", None).await
    }

    /// Latest price for ONE symbol.
    pub async fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v2/ticker/price", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Symbols order book ticker
    /// -> Best price/qty on the order book for ALL symbols.
    pub async fn get_all_book_tickers(&self) -> Result<BookTickers> {
        self.client.get_p("/fapi/v1/ticker/bookTicker", None).await
    }

    // -> Best price/qty on the order book for ONE symbol
    pub async fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v1/ticker/bookTicker", Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    pub async fn get_mark_prices(&self, symbol: Option<String>) -> Result<Vec<MarkPrice>> {
        if let Some(symbol) = symbol {
            Ok(vec![
                self.client
                    .get_d::<MarkPrice, PairQuery>("/fapi/v1/premiumIndex", Some(PairQuery { symbol }))
                    .await?,
            ])
        } else {
            self.client.get_p("/fapi/v1/premiumIndex", None).await
        }
    }

    pub async fn get_all_liquidation_orders(&self) -> Result<LiquidationOrders> {
        self.client.get_p("/fapi/v1/allForceOrders", None).await
    }

    pub async fn open_interest<S>(&self, symbol: S) -> Result<OpenInterest>
    where
        S: Into<String>,
    {
        self.client
            .get_d("/fapi/v1/openInterest", Some(PairQuery { symbol: symbol.into() }))
            .await
    }
}
