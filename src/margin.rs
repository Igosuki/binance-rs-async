use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;
use crate::util::bool_to_string;

static SAPI_V1_MARGIN_TRANSFER: &str = "/sapi/v1/margin/transfer";
static SAPI_V1_MARGIN_ISOLATED_TRANSFER: &str = "/sapi/v1/margin/isolated/transfer";
static SAPI_V1_MARGIN_LOAN: &str = "/sapi/v1/margin/loan";
static SAPI_V1_MARGIN_REPAY: &str = "/sapi/v1/margin/repay";
static SAPI_V1_MARGIN_ORDER: &str = "/sapi/v1/margin/order";
static SAPI_V1_MARGIN_OCO_ORDER: &str = "/sapi/v1/margin/order/oco";
static SAPI_V1_MARGIN_OCO_ORDER_LIST: &str = "/sapi/v1/margin/orderList";
static SAPI_V1_MARGIN_OCO_ALL_ORDER_LIST: &str = "/sapi/v1/margin/allOrderList";
static SAPI_V1_MARGIN_OCO_OPEN_ORDER_LIST: &str = "/sapi/v1/margin/openOrderList";
static SAPI_V1_MARGIN_ACCOUNT: &str = "/sapi/v1/margin/account";
static SAPI_V1_MARGIN_ISOLATED_ACCOUNT: &str = "/sapi/v1/margin/isolated/account";
static SAPI_V1_MARGIN_PAIR: &str = "/sapi/v1/margin/pair";
static SAPI_V1_MARGIN_ISOLATED_PAIR: &str = "/sapi/v1/margin/isolated/pair";
static SAPI_V1_MARGIN_ASSET: &str = "/sapi/v1/margin/asset";
static SAPI_V1_MARGIN_ALL_ASSETS: &str = "/sapi/v1/margin/allAssets";
static SAPI_V1_MARGIN_ALL_PAIRS: &str = "/sapi/v1/margin/allPairs";
static SAPI_V1_MARGIN_ALL_ISOLATED_PAIRS: &str = "/sapi/v1/margin/isolated/allPairs";
static SAPI_V1_MARGIN_ISOLATED_ACCOUNT_LIMIT: &str = "/sapi/v1/margin/isolated/accountLimit";
static SAPI_V1_MARGIN_PRICE_INDEX: &str = "/sapi/v1/margin/priceIndex";
static SAPI_V1_MARGIN_INTEREST_HISTORY: &str = "/sapi/v1/margin/interestHistory";
static SAPI_V1_MARGIN_FORCED_LIQUIDATION_RECORD: &str = "/sapi/v1/margin/forceLiquidationRec";
static SAPI_V1_MARGIN_OPEN_ORDERS: &str = "/sapi/v1/margin/openOrders";
static SAPI_V1_MARGIN_ALL_ORDERS: &str = "/sapi/v1/margin/allOrders";
static SAPI_V1_MARGIN_MY_TRADES: &str = "/sapi/v1/margin/myTrades";
static SAPI_V1_MARGIN_MAX_BORROWABLE: &str = "/sapi/v1/margin/maxBorrowable";
static SAPI_V1_MARGIN_MAX_TRANSFERABLE: &str = "/sapi/v1/margin/maxTransferable";
static SAPI_USER_DATA_STREAM: &str = "/sapi/v1/userDataStream";
static SAPI_USER_DATA_STREAM_ISOLATED: &str = "/sapi/v1/userDataStream/isolated";
static SAPI_V1_BNB_BURN: &str = "/sapi/v1/bnbBurn";
static SAPI_V1_MARGIN_INTEREST_RATE_HISTORY: &str = "/sapi/v1/margin/interestRateHistory";

