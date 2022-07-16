#[macro_use]
extern crate tracing;

use binance::account::*;
use binance::api::*;
use binance::config::Config;
use binance::errors::Error as BinanceLibError;
use binance::general::*;
use binance::market::*;
use binance::rest_model::{OrderSide, OrderType, SymbolPrice, TimeInForce};
use env_logger::Builder;

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    info!("running general endpoints");
    general().await;
    info!("running market endpoints");
    market_data().await;
    info!("running account (private) endpoints");
    account().await;
}

async fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping().await;
    match ping {
        Ok(answer) => info!("{:?}", answer),
        Err(err) => {
            match err {
                BinanceLibError::BinanceError { response } => match response.code {
                    -1000_i32 => error!("An unknown error occured while processing the request"),
                    _ => error!("Non-catched code {}: {}", response.code, response.msg),
                },
                _ => error!("Other errors: {}.", err),
            };
        }
    }

    let result = general.get_server_time().await;
    match result {
        Ok(answer) => info!("Server Time: {}", answer.server_time),
        Err(e) => error!("Error: {}", e),
    }

    let result = general.exchange_info().await;
    match result {
        Ok(answer) => info!("Exchange information: {:?}", answer),
        Err(e) => error!("Error: {}", e),
    }
}

async fn account() {
    let market: Market = Binance::new(None, None);
    let account: Account = Binance::new_with_env(&Config::testnet());
    let symbol = "BTCUSDT";
    let SymbolPrice { price, .. } = market.get_price(symbol).await.unwrap();
    match account.get_account().await {
        Ok(answer) => info!("{:?}", answer.balances),
        Err(e) => error!("Error: {}", e),
    }

    match account.get_open_orders(symbol).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let limit_buy = OrderRequest {
        symbol: symbol.to_string(),
        quantity: Some(0.001),
        price: Some(price),
        order_type: OrderType::Limit,
        side: OrderSide::Buy,
        time_in_force: Some(TimeInForce::FOK),
        ..OrderRequest::default()
    };
    match account.place_order(limit_buy).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let market_buy = OrderRequest {
        symbol: symbol.to_string(),
        quantity: Some(0.001),
        order_type: OrderType::Market,
        side: OrderSide::Buy,
        ..OrderRequest::default()
    };
    match account.place_order(market_buy).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let limit_sell = OrderRequest {
        symbol: symbol.to_string(),
        quantity: Some(0.001),
        price: Some(price),
        order_type: OrderType::Limit,
        side: OrderSide::Sell,
        time_in_force: Some(TimeInForce::FOK),
        ..OrderRequest::default()
    };
    match account.place_order(limit_sell).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let market_sell = OrderRequest {
        symbol: symbol.to_string(),
        quantity: Some(0.001),
        order_type: OrderType::Market,
        side: OrderSide::Sell,
        ..OrderRequest::default()
    };
    match account.place_order(market_sell).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let order_id = 1_957_528;
    let order_status = OrderStatusRequest {
        symbol: symbol.to_string(),
        order_id: Some(order_id),
        ..OrderStatusRequest::default()
    };

    match account.order_status(order_status).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    let order_cancellation = OrderCancellation {
        symbol: symbol.to_string(),
        order_id: Some(order_id),
        ..OrderCancellation::default()
    };

    match account.cancel_order(order_cancellation).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    match account.get_balance("BTC").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    match account.trade_history(symbol).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }
}

async fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices().await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH").await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers().await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => info!("Bid Price: {}, Ask Price: {}", answer.bid_price, answer.ask_price),
        Err(e) => error!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => info!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => error!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(answer) => info!("{:?}", answer),
        Err(e) => error!("Error: {}", e),
    }

    // 10 latest (aggregated) trades
    match market.get_agg_trades("BNBETH", None, None, None, Some(10)).await {
        Ok(trades) => {
            let trade = &trades[0]; // You need to iterate over them
            println!(
                "{} BNB Qty: {}, Price: {}",
                if trade.maker { "SELL" } else { "BUY" },
                trade.qty,
                trade.price
            )
        }
        Err(e) => println!("Error: {}", e),
    }
}
