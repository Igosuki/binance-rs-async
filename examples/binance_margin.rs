#[macro_use]
extern crate tracing;

use env_logger::Builder;

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    info!("running margin endpoints");
    #[cfg(feature = "margin_api")]
    margin_query().await;
    //#[cfg(feature = "margin_api")]
    //margin_post().await;
}

#[cfg(feature = "margin_api")]
async fn margin_query() {
    use binance::api::*;
    use binance::bool_to_string_some;
    use binance::config::Config;
    use binance::margin::Margin;
    use binance::rest_model::*;
    use chrono::{Duration, Utc};
    use std::ops::Sub;

    eprintln!("----------- Margin GET queries ----------");
    let margin: Margin = Binance::new_with_env(&Config::default());
    let yesterday = Utc::now().sub(Duration::days(1));
    let yesterday_millis = yesterday.timestamp_millis() as u64;
    let interest_rate_history = margin
        .interest_rate_history(InterestRateHistoryQuery {
            asset: "BTC".to_string(),
            vip_level: Some(1),
            ..InterestRateHistoryQuery::default()
        })
        .await
        .unwrap();
    eprintln!("interest_rate_history = {interest_rate_history:?}");
    let interest_rate_history = margin
        .interest_rate_history(InterestRateHistoryQuery {
            asset: "LTC".to_string(),
            ..InterestRateHistoryQuery::default()
        })
        .await
        .unwrap();
    eprintln!("interest_rate_history = {interest_rate_history:?}");
    let records_query = RecordsQuery {
        asset: "BTC".to_string(),
        transfer_type: Some(TransferType::RollIn),
        start_time: Some(yesterday_millis),
        ..RecordsQuery::default()
    };
    let loans = margin.loans(records_query).await;
    eprintln!("loans = {loans:?}");
    let records_query = RecordsQuery {
        asset: "BTC".to_string(),
        transfer_type: Some(TransferType::RollIn),
        start_time: Some(yesterday_millis),
        ..RecordsQuery::default()
    };
    let repays = margin.repays(records_query).await;
    eprintln!("repays = {repays:?}");
    let details = margin.details().await;
    eprintln!("details = {details:?}");
    let isolated_details = margin.isolated_details(None).await;
    eprintln!("isolated_details = {isolated_details:?}");
    let isolated_pair = margin.isolated_pair("BTCUSDT").await;
    eprintln!("isolated_pair = {isolated_pair:?}");
    let isolated_account_limit = margin.isolated_account_limit().await;
    eprintln!("isolated_pair = {isolated_account_limit:?}");
    let all_pairs = margin.all_pairs().await;
    eprintln!("all_pairs = {all_pairs:?}");
    let bnb_burn_status = margin.bnb_burn_status().await;
    eprintln!("bnb_burn_status = {bnb_burn_status:?}");
    let asset = margin.asset("BTC").await;
    eprintln!("asset = {asset:?}");
    let pair = margin.pair("BTCUSDT").await;
    eprintln!("pair = {pair:?}");
    let all_assets = margin.all_assets().await;
    eprintln!("all_assets = {all_assets:?}");
    let all_isolated_pairs = margin.all_isolated_pairs().await;
    eprintln!("all_isolated_pairs = {all_isolated_pairs:?}");
    let price_index = margin.price_index("BTCUSDT").await;
    eprintln!("price_index = {price_index:?}");
    let records_query = RecordsQuery {
        asset: "BTC".to_string(),
        transfer_type: Some(TransferType::RollIn),
        ..RecordsQuery::default()
    };
    let transfers = margin.transfers(records_query).await;
    eprintln!("transfers = {transfers:?}");
    let records_query = IsolatedTransfersQuery {
        symbol: "BTC".to_string(),
        ..IsolatedTransfersQuery::default()
    };
    let isolated_transfers = margin.isolated_transfers(records_query).await;
    eprintln!("isolated_transfers = {isolated_transfers:?}");
    let records_query = RecordsQuery {
        asset: "BTC".to_string(),
        transfer_type: Some(TransferType::RollIn),
        ..RecordsQuery::default()
    };
    let interests = margin.interests(records_query).await;
    eprintln!("interests = {interests:?}");
    let records_query = RecordsQuery {
        asset: "BTC".to_string(),
        transfer_type: Some(TransferType::RollIn),
        ..RecordsQuery::default()
    };
    let forced_liquidations = margin.forced_liquidations(records_query).await;
    eprintln!("forced_liquidations = {forced_liquidations:?}");
    let records_query = MarginOrderQuery {
        symbol: "BTCUSDT".to_string(),
        order_id: Some("1".to_string()),
        orig_client_order_id: Some("my_id".to_string()),
        is_isolated: None,
    };
    let order = margin.order(records_query).await;
    eprintln!("order = {order:?}");
    let open_orders = margin.open_orders("BTCUSDT", None).await;
    eprintln!("open_orders = {open_orders:?}");
    let records_query = MarginOrdersQuery {
        symbol: "BTCUSDT".to_string(),
        ..MarginOrdersQuery::default()
    };
    let orders = margin.orders(records_query).await;
    eprintln!("orders = {orders:?}");
    let records_query = MarginOwnTradesQuery {
        symbol: "BTCUSDT".to_string(),
        ..MarginOwnTradesQuery::default()
    };
    let trades = margin.trades(records_query).await;
    eprintln!("trades = {trades:?}");
    let records_query = MarginOCOOrderQuery {
        symbol: Some("BTCUSDT".to_string()),
        is_isolated: bool_to_string_some(true),
        orig_client_order_id: Some("id".to_string()),
        ..MarginOCOOrderQuery::default()
    };
    let oco_order = margin.oco_order(records_query).await;
    eprintln!("oco_order = {oco_order:?}");
    let records_query = OCORecordsQuery {
        symbol: Some("BTCUSDT".to_string()),
        is_isolated: bool_to_string_some(true),
        ..OCORecordsQuery::default()
    };
    let all_oco_orders = margin.all_oco_orders(records_query).await;
    eprintln!("all_oco_orders = {all_oco_orders:?}");
    let max_borrowable = margin.max_borrowable("BTC", None).await;
    eprintln!("max_borrowable = {max_borrowable:?}");
    let max_transferable = margin.max_transferable("BTC", None).await;
    eprintln!("max_transferable = {max_transferable:?}");
}

