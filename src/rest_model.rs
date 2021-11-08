#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
    pub exchange_filters: Vec<Filters>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub quote_asset_precision: u64,
    pub base_commission_precision: u64,
    pub quote_commission_precision: u64,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<Filters>,
    pub permissions: Vec<SymbolPermission>,
}

impl Symbol {
    pub fn lot_size(&self) -> Option<Filters> {
        self.filters
            .clone()
            .into_iter()
            .find(|filter| matches!(filter, Filters::LotSize { .. }))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType")]
pub enum Filters {
    #[serde(rename = "PRICE_FILTER")]
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(with = "string_or_float")]
        min_price: f64,
        #[serde(with = "string_or_float")]
        max_price: f64,
        #[serde(with = "string_or_float")]
        tick_size: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "string_or_float")]
        multiplier_up: f64,
        #[serde(with = "string_or_float")]
        multiplier_down: f64,
        avg_price_mins: u64,
    },
    #[serde(rename = "LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(with = "string_or_float")]
        min_qty: f64,
        #[serde(with = "string_or_float")]
        max_qty: f64,
        #[serde(with = "string_or_float")]
        step_size: f64,
    },
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        #[serde(with = "string_or_float")]
        min_qty: f64,
        #[serde(with = "string_or_float")]
        max_qty: f64,
        #[serde(with = "string_or_float")]
        step_size: f64,
    },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(with = "string_or_float")]
        min_notional: f64,
        apply_to_market: bool,
        avg_price_mins: u64,
    },
    #[serde(rename = "ICEBERG_PARTS")]
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u16 },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u16 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u16 },
    #[serde(rename = "MAX_NUM_ICEBERG_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },
    #[serde(rename = "MAX_POSITION")]
    #[serde(rename_all = "camelCase")]
    MaxPosition {
        #[serde(with = "string_or_float")]
        max_position: f64,
    },
    #[serde(rename = "EXCHANGE_MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumOrders { max_num_orders: u16 },
    #[serde(rename = "EXCHANGE_MAX_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumAlgoOrders { max_num_algo_orders: u16 },
    #[serde(other)]
    Others,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub account_type: AccountType,
    pub balances: Vec<Balance>,
    pub permissions: Vec<AccountType>,
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketPermission {
    Spot,
    Margin,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Spot,
    UsdtFuture,
    CoinFuture,
    Leveraged,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub locked: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i32,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    #[serde(with = "string_or_float")]
    pub iceberg_qty: f64,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    #[serde(with = "string_or_float")]
    pub orig_quote_order_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    pub client_order_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionId {
    pub tran_id: u64,
}

/// Response to a test order (endpoint /api/v3/order/test).
///
/// Currently, the API responds {} on a successfull test transaction,
/// hence this struct has no fields.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestResponse {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AveragePrice {
    pub mins: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Tickers>),
}

