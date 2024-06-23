use chrono::{DateTime, Utc};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

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
            .iter()
            .find(|filter| matches!(filter, Filters::LotSize { .. }))
            .cloned()
    }

    pub fn market_lot_size(&self) -> Option<Filters> {
        self.filters
            .iter()
            .find(|filter| matches!(filter, Filters::MarketLotSize { .. }))
            .cloned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketPermission {
    Spot,
    Margin,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
pub struct OrderCanceledReplaced {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: OrderCanceled,
    pub new_order_response: Transaction,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Tickers>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
#[serde(rename_all = "camelCase")]
pub struct HistoricalTrade {
    pub time: u64,
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "qty", with = "string_or_float")]
    pub quantity: f64,
    #[serde(rename = "quoteQty", with = "string_or_float")]
    pub quote_quantity: f64,
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggTrade {
    #[serde(rename = "T")]
    pub time: u64,
    #[serde(rename = "a")]
    pub agg_id: u64,
    #[serde(rename = "f")]
    pub first_id: u64,
    #[serde(rename = "l")]
    pub last_id: u64,
    #[serde(rename = "m")]
    pub maker: bool,
    #[serde(rename = "M")]
    pub best_match: bool,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, PartialEq, Eq)]
#[repr(u8)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderResponse {
    Ack,
    Result,
    Full,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SideEffectType {
    NoSideEffect,
    MarginBuy,
    AutoRepay,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// By default, buy
impl Default for OrderSide {
    fn default() -> Self {
        Self::Buy
    }
}

/// The allowed values are:
/// STOP_ON_FAILURE - If the cancel request fails, the new order placement will not be attempted.
/// ALLOW_FAILURE - new order placement will be attempted even if cancel request fails.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelReplaceMode {
    StopOnFailure,
    AllowFailure,
}

/// By default, STOP_ON_FAILURE
impl Default for CancelReplaceMode {
    fn default() -> Self {
        Self::StopOnFailure
    }
}

/// Order types, the following restrictions apply
/// LIMIT_MAKER are LIMIT orders that will be rejected if they would immediately match and trade as a taker.
/// STOP_LOSS and TAKE_PROFIT will execute a MARKET order when the stopPrice is reached.
/// Any LIMIT or LIMIT_MAKER type order can be made an iceberg order by sending an icebergQty.
/// Any order with an icebergQty MUST have timeInForce set to GTC.
/// MARKET orders using quantity specifies how much a user wants to buy or sell based on the market price.
/// MARKET orders using quoteOrderQty specifies the amount the user wants to spend (when buying) or receive (when selling) of the quote asset; the correct quantity will be determined based on the market liquidity and quoteOrderQty.
/// MARKET orders using quoteOrderQty will not break LOT_SIZE filter rules; the order will execute a quantity that will have the notional value as close as possible to quoteOrderQty.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    fn default() -> Self {
        Self::Market
    }
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
    #[serde(with = "string_or_u64_opt")]
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,
    pub client_order_id: Option<String>,
    #[serde(with = "string_or_float_opt")]
    pub price: Option<f64>,
    #[serde(with = "string_or_float_opt")]
    pub orig_qty: Option<f64>,
    #[serde(with = "string_or_float_opt")]
    pub executed_qty: Option<f64>,
    #[serde(with = "string_or_float_opt")]
    pub cummulative_quote_qty: Option<f64>,
    pub status: Option<OrderStatus>,
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub order_type: Option<OrderType>,
    pub side: Option<OrderSide>,
    pub is_isolated: Option<bool>,
    pub order_list_id: Option<i64>,
    pub transaction_time: Option<u64>,
    pub contingency_type: Option<ContingencyType>,
    pub orders: Option<Vec<OCOOrderDetail>>,
    pub order_reports: Option<Vec<OCOOrderReport>>,
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
    pub side: OrderSide,
    #[serde(default, with = "string_or_float_opt")]
    pub stop_price: Option<f64>,
    #[serde(default, with = "string_or_float_opt")]
    pub iceberg_qty: Option<f64>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    pub rows: Option<Vec<R>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    #[serde(with = "string_or_u64")]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolPermission {
    Spot,
    Margin,
    #[serde(other)]
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionType {
    /// The order has been accepted into the engine.
    New,
    /// The order has been canceled by the user.
    Canceled,
    /// Currently unused
    Replaced,
    /// The order has been rejected and was not processed (This message appears only with Cancel Replace Orders wherein the new order placement is rejected but the request to cancel request succeeds.)
    Rejected,
    /// Part of the order or all of the order's quantity has filled.
    Trade,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance).
    Expired,
    /// The order has expired due to STP trigger.
    TradePrevention,
}

