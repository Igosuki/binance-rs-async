pub use crate::rest_model::{string_or_float, string_or_u64, OrderSide, OrderStatus, TimeInForce};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub uni_mmr: String,
    pub account_equity: String,
    pub actual_equity: String,
    pub account_maint_margin: String,
    pub account_equity_usd: String,
    pub account_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    pub asset: String,
    #[serde(with = "string_or_float")]
    pub total_wallet_balance: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_asset: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_borrowed: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_free: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_interest: f64,
    #[serde(with = "string_or_float")]
    pub cross_margin_locked: f64,
    #[serde(with = "string_or_float")]
    pub um_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "umUnrealizedPNL")]
    pub um_unrealized_pnl: f64,
    #[serde(with = "string_or_float")]
    pub cm_wallet_balance: f64,
    #[serde(with = "string_or_float", rename = "cmUnrealizedPNL")]
    pub cm_unrealized_pnl: f64,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub negative_balance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UmPosition {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
    pub position_side: String,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    pub margin_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CmPosition {
    pub symbol: String,
    #[serde(with = "string_or_float")]
    pub entry_price: f64,
    #[serde(with = "string_or_float")]
    pub mark_price: f64,
    #[serde(with = "string_or_float", rename = "positionAmt")]
    pub position_amount: f64,
    #[serde(with = "string_or_float", rename = "unRealizedProfit")]
    pub unrealized_profit: f64,
    #[serde(with = "string_or_float")]
    pub liquidation_price: f64,
    #[serde(with = "string_or_u64")]
    pub leverage: u64,
    pub position_side: String,
    pub update_time: u64,
    #[serde(with = "string_or_float")]
    pub notional: f64,
    #[serde(with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(with = "string_or_float")]
    pub isolated_margin: f64,
    pub margin_type: String,
    #[serde(with = "string_or_float")]
    pub max_qty: f64,
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

impl Default for OrderType {
    fn default() -> Self { Self::Market }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UmOrder {
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
    pub side: OrderSide,
    pub position_side: String,
    pub status: OrderStatus,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub symbol: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub update_time: u64,
    pub working_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CmOrder {
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub cum_qty: f64,
    #[serde(with = "string_or_float")]
    pub cum_base: f64,
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
    pub position_side: String,
    pub status: OrderStatus,
    #[serde(with = "string_or_float", default = "default_stop_price")]
    pub stop_price: f64,
    pub symbol: String,
    pub pair: String,
    pub time_in_force: TimeInForce,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub update_time: u64,
    pub working_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrder {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub orig_qty: f64,
    #[serde(with = "string_or_float")]
    pub executed_qty: f64,
    #[serde(with = "string_or_float")]
    pub cummulative_quote_qty: f64,
    pub status: String,
    pub time_in_force: String,
    pub side: String,
    #[serde(rename = "type")]
    pub order_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaxBorrowable {
    #[serde(with = "string_or_float")]
    pub amount: f64,
    #[serde(with = "string_or_float")]
    pub borrowable_limit: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepayResult {
    pub tran_id: u64,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UmOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<String>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub working_type: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CmOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<String>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub stop_price: Option<f64>,
    pub working_type: Option<String>,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrderRequest {
    pub symbol: String,
    pub side: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub quantity: Option<f64>,
    pub quote_order_qty: Option<f64>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub new_client_order_id: Option<String>,
    pub time_in_force: Option<String>,
    pub side_effect_type: Option<String>,
}

fn default_stop_price() -> f64 { 0.0 }
