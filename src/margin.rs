use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;
use serde_json::from_str;

static SAPI_V1_MARGIN_TRANSFER: &str = "/sapi/v1/margin/transfer";
static SAPI_V1_MARGIN_LOAN: &str = "/sapi/v1/margin/loan";
static SAPI_V1_MARGIN_REPAY: &str = "/sapi/v1/margin/repay";
static SAPI_V1_MARGIN_ORDER: &str = "/sapi/v1/margin/order";
static SAPI_V1_MARGIN_ACCOUNT: &str = "/sapi/v1/margin/account";
static SAPI_V1_MARGIN_PAIR: &str = "/sapi/v1/margin/pair";
static SAPI_V1_MARGIN_ASSET: &str = "/sapi/v1/margin/asset";
static SAPI_V1_MARGIN_ALL_ASSETS: &str = "/sapi/v1/margin/allAssets";
static SAPI_V1_MARGIN_ALL_PAIRS: &str = "/sapi/v1/margin/allPairs";
static SAPI_V1_MARGIN_PRICE_INDEX: &str = "/sapi/v1/margin/priceIndex";
static SAPI_V1_MARGIN_INTEREST_HISTORY: &str = "/sapi/v1/margin/interestHistory";
static SAPI_V1_MARGIN_FORCED_LIQUIDATION_RECORD: &str = "/sapi/v1/margin/forcedLiquidationRec";
static SAPI_V1_MARGIN_OPEN_ORDERS: &str = "/sapi/v1/margin/openOrders";
static SAPI_V1_MARGIN_ALL_ORDERS: &str = "/sapi/v1/margin/allOrders";
static SAPI_V1_MARGIN_MY_TRADES: &str = "/sapi/v1/margin/myTrades";
static SAPI_V1_MARGIN_MAX_BORROWABLE: &str = "/sapi/v1/margin/maxBorrowable";
static SAPI_V1_MARGIN_MAX_TRANSFERABLE: &str = "/sapi/v1/margin/maxTransferable";
static SAPI_USER_DATA_STREAM: &str = "/sapi/v1/userDataStream";

#[derive(Clone)]
pub struct Margin {
    pub client: Client,
    pub recv_window: u64,
}

