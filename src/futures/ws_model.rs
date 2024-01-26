use crate::futures::rest_model::{MarginType, OrderType, PositionSide, WorkingType};
use crate::rest_model::{string_or_float, string_or_float_opt, ExecutionType, OrderSide, OrderStatus, TimeInForce};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "e")]
pub enum WebsocketEvent {
    AccountUpdate(Box<AccountUpdate>),
    OrderTradeUpdate(Box<OrderTradeUpdate>),
}

#[derive(Debug, Deserialize)]
pub struct AccountUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "a")]
    pub account: Account,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "m")]
    pub reason_type: ReasonType,
    #[serde(rename = "B")]
    pub balances: Vec<Balance>,
    #[serde(rename = "P")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReasonType {
    Deposit,
    Withdraw,
    Order,
    FundingFee,
    WithdrawReject,
    Adjustment,
    InsuranceClear,
    AdminDeposit,
    AdminWithdraw,
    MarginTransfer,
    MarginTypeChange,
    AssetTransfer,
    OptionsPremiumFee,
    OptionsSettleProfit,
    AutoExchange,
    CoinSwapDeposit,
    CoinSwapWithdraw,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "wb", with = "string_or_float")]
    pub wallet_balance: f64,
    #[serde(rename = "cw", with = "string_or_float")]
    pub cross_wallet_balance: f64,
    #[serde(rename = "bc", with = "string_or_float")]
    pub balance_change: f64,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "pa", with = "string_or_float")]
    pub position_amount: f64,
    #[serde(rename = "ep", with = "string_or_float")]
    pub entry_price: f64,
    #[serde(rename = "bep", with = "string_or_float")]
    pub breakeven_price: f64,
    #[serde(rename = "cr", with = "string_or_float")]
    pub accumulated_realized: f64,
    #[serde(rename = "up", with = "string_or_float")]
    pub unrealized_profit: f64,
    #[serde(rename = "mt")]
    pub margin_type: MarginType,
    #[serde(rename = "iw", with = "string_or_float")]
    pub isolated_wallet: f64,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderTradeUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "o")]
    pub order: Order,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Order {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: String,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q", with = "string_or_float")]
    pub quantity: f64,
    #[serde(rename = "p", with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "ap", with = "string_or_float")]
    pub average_price: f64,
    #[serde(rename = "sp", with = "string_or_float")]
    pub stop_price: f64,
    #[serde(rename = "x")]
    pub execution_type: ExecutionType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_or_float")]
    pub order_last_filled_quantity: f64,
    #[serde(rename = "z", with = "string_or_float")]
    pub order_filled_accumulated_quantity: f64,
    #[serde(rename = "L", with = "string_or_float")]
    pub last_filled_price: f64,
    #[serde(default, rename = "n", with = "string_or_float_opt")]
    pub commission: Option<f64>,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub order_trade_time: u64,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "b", with = "string_or_float")]
    pub bid_notional: f64,
    #[serde(rename = "a", with = "string_or_float")]
    pub ask_notional: f64,
    #[serde(rename = "m")]
    pub is_maker: bool,
    #[serde(rename = "R")]
    pub is_reduce: bool,
    #[serde(rename = "wt")]
    pub working_type: WorkingType,
    #[serde(rename = "ot")]
    pub original_order_type: OrderType,
    #[serde(rename = "ps")]
    pub position_side: PositionSide,
    #[serde(rename = "cp")]
    pub close_position: bool,
    #[serde(default, rename = "AP", with = "string_or_float_opt")]
    pub activation_price: Option<f64>,
    #[serde(default, rename = "cr", with = "string_or_float_opt")]
    pub callback_rate: Option<f64>,
    #[serde(rename = "pP")]
    pub price_protect: bool,
    #[serde(rename = "rp", with = "string_or_float")]
    pub realized_profit: f64,
    #[serde(rename = "V")]
    pub stp_mode: SelfTradePreventionMode,
    #[serde(rename = "pm")]
    pub price_match: PriceMatch,
    #[serde(rename = "gtd")]
    pub good_till_date: u64
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriceMatch {
    /// No price match
    None,
    /// Counterparty best price
    Opponent,
    /// The 5th best price from the counterparty
    Opponent5,
    /// The 10th best price from the counterparty
    Opponent10,
    /// The 20th best price from the counterparty
    Opponent20,
    /// The best price on the same side of the order book
    Queue,
    /// The 5th best price on the same side of the order book
    Queue5,
    /// The 10th best price on the same side of the order book
    Queue10,
    /// The 20th best price on the same side of the order book
    Queue20,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelfTradePreventionMode {
    /// No Self-Trade Prevention
    None,
    /// Expire taker order when STP trigger
    ExpireTaker,
    /// Expire taker and maker order when STP trigger
    ExpireBoth,
    /// Expire maker order when STP trigger
    ExpireMaker,
}
