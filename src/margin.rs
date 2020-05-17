use crate::client::*;
use crate::errors::*;
use crate::model::*;
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
    pub async fn transfer<S, F>(
        &self,
        symbol: S,
        qty: F,
        transfer_type: MarginTransferType,
    ) -> Result<TransactionId>
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
    pub async fn trade(&self, margin_order: MarginOrder) -> Result<TransactionId> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_ORDER, margin_order, self.recv_window)
            .await
    }

    /// Cancel an existing order
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
            .delete_signed_p(
                SAPI_V1_MARGIN_REPAY,
                margin_order_cancellation,
                self.recv_window,
            )
            .await
    }

    /// Get existing loan records
    pub async fn loans(&self, loan_query: RecordsQuery) -> Result<RecordsQueryResult<LoanState>> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_LOAN, loan_query, self.recv_window)
            .await
    }

    /// Get existing repay records history
    pub async fn repays(
        &self,
        repays_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<RepayState>> {
        self.client
            .post_signed_p(SAPI_V1_MARGIN_REPAY, repays_query, self.recv_window)
            .await
    }

    /// Get margin account details
    pub async fn details(&self) -> Result<MarginAccountDetails> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ACCOUNT, q).await
    }

    /// Get asset details
    pub async fn asset<S>(&self, asset: S) -> Result<AssetDetails>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_ASSET,
                Some(AssetQuery {
                    asset: asset.into(),
                }),
            )
            .await
    }

    /// Get margin pair market data
    pub async fn pair<S>(&self, symbol: S) -> Result<PairDetails>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_PAIR,
                Some(PairQuery {
                    symbol: symbol.into(),
                }),
            )
            .await
    }

    /// Get all assets details
    pub async fn all_assets(&self) -> Result<AllAssets> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ALL_ASSETS, q).await
    }

    /// Get all pair details
    pub async fn all_pairs(&self) -> Result<AllPairs> {
        let q: Option<PairQuery> = None;
        self.client.get_signed_p(SAPI_V1_MARGIN_ALL_PAIRS, q).await
    }

    /// Get price index
    pub async fn price_index<S>(&self, symbol: S) -> Result<PriceIndex>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_PRICE_INDEX,
                Some(PairQuery {
                    symbol: symbol.into(),
                }),
            )
            .await
    }

    /// Get transfer history
    pub async fn transfers(
        &self,
        transfers_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<OrderState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_TRANSFER, Some(transfers_query))
            .await
    }

    /// Get interest history
    pub async fn interests(
        &self,
        interest_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<InterestState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_INTEREST_HISTORY, Some(interest_query))
            .await
    }

    /// Get forced liquidation history
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
    pub async fn order(&self, margin_order: MarginOrderQuery) -> Result<MarginOrderState> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ORDER, Some(margin_order))
            .await
    }

    /// Get open orders
    pub async fn open_orders<S>(&self, symbol: S) -> Result<MarginOrderState>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_OPEN_ORDERS,
                Some(PairQuery {
                    symbol: symbol.into(),
                }),
            )
            .await
    }

    /// Get all orders
    pub async fn orders(
        &self,
        all_orders_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<OrderSumaryState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_ALL_ORDERS, Some(all_orders_query))
            .await
    }

    /// Get all trades
    pub async fn trades(
        &self,
        all_orders_query: RecordsQuery,
    ) -> Result<RecordsQueryResult<OwnTradesState>> {
        self.client
            .get_signed_p(SAPI_V1_MARGIN_MY_TRADES, Some(all_orders_query))
            .await
    }

    /// Get max borrowable
    pub async fn max_borrowable<S>(&self, asset: S) -> Result<MaxAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_MAX_BORROWABLE,
                Some(AssetQuery {
                    asset: asset.into(),
                }),
            )
            .await
    }

    /// Get max transferable
    pub async fn max_transferable<S>(&self, asset: S) -> Result<MaxAmount>
    where
        S: Into<String>,
    {
        self.client
            .get_signed_p(
                SAPI_V1_MARGIN_MAX_TRANSFERABLE,
                Some(AssetQuery {
                    asset: asset.into(),
                }),
            )
            .await
    }

    /// Start user data stream
    pub async fn start(&self) -> Result<UserDataStream> {
        let data = self.client.post(SAPI_USER_DATA_STREAM).await?;
        let user_data_stream: UserDataStream = from_str(data.as_str())?;

        Ok(user_data_stream)
    }

    /// Current open orders on a symbol
    pub async fn keep_alive(&self, listen_key: &str) -> Result<Success> {
        let data = self.client.put(SAPI_USER_DATA_STREAM, listen_key).await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }

    pub async fn close(&self, listen_key: &str) -> Result<Success> {
        let data = self
            .client
            .delete(SAPI_USER_DATA_STREAM, listen_key)
            .await?;

        let success: Success = from_str(data.as_str())?;

        Ok(success)
    }
}