/// Status of an order, this can typically change over time
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
    /// Currently unused
    PendingCancel,
    /// The order was not accepted by the engine and not processed.
    Rejected,
    /// The order was canceled according to the order type's rules (e.g. LIMIT FOK orders with no fill, LIMIT IOC or MARKET orders that partially fill) or by the exchange, (e.g. orders canceled during liquidation, orders canceled during maintenance)
    Expired,
    /// The order was canceled by the exchange due to STP trigger. (e.g. an order with EXPIRE_TAKER will match with existing orders on the book with the same account or same tradeGroupId)
    ExpiredInMatch,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OCOStatus {
    Response,
    ExecStarted,
    AllDone,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    RequestWeight,
    Orders,
    RawRequests,
    #[serde(other)]
    Other,
}

/// Rate Limit Interval, used by RateLimitType
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IsolatedSymbol {
    pub symbol: String,
    pub max_account: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemStatus {
    pub status: u64,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletCoinInfo {
    pub coin: String,
    pub deposit_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub freeze: f64,
    #[serde(with = "string_or_float")]
    pub ipoable: f64,
    #[serde(with = "string_or_float")]
    pub ipoing: f64,
    pub is_legal_money: bool,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    pub name: String,
    pub network_list: Vec<CoinNetwork>,
    #[serde(with = "string_or_float")]
    pub storage: f64,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdrawing: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinNetwork {
    pub address_regex: String,
    pub coin: String,
    #[serde(default)]
    pub deposit_desc: String,
    pub deposit_enable: bool,
    pub is_default: bool,
    pub memo_regex: String,
    pub min_confirm: u32,
    pub name: String,
    pub network: String,
    pub reset_address_status: bool,
    pub special_tips: Option<String>,
    pub un_lock_confirm: u32,
    #[serde(default)]
    pub withdraw_desc: String,
    pub withdraw_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_integer_multiple: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_max: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_min: f64,
    #[serde(default)]
    pub same_address: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSnapshot {
    pub code: u32,
    pub msg: String,
    pub snapshot_vos: Vec<SnapshotVos>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotVos {
    pub data: SnapshotVosData,
    #[serde(rename = "type")]
    pub snapshot_type: String,
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotVosData {
    pub balances: Vec<Balance>,
    #[serde(with = "string_or_float")]
    pub total_asset_of_btc: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountSnapshotType {
    Spot,
    Margin,
    Futures,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountSnapshotQuery {
    #[serde(rename = "type")]
    pub account_type: AccountSnapshotType,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CoinWithdrawalQuery {
    pub coin: String,
    /// client id for withdraw
    pub withdraw_order_id: Option<String>,
    pub network: Option<String>,
    pub address: String,
    /// Secondary address identifier for coins like XRP,XMR etc.
    pub address_tag: Option<String>,
    pub amount: f64,
    /// When making internal transfer, true for returning the fee to the destination account; false for returning the fee back to the departure account. Default false.
    pub transaction_fee_flag: Option<bool>,
    /// Description of the address. Space in name should be encoded into %20.
    pub name: Option<String>,
    /// The wallet type for withdraw，0: spot wallet. 1: funding wallet. Default:  spot wallet
    pub wallet_type: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositHistoryQuery {
    pub coin: Option<String>,
    /// 0(0:pending,6: credited but cannot withdraw, 1:success)
    pub status: Option<u16>,
    /// Default: 90 days from current timestamp
    pub start_time: Option<u64>,
    /// Default: present timestamp
    pub end_time: Option<u64>,
    /// Default:1000, Max:1000
    pub limit: Option<u64>,
    /// Default: present timestamp
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    pub coin: String,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub network: String,
    pub status: u8,
    pub address: String,
    pub address_tag: Option<String>,
    pub tx_id: String,
    pub insert_time: Option<u64>,
    pub transfer_type: u8,
    #[serde(default)]
    pub unlock_confirm: u32,
    pub confirm_times: String,
    pub wallet_type: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalHistoryQuery {
    pub coin: Option<String>,
    pub withdraw_order_id: Option<String>,
    /// 0(0:Email Sent,1:Cancelled 2:Awaiting Approval 3:Rejected 4:Processing 5:Failure 6:Completed)
    pub status: Option<u16>,
    /// Default: 90 days from current timestamp
    pub start_time: Option<u64>,
    /// Default: present timestamp
    pub end_time: Option<u64>,
    /// Default:1000, Max:1000
    pub limit: Option<u64>,
    /// Default: present timestamp
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecordHistory<T> {
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub records: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecord {
    pub address: String,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub apply_time: String,
    pub coin: String,
    pub id: String,
    /// // will not be returned if there's no withdrawOrderId for this withdraw.
    pub withdraw_order_id: Option<String>,
    pub network: String,
    /// 1 for internal transfer, 0 for external transfer
    pub transfer_type: u8,
    pub status: u8,
    #[serde(with = "string_or_float")]
    pub transaction_fee: f64,
    /// // confirm times for withdraw
    pub confirm_no: Option<u64>,
    pub info: Option<String>,
    pub tx_id: String,
}

#[cfg(feature = "wallet_api")]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressQuery {
    pub coin: String,
    /// If network is not send, return with default network of the coin.
    /// You can get network and isDefault in networkList in the response [`crate::wallet::Wallet::all_coin_info`]
    pub network: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddress {
    pub coin: String,
    pub address: String,
    pub tag: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniversalTransferType {
    /// Spot account transfer to USDⓈ-M Futures account
    MainUmfuture,
    /// Spot account transfer to COIN-M Futures account
    MainCmfuture,
    /// Spot account transfer to Margin (cross) account
    MainMargin,
    /// USDⓈ-M Futures account transfer to Spot account
    UmfutureMain,
    /// USDⓈ-M Futures account transfer to Margin (cross) account
    UmFutureMargin,
    /// COIN-M Futures account transfer to Spot account
    CmfutureMain,
    /// COIN-M Futures account transfer to Margin(cross) account
    CmfutureMargin,
    /// Margin（cross）account transfer to Spot account
    MarginMain,
    /// Margin (cross) account transfer to USDⓈ-M Futures
    MarginUmfuture,
    /// Margin (cross) account transfer to COIN-M Futures
    MarginCmfuture,
    /// Isolated margin account transfer to Margin (cross) account
    IsolatedmarginMargin,
    /// Margin (cross) account transfer to Isolated margin account
    MarginIsolatedmargin,
    /// Isolated margin account transfer to Isolated margin account
    IsolatedmarginIsolatedmargin,
    /// Spot account transfer to Funding account
    MainFunding,
    /// Funding account transfer to Spot account
    FundingMain,
    /// Funding account transfer to UMFUTURE account
    FundingUmfuture,
    /// UMFUTURE account transfer to Funding account
    UmfutureFunding,
    /// MARGIN account transfer to Funding account
    MarginFunding,
    /// Funding account transfer to Margin account
    FundingMargin,
    /// Funding account transfer to CMFUTURE account
    FundingCmfuture,
    /// CMFUTURE account transfer to Funding account
    CmfutureFunding,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransfer {
    pub asset: String,
    pub amount: f64,
    pub from_symbol: Option<String>,
    pub to_symbol: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub transfer_type: UniversalTransferType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferHistoryQuery {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub transfer_type: UniversalTransferType,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    /// Default : 1
    pub current: Option<u64>,
    /// Default 10, Max 100
    pub size: Option<u64>,
    pub from_symbol: Option<String>,
    pub to_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniversalTransferStatus {
    Confirmed,
    Pending,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferRecord {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub transfer_type: UniversalTransferType,
    pub status: UniversalTransferStatus,
    pub tran_id: u64,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatus {
    pub data: ApiTradingStatusData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiTradingStatusData {
    /// API trading function is locked or not
    pub is_locked: bool,
    /// If API trading function is locked, this is the planned recover time
    pub planned_recovery_time: Option<u64>,
    pub trigger_condition: ApiTradingStatusTriggerCondition,
    pub update_time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ApiTradingStatusTriggerCondition {
    /// Number of GTC orders
    pub gcr: i64,
    /// Number of FOK/IOC orders
    pub ifer: i64,
    /// Number of orders
    pub ufr: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DustLog {
    /// Total counts of exchange
    pub total: u64,
    pub user_asset_dribblets: Vec<UserAssetDribblet>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribblet {
    pub operate_time: u64,
    /// Total transfered BNB amount for this exchange.
    #[serde(with = "string_or_float")]
    pub total_transfered_amount: f64,
    ///Total service charge amount for this exchange.
    #[serde(with = "string_or_float")]
    pub total_service_charge_amount: f64,
    pub trans_id: u64,
    pub user_asset_dribblet_details: Vec<UserAssetDribbletDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAssetDribbletDetail {
    pub trans_id: u64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub transfered_amount: f64,
    #[serde(with = "string_or_float")]
    pub service_charge_amount: f64,
    pub operate_time: u64,
    pub from_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConvertibleAssets {
    pub details: Vec<ConvertibleAssetDetails>,
    #[serde(with = "string_or_float")]
    #[serde(rename = "totalTransferBtc")]
    pub total_transfer_btc: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "totalTransferBNB")]
    pub total_transfer_bnb: f64,
    #[serde(with = "string_or_float_opt", default)]
    pub driblet_percentage: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConvertibleAssetDetails {
    pub asset: String,
    pub asset_full_name: String,
    #[serde(with = "string_or_float")]
    pub amount_free: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "toBNB")]
    pub to_bnb: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "toBTC")]
    pub to_btc: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "toBNBOffExchange")]
    pub to_bnb_off_exchange: f64,
    #[serde(with = "string_or_float")]
    pub exchange: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DustTransfer {
    #[serde(with = "string_or_float")]
    pub total_service_charge: f64,
    #[serde(with = "string_or_float")]
    pub total_transferred: f64,
    pub transfer_result: Vec<DustTransferResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DustTransferResult {
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub from_asset: String,
    pub operate_time: u64,
    #[serde(with = "string_or_float")]
    pub service_charge_amount: f64,
    pub tran_id: u64,
    #[serde(with = "string_or_float")]
    pub transfered_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividend {
    pub id: u64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    pub asset: String,
    pub div_time: u64,
    pub en_info: String,
    pub tran_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetDividendQuery {
    pub asset: Option<String>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    /// Default 20, max 500
    pub limit: Option<u64>,
}

pub type SupportedAssetDetails = HashMap<String, SupportedAssetDetail>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SupportedAssetDetail {
    /// min withdraw amount
    #[serde(with = "string_or_float_opt")]
    #[serde(rename = "minWithdrawAmount")]
    pub min_withdrawal_amount: Option<f64>,
    /// deposit status (false if ALL of networks' are false)
    pub deposit_status: bool,
    /// withdraw fee
    #[serde(with = "string_or_float_opt")]
    pub withdraw_fee: Option<f64>,
    /// withdraw status (false if ALL of networks' are false)
    pub withdraw_status: bool,
    /// reason
    pub deposit_tip: Option<String>,
}

pub type TradeFees = Vec<TradeFee>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeFee {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub maker_commission: f64,
    #[serde(with = "string_or_float")]
    pub taker_commission: f64,
}

pub type WalletFundings = Vec<WalletFunding>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletFunding {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    #[serde(with = "string_or_float")]
    pub freeze: f64,
    #[serde(with = "string_or_float")]
    pub withdrawing: f64,
    #[serde(with = "string_or_float")]
    pub btc_valuation: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyPermissions {
    ip_restrict: bool,
    create_time: u64,
    /// This option allows you to withdraw via API. You must apply the IP Access Restriction filter in order to enable withdrawals
    enable_withdrawals: bool,
    /// This option authorizes this key to transfer funds between your master account and your sub account instantly
    enable_internal_transfer: bool,
    /// Authorizes this key to be used for a dedicated universal transfer API to transfer multiple supported currencies. Each business's own transfer API rights are not affected by this authorization
    permits_universal_transfer: bool,
    ///  Authorizes this key to Vanilla options trading
    enable_vanilla_options: bool,
    enable_reading: bool,
    ///  API Key created before your futures account opened does not support futures API service
    enable_futures: bool,
    ///  This option can be adjusted after the Cross Margin account transfer is completed
    enable_margin: bool,
    /// Spot and margin trading
    enable_spot_and_margin_trading: bool,
    /// Expiration time for spot and margin trading permission
    trading_authority_expiration_time: Option<u64>,
}

pub type WalletBalances = Vec<WalletBalance>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// Shows whether the wallet is activated or not
    activate: bool,
    /// Shows the overall balance of the wallet quoted in BTC
    balance: String,
    /// Indicates the wallet type: 'Spot', 'Funding', 'Cross Margin', 'Isolated Margin', 'USDⓈ-M Futures', 'COIN-M Futures', 'Earn', 'Options', 'Trading Bots'
    wallet_name: String,
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

    use serde::{Deserializer, Serializer};

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
        Ok(Some(crate::rest_model::string_or_float::deserialize(deserializer)?))
    }
}

pub mod string_or_u64 {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrU64 {
            String(String),
            U64(u64),
        }

        match StringOrU64::deserialize(deserializer)? {
            StringOrU64::String(s) => s.parse().map_err(de::Error::custom),
            StringOrU64::U64(i) => Ok(i),
        }
    }
}

pub mod string_or_u64_opt {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        match value {
            Some(v) => crate::rest_model::string_or_u64::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrU64 {
            String(String),
            U64(u64),
        }

        match StringOrU64::deserialize(deserializer)? {
            StringOrU64::String(s) => s.parse().map_err(de::Error::custom).map(Some),
            StringOrU64::U64(i) => Ok(Some(i)),
        }
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
        assert!(result.is_ok(), "{result:?}");
    }
}