/// This struct acts as a gateway for all margin endpoints.
/// Preferably use the trait [`crate::api::Binance`] to get an instance.
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

    /// Execute transfer between spot account and isolated margin account.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.isolated_transfer("BTC", "BTC", 0.001, IsolatedMarginTransferType::Spot, IsolatedMarginTransferType::IsolatedMargin));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn isolated_transfer<S, F>(
        &self,
        asset_symbol: S,
        symbol: S,
        qty: F,
        from: IsolatedMarginTransferType,
        to: IsolatedMarginTransferType,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let transfer = IsolatedTransfer {
            asset: asset_symbol.into(),
            symbol: symbol.into(),
            amount: qty.into(),
            trans_from: from,
            trans_to: to,
        };
        self.client
            .post_signed_p(SAPI_V1_MARGIN_ISOLATED_TRANSFER, transfer, self.recv_window)
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
        self.loan_with_isolation(symbol, qty, None, None).await
    }

    /// Apply for an isolated loan.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.loan_with_isolation("BTCUSDT", 0.001, Some(true), Some("BNB".to_string())));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn loan_with_isolation<S, F>(
        &self,
        symbol: S,
        qty: F,
        is_isolated: Option<bool>,
        isolated_asset: Option<String>,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let loan: Loan = Loan {
            asset: symbol.into(),
            amount: qty.into(),
            is_isolated: is_isolated.map(bool_to_string),
            symbol: isolated_asset,
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
        self.repay_with_isolation(symbol, qty, None, None).await
    }

    /// Apply for an isolated loan.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let transaction_id = tokio_test::block_on(margin.repay_with_isolation("BTCUSDT", 0.001, Some(true), Some("BNB".to_string())));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn repay_with_isolation<S, F>(
        &self,
        symbol: S,
        qty: F,
        is_isolated: Option<bool>,
        isolated_asset: Option<String>,
    ) -> Result<TransactionId>
    where
        S: Into<String>,
        F: Into<f64>,
    {
        let loan: Loan = Loan {
            asset: symbol.into(),
            amount: qty.into(),
            is_isolated: is_isolated.map(bool_to_string),
            symbol: isolated_asset,
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
    ///     quantity: Some(0.001),
    ///     quote_order_qty: None,
    ///     price: Some(10.0),
    ///     stop_price: Some(10.0),
    ///     new_client_order_id: Some("my_id".to_string()),
    ///     iceberg_qty: Some(10.0),
    ///     new_order_resp_type: OrderResponse::Ack,
    ///     time_in_force: Some(TimeInForce::FOK),
    ///     side_effect_type: SideEffectType::NoSideEffect,
    ///     is_isolated: None,
    /// };
    /// let transaction_id = tokio_test::block_on(margin.trade(margin_order));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn trade(&self, margin_order: MarginOrder) -> Result<MarginOrderResult> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_ORDER, margin_order, self.recv_window)
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
    ///     quantity: Some(0.001),
    ///     quote_order_qty: None,
    ///     price: Some(10.0),
    ///     stop_price: Some(10.0),
    ///     new_client_order_id: Some("my_id".to_string()),
    ///     iceberg_qty: Some(10.0),
    ///     new_order_resp_type: OrderResponse::Ack,
    ///     time_in_force: Some(TimeInForce::FOK),
    ///     side_effect_type: SideEffectType::NoSideEffect,
    ///     is_isolated: None,
    /// };
    /// let transaction_id = tokio_test::block_on(margin.new_order(margin_order));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn new_order(&self, margin_order: MarginOrder) -> Result<MarginOrderResult> {
        self.trade(margin_order).await
    }

    /// Post a new order for margin account.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let margin_order = MarginOCOOrder {
    ///     symbol: "BTCUSDT".to_string(),
    ///     side: OrderSide::Sell,
    ///     quantity: 10.0,
    ///     price: 10.0,
    ///     stop_price: 1.0,
    ///     ..MarginOCOOrder::default()
    /// };
    /// let transaction_id = tokio_test::block_on(margin.new_oco_order(margin_order));
    /// assert!(transaction_id.is_ok(), "{:?}", transaction_id);
    /// ```
    pub async fn new_oco_order(&self, margin_order: MarginOCOOrder) -> Result<MarginOCOOrderResult> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_OCO_ORDER, margin_order, self.recv_window)
            .await
    }

    /// Cancel an existing order
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let result = tokio_test::block_on(margin.cancel_trade("BTCUSDT", 1_u64, "my_id".to_string(), "my_next_id".to_string(), None));
    /// assert!(result.is_ok(), "{:?}", result);
    /// ```
    pub async fn cancel_trade<S, F>(
        &self,
        symbol: S,
        order_id: F,
        orig_client_order_id: String,
        new_client_order_id: String,
        is_isolated: Option<bool>,
    ) -> Result<MarginOrderCancellationResult>
    where
        S: Into<String>,
        F: Into<u64>,
    {
        let margin_order_cancellation: MarginOrderCancellation = MarginOrderCancellation {
            symbol: symbol.into(),
            order_id: order_id.into(),
            orig_client_order_id,
            new_client_order_id,
            is_isolated: is_isolated.map(bool_to_string),
        };
        self.client
            .delete_signed_p(SAPI_V1_MARGIN_ORDER, margin_order_cancellation, self.recv_window)
            .await
    }

    /// Cancel an existing order
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let result = tokio_test::block_on(margin.cancel_oco_order("BTCUSDT", 1_u64, "my_id".to_string(), "my_next_id".to_string(), None));
    /// assert!(result.is_ok(), "{:?}", result);
    /// ```
    pub async fn cancel_oco_order<S, F>(
        &self,
        symbol: S,
        order_list_id: F,
        list_client_order_id: String,
        new_client_order_id: String,
        is_isolated: Option<bool>,
    ) -> Result<MarginOCOOrderResult>
    where
        S: Into<String>,
        F: Into<u64>,
    {
        let margin_order_cancellation: MarginOCOOrderCancellation = MarginOCOOrderCancellation {
            symbol: symbol.into(),
            order_list_id: order_list_id.into(),
            list_client_order_id,
            new_client_order_id,
            is_isolated: is_isolated.map(bool_to_string),
        };
        self.client
            .delete_signed_p(SAPI_V1_MARGIN_OCO_ORDER, margin_order_cancellation, self.recv_window)
            .await
    }

    /// Cancel all existing orders for a symbol
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let result = tokio_test::block_on(margin.cancel_all_orders("BTCUSDT", None));
    /// assert!(result.is_ok(), "{:?}", result);
    /// ```
    pub async fn cancel_all_orders<S>(
        &self,
        symbol: S,
        is_isolated: Option<bool>,
    ) -> Result<MarginOrdersCancellationResult>
    where
        S: Into<String>,
    {
        let margin_orders_cancellation: MarginOrdersCancellation = MarginOrdersCancellation {
            symbol: symbol.into(),
            is_isolated: is_isolated.map(bool_to_string),
        };
        self.client
            .delete_signed_p(SAPI_V1_MARGIN_OPEN_ORDERS, margin_orders_cancellation, self.recv_window)
            .await
    }

    /// Get existing loan records
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let loan_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    transfer_type: Some(TransferType::RollIn),
    ///    ..RecordsQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.loans(loan_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn loans(&self, loan_query: RecordsQuery) -> Result<RecordsQueryResult<LoanState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_LOAN, Some(loan_query), self.recv_window)
            .await
    }

    /// Get existing repay records history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    transfer_type: Some(TransferType::RollIn),
    ///    ..RecordsQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.repays(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn repays(&self, repays_query: RecordsQuery) -> Result<RecordsQueryResult<RepayState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_REPAY, Some(repays_query), self.recv_window)
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
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ACCOUNT, q, self.recv_window)
            .await
    }

    /// Get isolated margin account details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let account_details = tokio_test::block_on(margin.isolated_details(None));
    /// assert!(account_details.is_ok(), "{:?}", account_details);
    /// ```
    pub async fn isolated_details(&self, symbols: Option<Vec<String>>) -> Result<IsolatedMarginAccountDetails> {
        let q: Option<IsolatedMarginPairQuery> = symbols.map(|s| IsolatedMarginPairQuery { symbols: s.join(",") });
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ISOLATED_ACCOUNT, q, self.recv_window)
            .await
    }

    /// Disable isolated margin account
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let account_details = tokio_test::block_on(margin.disable_isolated("BTCUSDT".to_string()));
    /// assert!(account_details.is_ok(), "{:?}", account_details);
    /// ```
    pub async fn disable_isolated(&self, symbol: String) -> Result<IsolatedMarginAccountDetails> {
        let q: Option<PairQuery> = Some(PairQuery { symbol });
        self.client
            .delete_signed_p(SAPI_V1_MARGIN_ISOLATED_ACCOUNT, q, self.recv_window)
            .await
    }

    /// Enable isolated margin account
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let account_details = tokio_test::block_on(margin.enable_isolated("BTCUSDT".to_string()));
    /// assert!(account_details.is_ok(), "{:?}", account_details);
    /// ```
    pub async fn enable_isolated(&self, symbol: String) -> Result<IsolatedMarginAccountDetails> {
        let q: Option<PairQuery> = Some(PairQuery { symbol });
        self.client
            .post_signed_p(SAPI_V1_MARGIN_ISOLATED_ACCOUNT, q, self.recv_window)
            .await
    }

    /// Get margin pair market data
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let pair_details = tokio_test::block_on(margin.isolated_pair("BTCUSDT"));
    /// assert!(pair_details.is_ok(), "{:?}", pair_details);
    /// ```
    pub async fn isolated_pair<S>(&self, symbol: S) -> Result<IsolatedPairDetails>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_ISOLATED_PAIR,
                Some(PairQuery { symbol: symbol.into() }),
                self.recv_window,
            )
            .await
    }

    /// Get all isolated pair details
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_pairs = tokio_test::block_on(margin.all_pairs());
    /// assert!(all_pairs.is_ok(), "{:?}", all_pairs);
    /// ```
    pub async fn all_isolated_pairs(&self) -> Result<AllIsolatedPairs> {
        let q: Option<PairQuery> = None;
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_ISOLATED_PAIRS, q, self.recv_window)
            .await
    }

    /// Toggle BNB Burn on Spot Trade and Margin Interest
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_pairs = tokio_test::block_on(margin.toggle_bnb_burn(BnbBurnQuery::default()));
    /// assert!(all_pairs.is_ok(), "{:?}", all_pairs);
    /// ```
    pub async fn toggle_bnb_burn(&self, query: BnbBurnQuery) -> Result<BnbBurnStatus> {
        self.client
            .post_signed_p(SAPI_V1_BNB_BURN, query, self.recv_window)
            .await
    }

    /// Query BNB Burn on Spot Trade and Margin Interest
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_pairs = tokio_test::block_on(margin.bnb_burn_status());
    /// assert!(all_pairs.is_ok(), "{:?}", all_pairs);
    /// ```
    pub async fn bnb_burn_status(&self) -> Result<BnbBurnStatus> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_BNB_BURN, q, self.recv_window).await
    }

    /// Query Interest rate history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let all_pairs = tokio_test::block_on(margin.interest_rate_history(InterestRateHistoryQuery::default()));
    /// assert!(all_pairs.is_ok(), "{:?}", all_pairs);
    /// ```
    pub async fn interest_rate_history(&self, q: InterestRateHistoryQuery) -> Result<InterestRateHistory> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_INTEREST_RATE_HISTORY, Some(q), self.recv_window)
            .await
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
            .get_signed_p(
                SAPI_V1_MARGIN_ASSET,
                Some(AssetQuery { asset: asset.into() }),
                self.recv_window,
            )
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
            .get_signed_p(
                SAPI_V1_MARGIN_PAIR,
                Some(PairQuery { symbol: symbol.into() }),
                self.recv_window,
            )
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
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_ASSETS, q, self.recv_window)
            .await
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
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_PAIRS, q, self.recv_window)
            .await
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
            .get_signed_p(
                SAPI_V1_MARGIN_PRICE_INDEX,
                Some(PairQuery { symbol: symbol.into() }),
                self.recv_window,
            )
            .await
    }

    /// Get transfer history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    transfer_type: Some(TransferType::RollIn),
    ///    ..RecordsQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.transfers(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn transfers(&self, transfers_query: RecordsQuery) -> Result<RecordsQueryResult<OrderState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_TRANSFER, Some(transfers_query), self.recv_window)
            .await
    }

    /// Get isolated transfer history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = IsolatedTransfersQuery {
    ///    symbol: "BTC".to_string(),
    ///    ..IsolatedTransfersQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.isolated_transfers(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn isolated_transfers(
        &self,
        transfers_query: IsolatedTransfersQuery,
    ) -> Result<RecordsQueryResult<OrderState>> {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_ISOLATED_TRANSFER,
                Some(transfers_query),
                self.recv_window,
            )
            .await
    }

    /// Get interest history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    transfer_type: Some(TransferType::RollIn),
    ///    ..RecordsQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.interests(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn interests(&self, interest_query: RecordsQuery) -> Result<RecordsQueryResult<InterestState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_INTEREST_HISTORY, Some(interest_query), self.recv_window)
            .await
    }

    /// Get forced liquidation history
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = RecordsQuery {
    ///    asset: "BTC".to_string(),
    ///    transfer_type: Some(TransferType::RollIn),
    ///    ..RecordsQuery::default()
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
                self.recv_window,
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
    ///     order_id: Some("1".to_string()),
    ///     orig_client_order_id: Some("my_id".to_string()),
    ///     is_isolated: Some("FALSE".to_string()),
    /// };
    /// let records = tokio_test::block_on(margin.order(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn order(&self, margin_order: MarginOrderQuery) -> Result<MarginOrderState> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ORDER, Some(margin_order), self.recv_window)
            .await
    }

    /// Get open orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let order_state = tokio_test::block_on(margin.open_orders("BTCUSDT", None));
    /// assert!(order_state.is_ok(), "{:?}", order_state);
    /// ```
    pub async fn open_orders<S>(&self, symbol: S, is_isolated: Option<bool>) -> Result<Vec<MarginOrderState>>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_OPEN_ORDERS,
                Some(MarginPairQuery {
                    symbol: symbol.into(),
                    is_isolated: is_isolated.map(bool_to_string),
                }),
                self.recv_window,
            )
            .await
    }

    /// Get all orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = MarginOrdersQuery {
    ///    symbol: "BTCUSDT".to_string(),
    ///    ..MarginOrdersQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.orders(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn orders(&self, all_orders_query: MarginOrdersQuery) -> Result<Vec<MarginOrderState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_ORDERS, Some(all_orders_query), self.recv_window)
            .await
    }

    /// Get all trades
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = MarginOwnTradesQuery {
    ///    symbol: "BTCUSDT".to_string(),
    ///    ..MarginOwnTradesQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.trades(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn trades(&self, my_trades_query: MarginOwnTradesQuery) -> Result<Vec<OwnTradesState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_MY_TRADES, Some(my_trades_query), self.recv_window)
            .await
    }

    /// Get an existing oco order state
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = MarginOCOOrderQuery {
    ///     symbol: Some("BTCUSDT".to_string()),
    ///     ..MarginOCOOrderQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.oco_order(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn oco_order(&self, query: MarginOCOOrderQuery) -> Result<MarginOCOOrderResult> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_OCO_ORDER_LIST, Some(query), self.recv_window)
            .await
    }

    /// Query all OCO Orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = OCORecordsQuery {
    ///     symbol: Some("BTCUSDT".to_string()),
    ///     ..OCORecordsQuery::default()
    /// };
    /// let records = tokio_test::block_on(margin.all_oco_orders(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn all_oco_orders(&self, query: OCORecordsQuery) -> Result<Vec<MarginOCOOrderResult>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_OCO_ALL_ORDER_LIST, Some(query), self.recv_window)
            .await
    }

    /// Query open OCO Orders
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let records_query = MarginPairQuery {
    ///     symbol: "BTCUSDT".to_string(),
    ///     is_isolated: None
    /// };
    /// let records = tokio_test::block_on(margin.open_oco_orders(records_query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn open_oco_orders(&self, query: MarginPairQuery) -> Result<Vec<MarginOCOOrderResult>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_OCO_OPEN_ORDER_LIST, Some(query), self.recv_window)
            .await
    }

    /// Get max borrowable
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let max = tokio_test::block_on(margin.max_borrowable("BTC", None));
    /// assert!(max.is_ok(), "{:?}", max);
    /// ```
    pub async fn max_borrowable<S>(&self, asset: S, isolated_symbol: Option<String>) -> Result<MaxBorrowableAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_MAX_BORROWABLE,
                Some(MarginAssetQuery {
                    asset: asset.into(),
                    isolated_symbol,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get max transferable
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*, rest_model::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let max = tokio_test::block_on(margin.max_transferable("BTC", None));
    /// assert!(max.is_ok(), "{:?}", max);
    /// ```
    pub async fn max_transferable<S>(&self, asset: S, isolated_symbol: Option<String>) -> Result<MaxTransferableAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_MAX_TRANSFERABLE,
                Some(MarginAssetQuery {
                    asset: asset.into(),
                    isolated_symbol,
                }),
                self.recv_window,
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
        self.client.post(SAPI_USER_DATA_STREAM, None).await
    }

    /// Keep the connection alive
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
        self.client.put(SAPI_USER_DATA_STREAM, listen_key, None).await
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
        self.client.delete(SAPI_USER_DATA_STREAM, listen_key, None).await
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
    pub async fn start_isolated(&self, symbol: &str) -> Result<UserDataStream> {
        self.client.post(SAPI_USER_DATA_STREAM_ISOLATED, Some(symbol)).await
    }

    /// Keep the connection alive
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, margin::*, config::*};
    /// let margin: Margin = Binance::new_with_env(&Config::testnet());
    /// let start = tokio_test::block_on(margin.start());
    /// assert!(start.is_ok(), "{:?}", start);
    /// let keep_alive = tokio_test::block_on(margin.keep_alive(&start.unwrap().listen_key));
    /// assert!(keep_alive.is_ok())
    /// ```
    pub async fn keep_alive_isolated(&self, listen_key: &str, symbol: &str) -> Result<Success> {
        self.client
            .put(SAPI_USER_DATA_STREAM_ISOLATED, listen_key, Some(symbol))
            .await
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
    pub async fn close_isolated(&self, listen_key: &str, symbol: &str) -> Result<Success> {
        self.client
            .delete(SAPI_USER_DATA_STREAM_ISOLATED, listen_key, Some(symbol))
            .await
    }

    pub async fn isolated_account_limit(&self) -> Result<IsolatedAccountLimit> {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_ISOLATED_ACCOUNT_LIMIT,
                None::<PairQuery>,
                self.recv_window,
            )
            .await
    }
}
