use crate::rest_model::{
    string_or_float, Asks, Bids, ExecutionType, OrderBook, OrderSide, OrderStatus, OrderType, TimeInForce,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "e")]
pub enum WebsocketEvent {
    #[serde(alias = "aggTrade")]
    AggTrade(Box<TradesEvent>),
    #[serde(alias = "trade")]
    Trade(Box<TradeEvent>),
    #[serde(alias = "kline")]
    Kline(Box<KlineEvent>),
    #[serde(alias = "24hrTicker")]
    DayTicker(Box<DayTickerEvent>),
    #[serde(alias = "24hrMiniTicker")]
    DayMiniTicker(Box<MiniDayTickerEvent>),
    #[serde(alias = "depthUpdate")]
    DepthOrderBook(Box<DepthOrderBookEvent>),
    #[serde(alias = "outboundAccountPosition")]
    AccountPositionUpdate(Box<AccountPositionUpdate>),
    #[serde(alias = "balanceUpdate")]
    BalanceUpdate(Box<BalanceUpdate>),
    #[serde(alias = "executionReport")]
    OrderUpdate(Box<OrderUpdate>),
    #[serde(alias = "listStatus")]
    ListOrderUpdate(Box<OrderListUpdate>),
    #[serde(alias = "markPriceUpdate")]
    MarkPriceUpdate(Box<MarkPriceEvent>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResult {
    pub result: Option<String>,
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradesEvent {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "f")]
    pub first_break_trade_id: u64,

    #[serde(rename = "l")]
    pub last_break_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeEvent {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub price: String,

    #[serde(rename = "q")]
    pub qty: String,

    #[serde(rename = "b")]
    pub buyer_order_id: u64,

    #[serde(rename = "a")]
    pub seller_order_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DayTickerEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p")]
    pub price_change: String,
    #[serde(rename = "P")]
    pub price_change_percent: String,
    #[serde(rename = "w")]
    pub average_price: String,
    #[serde(rename = "x")]
    pub prev_close: String,
    #[serde(rename = "c")]
    pub current_close: String,
    #[serde(rename = "Q")]
    pub current_close_qty: String,
    #[serde(rename = "b")]
    pub best_bid: String,
    #[serde(rename = "B")]
    pub best_bid_qty: String,
    #[serde(rename = "a")]
    pub best_ask: String,
    #[serde(rename = "A")]
    pub best_ask_qty: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniDayTickerEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub current_close: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "q")]
    pub quote_volume: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i64,
    #[serde(rename = "L")]
    pub last_trade_id: i64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "n")]
    pub number_of_trades: i64,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(with = "string_or_float")]
    #[serde(rename = "q")]
    pub quote_volume: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "V")]
    pub active_buy_volume: f64,
    #[serde(with = "string_or_float")]
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: f64,
    #[serde(skip, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderBookEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "U")]
    pub first_update_id: u64,
    #[serde(rename = "u")]
    pub final_update_id: u64,
    #[serde(rename = "b")]
    pub bids: Vec<Bids>,
    #[serde(rename = "a")]
    pub asks: Vec<Asks>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerEvent {
    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b", with = "string_or_float")]
    pub best_bid: f64,

    #[serde(rename = "B", with = "string_or_float")]
    pub best_bid_qty: f64,

    #[serde(rename = "a", with = "string_or_float")]
    pub best_ask: f64,

    #[serde(rename = "A", with = "string_or_float")]
    pub best_ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarkPriceEvent {
    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub mark_price: String,

    #[serde(rename = "i")]
    pub index_price: String,

    #[serde(rename = "P")]
    pub estimated_settle_price: String,

    #[serde(rename = "r")]
    pub funding_rate: String,

    #[serde(rename = "T")]
    pub next_funding_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombinedStreamEvent<T> {
    stream: String,
    pub data: T,
}

///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebsocketEventUntag {
    WebsocketEvent(WebsocketEvent),
    Orderbook(Box<OrderBook>),
    BookTicker(Box<BookTickerEvent>),
}

impl<T> CombinedStreamEvent<T> {
    /// Returns (stream_name, channel)
    pub fn parse_stream(&self) -> (String, String) {
        let mut parsed = self.stream.clone();
        if let Some(0) = parsed.find('!') {
            parsed.remove(0);
        }
        let split = parsed.split_once('@').unwrap_or((&parsed, ""));
        (split.0.to_string(), split.1.to_string())
    }
}

/// User Stream related events

/// Account position update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountPositionUpdate {
    #[serde(alias = "E")]
    pub event_time: u64,

    #[serde(alias = "u")]
    pub last_update_time: u64,

    #[serde(alias = "B")]
    pub balances: Vec<EventBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(alias = "E")]
    pub event_time: u64,

    /// Maker commission rate (bips)
    #[serde(alias = "m")]
    maker_commission_rate: u64,
    /// Taker commission rate (bips)
    #[serde(alias = "t")]
    taker_commission_rate: u64,
    /// Buyer commission rate (bips)
    #[serde(alias = "b")]
    buyer_commission_rate: u64,
    /// Seller commission rate (bips)
    #[serde(alias = "s")]
    seller_commission_rate: u64,

    #[serde(alias = "T")]
    can_trade: bool,
    #[serde(alias = "W")]
    can_withdraw: bool,
    #[serde(alias = "D")]
    can_deposit: bool,

    #[serde(alias = "B")]
    pub balances: Vec<EventBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f")]
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(rename = "l")]
    #[serde(with = "string_or_float")]
    pub locked: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BalanceUpdate {
    #[serde(alias = "E")]
    pub event_time: u64,

    #[serde(rename = "a")]
    pub asset: String,

    #[serde(rename = "d")]
    #[serde(with = "string_or_float")]
    pub delta: f64,

    #[serde(alias = "T")]
    pub clear_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub client_order_id: Option<String>,
    #[serde(rename = "S")]
    pub side: OrderSide,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q")]
    #[serde(with = "string_or_float")]
    pub qty: f64,
    #[serde(rename = "p")]
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(rename = "P")]
    #[serde(with = "string_or_float")]
    pub stop_price: f64,
    #[serde(rename = "F")]
    #[serde(with = "string_or_float")]
    pub iceberg_qty: f64,
    #[serde(rename = "g")]
    pub order_list_id: i64,
    #[serde(rename = "C")]
    pub origin_client_id: Option<String>,
    #[serde(rename = "x")]
    pub execution_type: ExecutionType,
    #[serde(rename = "X")]
    pub current_order_status: OrderStatus,
    #[serde(rename = "r")]
    pub order_reject_reason: String,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l")]
    #[serde(with = "string_or_float")]
    pub qty_last_executed: f64,
    #[serde(rename = "z")]
    #[serde(with = "string_or_float")]
    pub cumulative_filled_qty: f64,
    #[serde(rename = "L")]
    #[serde(with = "string_or_float")]
    pub last_executed_price: f64,
    #[serde(rename = "n")]
    #[serde(with = "string_or_float")]
    pub commission: f64,
    #[serde(rename = "N")]
    pub commission_asset: Option<String>,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(skip, rename = "I")]
    pub i_ignore: u64,
    #[serde(rename = "w")]
    pub is_order_on_the_book: bool,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip, rename = "M")]
    pub m_ignore: bool,
    #[serde(rename = "O")]
    pub order_creation_time: u64,
    #[serde(rename = "Z")]
    #[serde(with = "string_or_float")]
    pub cumulative_quote_asset_transacted_qty: f64,
    /// (i.e. lastPrice * lastQty)
    #[serde(rename = "Y")]
    #[serde(with = "string_or_float")]
    pub last_quote_asset_transacted_qty: f64,
    #[serde(rename = "Q")]
    #[serde(with = "string_or_float")]
    pub quote_order_qty: f64,
}

/// For OCO Events
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderListUpdate {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "g")]
    order_list_id: i64,
    #[serde(rename = "c")]
    contingency_type: String,
    #[serde(rename = "l")]
    list_status_type: String,
    #[serde(rename = "L")]
    list_order_status: String,
    #[serde(rename = "r")]
    list_reject_reason: String,
    #[serde(rename = "C")]
    list_client_order_id: String,
    #[serde(rename = "T")]
    pub transaction_time: u64,
    #[serde(rename = "O")]
    pub objects: Vec<OrderListTransaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderListTransaction {
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub order_id: i64,
    #[serde(rename = "c")]
    pub client_order_id: String,
}