impl Margin {
    /// Execute transfer between spot account and margin account.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.transfer("BTCUSDT", 0.001, MarginTransferType::FromMainToMargin));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn transfer<S, F>(&self, symbol: S, qty: F, transfer_type: MarginTransferType) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let transfer: Transfer = Transfer {
            asset: symbol.into(),
            amount: qty.into(),
            transfer_type,
        };
        self.client
            .post_signed_p(SAPI_V1_MARGIN_TRANSFER, transfer, self.recv_window)
            .await
    }

    /// Apply for a loan.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.loan("BTCUSDT", 0.001));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn loan<S, F>(&self, symbol: S, qty: F) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let loan: Loan = Loan {
            asset: symbol.into(),
            amount: qty.into(),
        };
        self.client
            .post_signed_p(SAPI_V1_MARGIN_LOAN, loan, self.recv_window)
            .await
    }

    /// Repay loan for margin account.
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.repay("BTCUSDT", 0.001));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn repay<S, F>(&self, symbol: S, qty: F) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let loan: Loan = Loan {
            asset: symbol.into(),
            amount: qty.into(),
        };
        self.client
            .post_signed_p(SAPI_V1_MARGIN_REPAY, loan, self.recv_window)
            .await
    }

    /// Post a new order for margin account.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let margin_order = MarginOrder {
    ///     symbol: "BTCUSDT".to_string(),
    ///     side: OrderSide::Sell,
    ///     order_type: OrderType::Limit,
    ///     quantity: 0.001,
    ///     price: 10.0,
    ///     stop_price: 10.0,
    ///     new_client_order_id: "my_id".to_string(),
    ///     iceberg_qty: 10.0,
    ///     new_order_resp_type: OrderResponse::Ack,
    ///     time_in_force: TimeInForce::FOK
    /// };
    /// let transaction_id = tokio_test::block_on(margin.trade(margin_order));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn trade(&self, margin_order: MarginOrder) -> Result<TransactionId> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_ORDER, margin_order, self.recv_window)
            .await
    }

    /// Cancel an existing order
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.cancel_trade("BTCUSDT", 1_u64, "my_id".to_string(), "my_next_id".to_string()));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn cancel_trade<S, F>(
        &self,
        symbol: S,
        order_id: F,
        orig_client_order_id: String,
        new_client_order_id: String,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<u64>,
    {
        let margin_order_cancellation: MarginOrderCancellation = MarginOrderCancellation {
            symbol: symbol.into(),
            order_id: order_id.into(),
            orig_client_order_id,
            new_client_order_id,
        };
        self.client
            .delete_signed_p(SAPI_V1_MARGIN_REPAY, margin_order_cancellation, self.recv_window)
            .await
    }

    /// Get existing loan records
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let loan_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.loans(loan_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn loans(&self, loan_query: RecordsQuery) -> Result<RecordsQueryResult<LoanState>> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_LOAN, loan_query, self.recv_window)
            .await
    }

    /// Get existing repay records history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.repays(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn repays(&self, repays_query: RecordsQuery) -> Result<RecordsQueryResult<RepayState>> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_REPAY, repays_query, self.recv_window)
            .await
    }

    /// Get margin account details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let account_details = tokio_test::block_on(margin.details());
    /// assert!(account_details.is_ok(), "{:?}", account_details);
    /// ```
    pub async fn details(&self) -> Result<MarginAccountDetails> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ACCOUNT, q).await
    }

    /// Get asset details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let asset_detail = tokio_test::block_on(margin.asset("BTC"));
    /// assert!(asset_detail.is_ok(), "{:?}", asset_detail);
    /// ```
    pub async fn asset<S>(&self, asset: S) -> Result<AssetDetails>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ASSET, Some(AssetQuery { asset: asset.into() }))
            .await
    }

    /// Get margin pair market data
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let pair_details = tokio_test::block_on(margin.pair("BTCUSDT"));
    /// assert!(pair_details.is_ok(), "{:?}", pair_details);
    /// ```
    pub async fn pair<S>(&self, symbol: S) -> Result<PairDetails>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_PAIR, Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get all assets details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_assets = tokio_test::block_on(margin.all_assets());
    /// assert!(all_assets.is_ok(), "{:?}", all_assets);
    /// ```
    pub async fn all_assets(&self) -> Result<AllAssets> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ALL_ASSETS, q).await
    }

    /// Get all pair details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_pairs = tokio_test::block_on(margin.all_pairs());
    /// assert!(all_pairs.is_ok(), "{:?}", all_pairs);
    /// ```
    pub async fn all_pairs(&self) -> Result<AllPairs> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ALL_PAIRS, q).await
    }

    /// Get price index
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let price_index = tokio_test::block_on(margin.price_index("BTCUSDT"));
    /// assert!(price_index.is_ok(), "{:?}", price_index);
    /// ```
    pub async fn price_index<S>(&self, symbol: S) -> Result<PriceIndex>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_PRICE_INDEX, Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get transfer history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.transfers(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn transfers(&self, transfers_query: RecordsQuery) -> Result<RecordsQueryResult<OrderState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_TRANSFER, Some(transfers_query))
            .await
    }

    /// Get interest history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.interests(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn interests(&self, interest_query: RecordsQuery) -> Result<RecordsQueryResult<InterestState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_INTEREST_HISTORY, Some(interest_query))
            .await
    }

    /// Get forced liquidation history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.forced_liquidations(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn forced_liquidations(
        &self,
        forced_liquidations_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<ForcedLiquidationState>> {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_FORCED_LIQUIDATION_RECORD,
                Some(forced_liquidations_query),
            )
            .await
    }

    /// Get an existing order state
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = MarginOrderQuery {
    ///     symbol: "BTCUSDT".to_string(),
    ///     order_id: "1".to_string(),
    ///     orig_client_order_id: "my_id".to_string(),
    /// };
    /// let records = tokio_test::block_on(margin.order(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn order(&self, margin_order: MarginOrderQuery) -> Result<MarginOrderState> {
        self.client.get_signed_p(SAPI_V1_MARGIN_ORDER, Some(margin_order)).await
    }

    /// Get open orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let order_state = tokio_test::block_on(margin.open_orders("BTCUSDT"));
    /// assert!(order_state.is_ok(), "{:?}", order_state);
    /// ```
    pub async fn open_orders<S>(&self, symbol: S) -> Result<MarginOrderState>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_OPEN_ORDERS, Some(PairQuery { symbol: symbol.into() }))
            .await
    }

    /// Get all orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.orders(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn orders(&self, all_orders_query: RecordsQuery) -> Result<RecordsQueryResult<OrderSumaryState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_ORDERS, Some(all_orders_query))
            .await
    }

    /// Get all trades
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    tx_id: None,
    ///    start_time: None,
    ///    end_time: None,
    ///    current: None,
    ///    size: None,
    ///    transfer_type: Some(TransferType::RollIn)
    /// };
    /// let records = tokio_test::block_on(margin.trades(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn trades(&self, all_orders_query: RecordsQuery) -> Result<RecordsQueryResult<OwnTradesState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_MY_TRADES, Some(all_orders_query))
            .await
    }

    /// Get max borrowable
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let max = tokio_test::block_on(margin.max_borrowable("BTC"));
    /// assert!(max.is_ok(), "{:?}", max);
    /// ```
    pub async fn max_borrowable<S>(&self, asset: S) -> Result<MaxAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_MAX_BORROWABLE, Some(AssetQuery { asset: asset.into() }))
            .await
    }

    /// Get max transferable
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let max = tokio_test::block_on(margin.max_transferable("BTC"));
    /// assert!(max.is_ok(), "{:?}", max);
    /// ```
    pub async fn max_transferable<S>(&self, asset: S) -> Result<MaxAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_MAX_TRANSFERABLE,
                Some(AssetQuery { asset: asset.into() }),
            )
            .await
    }

    /// Start user data stream
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(margin.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// assert!(start.unwrap().listen_key.len() > 0)
    /// ```
    pub async fn start(&self) -> Result<UserDataStream> {
        let data = self.client.post(SAPI_USER_DATA_STREAM).await?;
        let user_data_stream: UserDataStream = from_str(data.as_str())?;

        Ok(user_data_stream)
    }

    /// Current open orders on a symbol
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(margin.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let keep_alive = tokio_test::block_on(margin.keep_alive(&start.unwrap().listen_key));
    /// assert!(keep_alive.is_ok())
    /// ```
    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.put(SAPI_USER_DATA_STREAM, listen_key).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }

    /// Close the user stream
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(margin.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let close = tokio_test::block_on(margin.close(&start.unwrap().listen_key));
    /// assert!(close.is_ok())
    /// ```
    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.delete(SAPI_USER_DATA_STREAM, listen_key).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }
}