#[derive(Debug, Clone)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tickers {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub commission: String,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub prev_close_price: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MarginTransferType {
    FromMainToMargin = 1,
    FromMarginToMain = 2,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub asset: String,
    pub amount: f64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub transfer_type: MarginTransferType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IsolatedMarginTransferType {
    Spot,
    IsolatedMargin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedTransfer {
    pub asset: String,
    pub symbol: String,
    pub amount: f64,
    pub trans_from: IsolatedMarginTransferType,
    pub trans_to: IsolatedMarginTransferType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Loan {
    pub asset: String,
    pub amount: f64,
    pub is_isolated: Option<String>,
    pub symbol: Option<String>,
}

/// How long will an order stay alive
#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum TimeInForce {
    /// Good Till Canceled
    GTC,
    /// Immediate Or Cancel
    IOC,
    /// Fill or Kill
    FOK,
    /// Good till expired
    GTX,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponse {
    Ack,
    Result,
    Full,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SideEffectType {
    NoSideEffect,
    MarginBuy,
    AutoRepay,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// By default, buy
impl Default for OrderSide {
    fn default() -> Self { Self::Buy }
}

/// Order types, the following restrictions apply
/// LIMIT_MAKER are LIMIT orders that will be rejected if they would immediately match and trade as a taker.
/// STOP_LOSS and TAKE_PROFIT will execute a MARKET order when the stopPrice is reached.
/// Any LIMIT or LIMIT_MAKER type order can be made an iceberg order by sending an icebergQty.
/// Any order with an icebergQty MUST have timeInForce set to GTC.
/// MARKET orders using quantity specifies how much a user wants to buy or sell based on the market price.
/// MARKET orders using quoteOrderQty specifies the amount the user wants to spend (when buying) or receive (when selling) of the quote asset; the correct quantity will be determined based on the market liquidity and quoteOrderQty.
/// MARKET orders using quoteOrderQty will not break LOT_SIZE filter rules; the order will execute a quantity that will have the notional value as close as possible to quoteOrderQty.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
    #[serde(other)]
    Other,
}

/// By default, use market orders
impl Default for OrderType {
    fn default() -> Self { Self::Market }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrder {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub order_type: OrderType,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    /// Used with `OrderType::StopLoss`, `OrderType::StopLossLimit`, `OrderType::TakeProfit` and `OrderType::TakeProfitLimit`
    pub stop_price: Option<f64>,
    pub new_client_order_id: Option<String>,
    /// Used with `OrderType::Limit`, `OrderType::StopLossLimit` and `OrderType::TakeProfitLimit` to create an iceberg order
    pub iceberg_qty: Option<f64>,
    /// Default is `OrderResponse::ACK`
    pub new_order_resp_type: OrderResponse,
    /// N.B. : do not set with `OrderType::Market`
    pub time_in_force: Option<TimeInForce>,
    /// "TRUE" or "FALSE", in upper case, default is "FALSE"
    pub is_isolated: Option<String>,
    /// Default is `SideEffectType::NoSideEffect`
    pub side_effect_type: SideEffectType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderCancellation {
    pub symbol: String,
    pub order_id: u64,
    pub orig_client_order_id: String,
    pub new_client_order_id: String,
    pub is_isolated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderCancellationResult {
    pub symbol: String,
    pub order_id: u64,
    pub orig_client_order_id: String,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub order_type: OrderType,
    pub side: OrderSide,
    pub is_isolated: Option<bool>,
    pub order_list_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrdersCancellation {
    pub symbol: String,
    pub is_isolated: Option<String>,
}

pub type MarginOrdersCancellationResult = Vec<MarginOrderCancellationResult>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginOCOOrder {
    pub symbol: String,
    /// "TRUE" or "FALSE", in upper case, default is "FALSE"
    pub is_isolated: Option<String>,
    /// A unique identifier that will be applied to all orders
    pub list_client_order_id: Option<String>,
    pub side: OrderSide,
    pub quantity: f64,
    /// A unique identifier that will be applied to the limit order
    pub limit_client_order_id: Option<String>,
    pub price: f64,
    pub limit_iceberg_qty: Option<f64>,
    /// A unique identifier that will be applied to the stop order
    pub stop_client_order_id: Option<String>,
    pub stop_price: f64,
    pub stop_limit_price: Option<f64>,
    pub stop_iceberg_qty: Option<f64>,
    pub stop_limit_time_in_force: Option<TimeInForce>,
    /// Default is `OrderResponse::ACK`
    pub new_order_resp_type: Option<OrderResponse>,
    /// Default is `SideEffectType::NoSideEffect`
    pub side_effect_type: Option<SideEffectType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOCOOrderResult {
    pub order_list_id: u64,
    pub contingency_type: ContingencyType,
    pub list_status_type: OCOStatus,
    pub list_order_status: OCOOrderStatus,
    pub list_client_order_id: Option<String>,
    pub transaction_time: u128,
    pub symbol: String,
    #[serde(default, with = "string_or_float_opt")]
    pub margin_buy_borrow_amount: Option<f64>,
    pub margin_buy_borrow_asset: Option<String>,
    pub is_isolated: Option<bool>,
    pub orders: Vec<OCOOrderDetail>,
    pub order_reports: Vec<OCOOrderReport>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OCOOrderDetail {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OCOOrderReport {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: Option<String>,
    pub transact_time: u128,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    #[serde(rename = "type")]
    pub side: OrderSide,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
}

/// archived and is_isolated are only applicable to certain endpoints
/// refer to Binance documentation for full disclosure
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecordsQuery {
    pub asset: String,
    pub tx_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub current: Option<u16>,
    pub size: Option<u8>,
    pub transfer_type: Option<TransferType>,
    pub archived: Option<bool>,
    /// "TRUE" or "FALSE", default is "FALSE"
    pub is_isolated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct OCORecordsQuery {
    pub symbol: Option<String>,
    pub from_id: Option<u64>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u8>,
    /// "TRUE" or "FALSE", default is "FALSE"
    pub is_isolated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrdersQuery {
    pub symbol: String,
    /// "TRUE" or "FALSE", default is "FALSE"
    pub is_isolated: Option<String>,
    pub order_id: u64,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginOwnTradesQuery {
    pub symbol: String,
    /// "TRUE" or "FALSE", default is "FALSE"
    pub is_isolated: Option<String>,
    pub from_id: u64,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u8>,
}

/// archived and is_isolated are only applicable to certain endpoints
/// refer to Binance documentation for full disclosure
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedTransfersQuery {
    pub symbol: String,
    pub asset: Option<String>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub current: Option<u16>,
    pub size: Option<u8>,
    pub trans_from: Option<IsolatedMarginTransferType>,
    pub trans_to: Option<IsolatedMarginTransferType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepayState {
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub interest: f64,
    #[serde(with = "string_or_float")]
    pub principal: f64,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub tx_id: u64,
    pub isolated_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoanState {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub principal: f64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub isolated_symbol: Option<String>,
    pub tx_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferType {
    #[serde(rename = "ROLL_IN")]
    RollIn,
    #[serde(rename = "ROLL_OUT")]
    RollOut,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderState {
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub asset: String,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub tx_id: u64,
    #[serde(rename = "type")]
    pub transfer_type: TransferType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InterestType {
    /// First interested charged on borrow
    OnBorrow,
    /// Interested charged per hour
    Periodic,
    /// Interested charged per hour converted into BNB
    PeriodicConverted,
    /// First interested charged on borrow converted into BNB
    OnBorrowConverted,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestState {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub interest: f64,
    pub interest_accured_time: u64,
    #[serde(with = "string_or_float")]
    pub interest_rate: f64,
    #[serde(with = "string_or_float")]
    pub principal: f64,
    #[serde(rename = "type")]
    pub interest_type: InterestType,
    pub isolated_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ForcedLiquidationState {
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub side: OrderSide,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    pub updated_time: u128,
    pub is_isolated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordsQueryResult<R> {
    pub rows: Vec<R>,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAsset {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub borrowed: f64,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub interest: f64,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    #[serde(with = "string_or_float")]
    pub net_asset: f64,
}

pub type UserAssets = Vec<UserAsset>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginAccountDetails {
    pub borrow_enabled: bool,
    #[serde(with = "string_or_float")]
    pub margin_level: f64,
    #[serde(with = "string_or_float")]
    pub total_asset_of_btc: f64,
    #[serde(with = "string_or_float")]
    pub total_liability_of_btc: f64,
    #[serde(with = "string_or_float")]
    pub total_net_asset_of_btc: f64,
    pub trade_enabled: bool,
    pub transfer_enabled: bool,
    pub user_assets: UserAssets,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginAccountAsset {
    pub asset: String,
    pub borrow_enabled: bool,
    #[serde(with = "string_or_float")]
    pub borrowed: f64,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub interest: f64,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    #[serde(with = "string_or_float")]
    pub net_asset: f64,
    #[serde(with = "string_or_float")]
    pub net_asset_of_btc: f64,
    pub repay_enabled: bool,
    #[serde(with = "string_or_float")]
    pub total_asset: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginAccountAssetDetails {
    pub base_asset: IsolatedMarginAccountAsset,
    pub quote_asset: IsolatedMarginAccountAsset,
    pub symbol: String,
    pub isolated_created: bool,
    pub enabled: bool,
    #[serde(with = "string_or_float")]
    pub margin_level: f64,
    #[serde(with = "string_or_float")]
    pub margin_ratio: f64,
    pub margin_level_status: MarginLevelStatus,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float")]
    pub liquidate_price: f64,
    #[serde(with = "string_or_float")]
    pub liquidate_rate: f64,
    pub trade_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginLevelStatus {
    Excessive,
    Normal,
    MarginCall,
    PreLiquidation,
    ForceLiquidation,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginAccountDetails {
    pub assets: Vec<IsolatedMarginAccountAssetDetails>,
    #[serde(default, with = "string_or_float_opt")]
    pub total_asset_of_btc: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub total_liability_of_btc: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub total_net_asset_of_btc: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetQuery {
    pub asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginAssetQuery {
    pub asset: String,
    pub isolated_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetails {
    pub asset_full_name: String,
    pub asset_name: String,
    pub is_borrowable: bool,
    pub is_mortgageable: bool,
    #[serde(with = "string_or_float")]
    pub user_min_borrow: f64,
    #[serde(with = "string_or_float")]
    pub user_min_repay: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairQuery {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginPairQuery {
    pub symbol: String,
    pub is_isolated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedMarginPairQuery {
    pub symbols: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairAndWindowQuery {
    pub symbol: String,
    pub recv_window: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PairDetails {
    pub id: u128,
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub is_margin_trade: bool,
    pub is_buy_allowed: bool,
    pub is_sell_allowed: bool,
}

pub type AllAssets = Vec<AssetDetails>;

pub type AllPairs = Vec<PairDetails>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedPairDetails {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub is_margin_trade: bool,
    pub is_buy_allowed: bool,
    pub is_sell_allowed: bool,
}

pub type AllIsolatedPairs = Vec<IsolatedPairDetails>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceIndex {
    pub calc_time: u128,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub symbol: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderQuery {
    pub symbol: String,
    pub is_isolated: Option<String>,
    pub order_id: Option<String>,
    pub orig_client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderResult {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u128,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub order_type: OrderType,
    pub side: OrderSide,
    #[serde(default, with = "string_or_float_opt")]
    pub margin_buy_borrow_amount: Option<f64>,
    pub margin_buy_borrow_asset: Option<String>,
    pub is_isolated: Option<bool>,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderState {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub iceberg_qty: f64,
    pub is_working: bool,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: OrderSide,
    pub status: OrderStatus,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub symbol: String,
    pub is_isolated: Option<bool>,
    pub time: u64,
    pub time_in_force: TimeInForce,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub order_type: OrderType,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderSumaryState {
    pub id: u64,
    pub price: f64,
    pub qty: f64,
    pub quote_qty: f64,
    pub symbol: String,
    pub time: u128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OwnTradesState {
    #[serde(with = "string_or_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub id: u64,
    pub is_best_match: bool,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub symbol: String,
    pub time: u128,
    pub is_isolated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaxBorrowableAmount {
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub borrow_limit: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaxTransferableAmount {
    #[serde(with = "string_or_float")]
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
    PendingTrading,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolPermission {
    Spot,
    Margin,
    #[serde(other)]
    Other,
}

/// Status of an order, this can typically change over time
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    /// The order has been accepted by the engine.
    New,
    /// A part of the order has been filled.
    PartiallyFilled,
    /// The order has been completely filled.
    Filled,
    /// The order has been canceled by the user.
    Canceled,
    /// (currently unused)
    PendingCancel,
    /// The order was not accepted by the engine and not processed.
    Rejected,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance)
    Expired,
    /// Part of the order or all of the order's quantity has filled.
    Trade,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OCOStatus {
    Response,
    ExecStarted,
    AllDone,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OCOOrderStatus {
    Executing,
    AllDone,
    Reject,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOCOOrderCancellation {
    pub symbol: String,
    pub order_list_id: u64,
    pub list_client_order_id: String,
    pub new_client_order_id: String,
    pub is_isolated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginOCOOrderQuery {
    pub symbol: Option<String>,
    pub is_isolated: Option<String>,
    pub order_list_id: Option<String>,
    pub orig_client_order_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContingencyType {
    OCO,
    #[serde(other)]
    Other,
}

/// API Rate Limit
/// Example
/// {
///   "rateLimitType": "REQUEST_WEIGHT",
///   "interval": "MINUTE",
///   "intervalNum": 1,
///   "limit": 1200
/// }
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
    #[serde(other)]
    Other,
}

/// Rate Limit Interval, used by RateLimitType
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub interval: RateLimitInterval,
    pub rate_limit_type: RateLimitType,
    pub interval_num: i32,
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BnbBurnQuery {
    /// "true" or "false", defaults to "false"
    #[serde(rename = "spotBNBBurn")]
    pub spot_bnb_burn: Option<String>,
    /// "true" or "false", defaults to "false"
    #[serde(rename = "interestBNBBurn")]
    pub interest_bnb_burn: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BnbBurnStatus {
    #[serde(rename = "spotBNBBurn")]
    pub spot_bnb_burn: Option<bool>,
    #[serde(rename = "interestBNBBurn")]
    pub interest_bnb_burn: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateHistoryQuery {
    pub asset: String,
    pub vip_level: Option<u8>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateAssetHistory {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub daily_interest_rate: f64,
    pub timestamp: u128,
    pub vip_level: u8,
}

pub type InterestRateHistory = Vec<InterestRateAssetHistory>;

#[derive(Debug, Clone)]
pub struct KlineSummary {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub close_time: i64,
    pub quote_asset_volume: f64,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: f64,
    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyCmd {
    pub id: i32,
    pub method: String,
    pub params: (String, bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedAccountLimit {
    pub enabled_account: u64,
    pub max_account: u64,
}

pub mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

pub(crate) mod string_or_float_opt {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::rest_model::string_or_float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        Ok(Some(crate::rest_model::string_or_float::deserialize(deserializer)?))
    }
}

pub(crate) mod string_or_bool {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Bool(bool),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Bool(i) => Ok(i),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::rest_model::ExchangeInformation;

    #[test]
    fn exchange_info_serde() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/exchangeInfo.json");
        let fc = std::fs::read_to_string(d).unwrap();
        let result = serde_json::from_str::<ExchangeInformation>(&fc);
        assert!(result.is_ok(), "{:?}", result);
    }
}
