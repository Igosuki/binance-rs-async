pub use crate::rest_model::{Asks, Bids, OrderSide, OrderStatus, RateLimit, ServerTime, TimeInForce};
use crate::rest_model::string_or_float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub option_contracts: Vec<OptionContract>,
    pub option_assets: Vec<OptionAsset>,
    pub option_symbols: Vec<OptionSymbol>,
    pub rate_limits: Vec<RateLimit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub id: u64,
    pub base_asset: String,
    pub quote_asset: String,
    pub underlying: String,
    pub settle_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionAsset {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionSymbol {
    pub contract_id: u64,
    pub expiry_date: u64,
    pub filters: Vec<Filters>,
    pub id: u64,
    pub symbol: String,
    pub side: OptionSide,
    #[serde(with = "string_or_float")]
    pub strike_price: f64,
    pub underlying: String,
    pub unit: u64,
    #[serde(with = "string_or_float")]
    pub min_qty: f64,
    #[serde(with = "string_or_float")]
    pub max_qty: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub maintenance_margin: f64,
    #[serde(with = "string_or_float")]
    pub min_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub min_maintenance_margin: f64,
    pub price_scale: u16,
    pub quantity_scale: u16,
    pub quote_asset: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionSide {
    Call,
    Put,
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
    #[serde(other)]
    Others,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    #[default]
    Limit,
    Market,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price_change: f64,
    #[serde(with = "string_or_float")]
    pub price_change_percent: f64,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    #[serde(with = "string_or_float")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float")]
    pub ask_price: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_trade_id: i64,
    pub trade_count: u64,
    #[serde(with = "string_or_float")]
    pub strike_price: f64,
    #[serde(with = "string_or_float")]
    pub exercise_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionMarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub bid_iv: f64,
    #[serde(with = "string_or_float")]
    pub ask_iv: f64,
    #[serde(with = "string_or_float")]
    pub mark_iv: f64,
    #[serde(with = "string_or_float")]
    pub delta: f64,
    #[serde(with = "string_or_float")]
    pub theta: f64,
    #[serde(with = "string_or_float")]
    pub gamma: f64,
    #[serde(with = "string_or_float")]
    pub vega: f64,
    #[serde(with = "string_or_float")]
    pub high_price_limit: f64,
    #[serde(with = "string_or_float")]
    pub low_price_limit: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTrade {
    pub id: u64,
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub side: i32,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionOpenInterest {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub sum_open_interest: f64,
    #[serde(with = "string_or_float")]
    pub sum_open_interest_usd: f64,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub equity: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionPosition {
    pub symbol: String,
    pub side: String,
    #[serde(with = "string_or_float")]
    pub quantity: f64,
    #[serde(with = "string_or_float")]
    pub reduced_quantity: f64,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_pnl: f64,
    pub ror: String,
    pub expiry_date: u64,
    #[serde(with = "string_or_float")]
    pub strike_price: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionOrder {
    pub order_id: u64,
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub quantity: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub fee: f64,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
    pub reduce_only: bool,
    pub post_only: bool,
    pub create_time: u64,
    pub update_time: u64,
    pub status: OrderStatus,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    pub source: String,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price_scale: f64,
    #[serde(with = "string_or_float")]
    pub quantity_scale: f64,
    pub option_side: OptionSide,
    pub quote_asset: String,
    pub mmp: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseRecord {
    pub id: String,
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub strike_price: f64,
    #[serde(with = "string_or_float")]
    pub real_strike_price: f64,
    pub exercise_date: u64,
    #[serde(with = "string_or_float")]
    pub quantity: f64,
    #[serde(with = "string_or_float")]
    pub fee: f64,
    pub create_date: u64,
    #[serde(with = "string_or_float")]
    pub price_scale: f64,
    #[serde(with = "string_or_float")]
    pub quantity_scale: f64,
    pub option_side: OptionSide,
    pub expiry_date: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub quantity: f64,
    pub price: Option<f64>,
    pub time_in_force: Option<TimeInForce>,
    pub reduce_only: Option<bool>,
    pub post_only: Option<bool>,
    pub new_order_resp_type: Option<String>,
    pub client_order_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UnderlyingQuery {
    pub underlying: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OptionKlineQuery {
    pub symbol: String,
    pub interval: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u16>,
}
