extern crate binance;

use binance::account::*;
use binance::api::*;
use binance::errors::ErrorKind as BinanceLibErrorKind;
use binance::general::*;
use binance::market::*;

fn main() {
    futures::executor::block_on(general());
    //account();
    //market_data();
}

async fn general() {
    let general: General = Binance::new(None, None);

    let ping = general.ping().await;
    match ping {
        Ok(answer) => println!("{:?}", answer),
        Err(err) => {
            match err.0 {
                BinanceLibErrorKind::BinanceError(response) => match response.code {
                    -1000_i16 => println!("An unknown error occured while processing the request"),
                    _ => println!("Non-catched code {}: {}", response.code, response.msg),
                },
                BinanceLibErrorKind::Msg(msg) => println!("Binancelib error msg: {}", msg),
                _ => println!("Other errors: {}.", err.0),
            };
        }
    }

    let result = general.get_server_time().await;
    match result {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {}", e),
    }

    let result = general.exchange_info().await;
    match result {
        Ok(answer) => println!("Exchange information: {:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
async fn account() {
    let api_key = Some("YOUR_API_KEY".into());
    let secret_key = Some("YOUR_SECRET_KEY".into());

    let account: Account = Binance::new(api_key, secret_key);

    match account.get_account().await {
        Ok(answer) => println!("{:?}", answer.balances),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_open_orders("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_buy("WTCETH", 10, 0.014000).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_buy("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.limit_sell("WTCETH", 10, 0.035000).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.market_sell("WTCETH", 5).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    let order_id = 1_957_528;
    match account.order_status("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.cancel_order("WTCETH", order_id).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.get_balance("KNC").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    match account.trade_history("WTCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}

#[allow(dead_code)]
async fn market_data() {
    let market: Market = Binance::new(None, None);

    // Order book
    match market.get_depth("BNBETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ALL symbols
    match market.get_all_prices().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Latest price for ONE symbol
    match market.get_price("KNCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Current average price for ONE symbol
    match market.get_average_price("KNCETH").await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ALL symbols
    match market.get_all_book_tickers().await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }

    // Best price/qty on the order book for ONE symbol
    match market.get_book_ticker("BNBETH").await {
        Ok(answer) => println!(
            "Bid Price: {}, Ask Price: {}",
            answer.bid_price, answer.ask_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // 24hr ticker price change statistics
    match market.get_24h_price_stats("BNBETH").await {
        Ok(answer) => println!(
            "Open Price: {}, Higher Price: {}, Lower Price: {:?}",
            answer.open_price, answer.high_price, answer.low_price
        ),
        Err(e) => println!("Error: {}", e),
    }

    // last 10 5min klines (candlesticks) for a symbol:
    match market.get_klines("BNBETH", "5m", 10, None, None).await {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {}", e),
    }
}