#[allow(dead_code)]
#[cfg(feature = "margin_api")]
async fn margin_post() {
    use binance::api::*;
    use binance::config::Config;
    use binance::margin::Margin;
    use binance::rest_model::*;

    eprintln!("----------- Margin POST queries ----------");
    let margin: Margin = Binance::new_with_env(&Config::testnet());

    let transfer = margin
        .transfer("BTC", 0.001, MarginTransferType::FromMainToMargin)
        .await;
    eprintln!("transfer = {transfer:?}");
    let isolated_transfer = margin
        .isolated_transfer(
            "BTC",
            "ETH",
            0.001,
            IsolatedMarginTransferType::Spot,
            IsolatedMarginTransferType::IsolatedMargin,
        )
        .await;
    eprintln!("isolated_transfer = {isolated_transfer:?}");
    let loan = margin.loan("BTC", 0.001).await;
    eprintln!("loan = {loan:?}");
    let loan_with_isolation = margin
        .loan_with_isolation("BTC", 0.001, Some(true), Some("BNB".to_string()))
        .await;
    eprintln!("loan_with_isolation = {loan_with_isolation:?}");
    let repay = margin.repay("BTC", 0.001).await;
    eprintln!("repay = {repay:?}");
    let repay_with_isolation = margin
        .repay_with_isolation("BTCUSDT", 0.001, Some(true), Some("BNB".to_string()))
        .await;
    eprintln!("repay_with_isolation = {repay_with_isolation:?}");
    let margin_order = MarginOrder {
        symbol: "BTCUSDT".to_string(),
        side: OrderSide::Sell,
        order_type: OrderType::Limit,
        quantity: Some(0.001),
        quote_order_qty: None,
        price: Some(10.0),
        stop_price: Some(10.0),
        new_client_order_id: Some("my_id".to_string()),
        iceberg_qty: Some(10.0),
        new_order_resp_type: OrderResponse::Ack,
        time_in_force: Some(TimeInForce::FOK),
        side_effect_type: SideEffectType::NoSideEffect,
        is_isolated: None,
    };
    let new_order = margin.new_order(margin_order).await;
    eprintln!("new_order = {new_order:?}");

    let cancel_trade = margin
        .cancel_trade("BTCUSDT", 1_u64, "my_id".to_string(), "my_next_id".to_string(), None)
        .await;
    eprintln!("cancel_trade = {cancel_trade:?}");
    let cancel_oco_order = margin
        .cancel_oco_order("BTCUSDT", 1_u64, "my_id".to_string(), "my_next_id".to_string(), None)
        .await;
    eprintln!("cancel_oco_order = {cancel_oco_order:?}");
    let cancel_all_orders = margin.cancel_all_orders("BTCUSDT", None).await;
    eprintln!("cancel_all_orders = {cancel_all_orders:?}");
    let disable_isolated = margin.disable_isolated("BTCUSDT".to_string()).await;
    eprintln!("disable_isolated = {disable_isolated:?}");
    let enable_isolated = margin.enable_isolated("BTCUSDT".to_string()).await;
    eprintln!("enable_isolated = {enable_isolated:?}");
    let toggle_bnb_burn = margin.toggle_bnb_burn(BnbBurnQuery::default()).await;
    eprintln!("toggle_bnb_burn = {toggle_bnb_burn:?}");
}
