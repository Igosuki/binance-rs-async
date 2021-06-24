use serde::{Deserialize, Serialize};

use crate::rest_model::{string_or_bool, string_or_float, string_or_float_opt, Asks, Bids, Filters, RateLimit};
pub use crate::rest_model::{BookTickers, KlineSummaries, KlineSummary, ServerTime, SymbolPrice, Tickers};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<String>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub maint_margin_percent: String,
    pub required_margin_percent: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub filters: Vec<Filters>,
    pub order_types: Vec<String>,
    pub time_in_force: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    // Undocumented
    #[serde(rename = "E")]
    pub event_time: u64,
    // Undocumented
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    #[serde(with = "string_or_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float")]
    pub volume: f64,
    #[serde(with = "string_or_float")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    pub last_qty: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: u64,
    pub last_id: u64,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Trades {
    AllTrades(Vec<Trade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub is_buyer_maker: bool,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(with = "string_or_float")]
    pub quote_qty: f64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum AggTrades {
    AllAggTrades(Vec<AggTrade>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "q", with = "string_or_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MarkPrices {
    AllMarkPrices(Vec<MarkPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LiquidationOrders {
    AllLiquidationOrders(Vec<LiquidationOrder>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationOrder {
    #[serde(with = "string_or_float")]
    pub average_price: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub status: String,
    pub symbol: String,
    pub time: u64,
    pub time_in_force: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(with = "string_or_float")]
    pub open_interest: f64,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct Order {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
    pub reduce_only: bool,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub orig_type: String,
    #[serde(with = "string_or_float", default = "default_activation_price")]
    pub activation_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
    pub update_time: u64,
    pub working_type: String,
    pub price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub avg_price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub orig_type: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CanceledOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_quote: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    pub order_id: u64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    pub orig_type: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub reduce_only: bool,
    pub side: String,
    pub position_side: String,
    pub status: String,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: String,
    price_protect: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    pub margin_type: String,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    pub leverage: String,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    pub symbol: String,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    pub position_side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub account_alias: String,
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "crossUnPnl")]
    pub cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub margin_available: bool,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLeverageResponse {
    pub leverage: u8,
    #[serde(with = "string_or_float")]
    pub max_notional_value: f64,
    pub symbol: String,
}

fn default_stop_price() -> f64 { 0.0 }
fn default_activation_price() -> f64 { 0.0 }
fn default_price_rate() -> f64 { 0.0 }

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistoryQuery {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>,
    pub limit: u16,
    pub symbol: String,
    pub interval: Option<String>,
    pub period: Option<String>,
}

impl HistoryQuery {
    pub fn validate(&self) -> crate::errors::Result<()> {
        if let Some(period) = &self.period {
            if !PERIODS.contains(&period.as_str()) {
                return Err(crate::errors::Error::InvalidPeriod(period.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub funding_time: u64,
    #[serde(with = "string_or_float")]
    pub funding_rate: f64,
}

pub static PERIODS: &'static [&str] = &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"];

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestHistory {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub sum_open_interest: f64,
    #[serde(with = "string_or_float")]
    pub sum_open_interest_value: f64,
    pub timestamp: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub long_account: f64,
    #[serde(with = "string_or_float")]
    pub long_short_ratio: f64,
    #[serde(with = "string_or_float")]
    pub short_account: f64,
    pub timestamp: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageBracket {
    pub bracket: u8,
    pub initial_leverage: u8,
    pub notional_cap: u64,
    pub notional_floor: u64,
    pub maint_margin_ratio: f64,
    pub cum: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolBrackets {
    pub symbol: String,
    pub brackets: Vec<LeverageBracket>,
}
