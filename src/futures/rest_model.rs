use crate::rest_model::{string_or_bool, string_or_float_opt};
pub use crate::rest_model::{
    string_or_float, string_or_u64, Asks, Bids, BookTickers, KlineSummaries, KlineSummary, OrderSide, OrderStatus,
    RateLimit, ServerTime, SymbolPrice, SymbolStatus, Tickers, TimeInForce,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub futures_type: String,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<Filters>,
    pub assets: Vec<AssetDetail>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    pub asset: String,
    pub margin_available: bool,
    #[serde(with = "string_or_float")]
    pub auto_asset_exchange: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: ContractType,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub status: SymbolStatus,
    #[serde(with = "string_or_float")]
    pub maint_margin_percent: f64,
    #[serde(with = "string_or_float")]
    pub required_margin_percent: f64,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u64,
    pub quote_precision: u64,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: u64,
    #[serde(with = "string_or_float")]
    pub trigger_protect: f64,
    pub filters: Vec<Filters>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ContractType {
    Perpetual,
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,
    #[serde(rename = "CURRENT_QUARTER DELIVERING")]
    CurrentQuarterDelivery,
    PerpetualDelivering,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,
}

/// By default, use market orders
impl Default for OrderType {
    fn default() -> Self {
        Self::Market
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Both,
    Long,
    Short,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WorkingType {
    MarkPrice,
    ContractPrice,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MarginType {
    Isolated,
    Cross,
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
    #[serde(rename = "MARKET_LOT_SIZE")]
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename = "MAX_NUM_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { limit: u16 },
    #[serde(rename = "MAX_NUM_ALGO_ORDERS")]
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { limit: u16 },
    #[serde(rename = "MIN_NOTIONAL")]
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(with = "string_or_float")]
        notional: f64,
    },
    #[serde(rename = "PERCENT_PRICE")]
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "string_or_float")]
        multiplier_up: f64,
        #[serde(with = "string_or_float")]
        multiplier_down: f64,
        #[serde(with = "string_or_float")]
        multiplier_decimal: f64,
    },
    #[serde(other)]
    Others,
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

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
// pub enum MarkPrices {
//     AllMarkPrices(Vec<MarkPrice>),
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float")]
    pub index_price: f64,
    #[serde(with = "string_or_float")]
    pub estimated_settle_price: f64,
    #[serde(with = "string_or_float")]
    pub last_funding_rate: f64,
    pub next_funding_time: u64,
    #[serde(with = "string_or_float")]
    pub interest_rate: f64,
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
pub struct Order {
    pub client_order_id: String,
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
    pub side: OrderSide,
    pub reduce_only: bool,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub orig_type: OrderType,
    #[serde(with = "string_or_float", default = "default_activation_price")]
    pub activate_price: f64,
    #[serde(with = "string_or_float", default = "default_price_rate")]
    pub price_rate: f64,
    pub update_time: u64,
    pub working_type: WorkingType,
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
    pub side: OrderSide,
    pub position_side: PositionSide,
    pub status: OrderStatus,
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    pub close_position: bool,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub type_name: OrderType,
    pub orig_type: OrderType,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub activate_price: Option<f64>,
    #[serde(default)]
    #[serde(with = "string_or_float_opt")]
    pub price_rate: Option<f64>,
    pub update_time: u64,
    pub working_type: WorkingType,
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
    pub margin_type: MarginType,
    #[serde(with = "string_or_bool")]
    pub is_auto_add_margin: bool,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
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
    pub position_side: PositionSide,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
}

// https://binance-docs.github.io/apidocs/futures/en/#account-information-v2-user_data
// it has differences from Position returned by positionRisk endpoint
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountPosition {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float", rename = "maintMargin")]
    pub maintenance_margin: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
    pub isolated: bool,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub max_notional: f64,
    #[serde(with = "string_or_float")]
    pub bid_notional: f64,
    #[serde(with = "string_or_float")]
    pub ask_notional: f64,
    pub position_side: PositionSide,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    pub update_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountAsset {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub maint_margin: f64,
    #[serde(with = "string_or_float")]
    pub initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub open_order_initial_margin: f64,
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
pub struct AccountInformation {
    pub fee_tier: u64,
    pub can_trade: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub update_time: u64,
    pub multi_assets_margin: bool,
    #[serde(with = "string_or_float")]
    pub total_initial_margin: f64,
    #[serde(with = "string_or_float", rename = "totalMaintMargin")]
    pub total_maintenance_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub total_margin_balance: f64,
    #[serde(with = "string_or_float")]
    pub total_position_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_open_order_initial_margin: f64,
    #[serde(with = "string_or_float")]
    pub total_cross_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "totalCrossUnPnl")]
    pub total_cross_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub available_balance: f64,
    #[serde(with = "string_or_float")]
    pub max_withdraw_amount: f64,
    pub assets: Vec<AccountAsset>,
    pub positions: Vec<AccountPosition>,
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

fn default_stop_price() -> f64 {
    0.0
}
fn default_activation_price() -> f64 {
    0.0
}
fn default_price_rate() -> f64 {
    0.0
}

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IndexQuery {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: u16,
    pub pair: String,
    pub interval: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FundingRate {
    pub symbol: String,
    pub funding_time: u64,
    #[serde(with = "string_or_float")]
    pub funding_rate: f64,
}

pub static PERIODS: &[&str] = &["5m", "15m", "30m", "1h", "2h", "4h", "6h", "12h", "1d"];

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
    pub cum: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolBrackets {
    pub symbol: String,
    pub notional_coef: Option<f64>,
    pub brackets: Vec<LeverageBracket>,
}
