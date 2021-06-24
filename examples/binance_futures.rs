use binance::api::*;
use binance::errors::Error as BinanceLibError;
use binance::futures::general::*;
use binance::futures::market::*;
use binance::futures::model::*;

fn main() {
    futures::executor::block_on(general());
    futures::executor::block_on(market_data());
}

async fn general() {
    let general: FuturesGeneral = Binance::new(None, None);

    match general.ping().await {
        Ok(answer) => println!("{:?}", answer),
        Err(err) => {
            match err {
                BinanceLibError::BinanceError { response } => match response.code {
                    -1000_i16 => println!("An unknown error occured while processing the request"),
                    _ => println!("Uncaught code {}: {}", response.code, response.msg),
                },
                BinanceLibError::Msg(msg) => println!("Binancelib error msg: {}", msg),
                _ => println!("Other errors: {:?}.", err),
            };
        }
    }

    match general.get_server_time().await {
        Ok(answer) => println!("Server Time: {}", answer.server_time),
        Err(e) => println!("Error: {:?}", e),
    }

    match general.exchange_info().await {
        Ok(answer) => println!("Exchange information: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match general.get_symbol_info("btcusdt").await {
        Ok(answer) => println!("Symbol information: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}

async fn market_data() {
    let market: FuturesMarket = Binance::new(None, None);

    match market.get_depth("btcusdt").await {
        Ok(answer) => println!("Depth update ID: {:?}", answer.last_update_id),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_trades("btcusdt").await {
        Ok(Trades::AllTrades(answer)) => println!("First trade: {:?}", answer[0]),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_agg_trades("btcusdt", None, None, None, 500u16).await {
        Ok(AggTrades::AllAggTrades(answer)) => println!("First aggregated trade: {:?}", answer[0]),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_klines("btcusdt", "5m", 10u16, None, None).await {
        Ok(KlineSummaries::AllKlineSummaries(answer)) => println!("First kline: {:?}", answer[0]),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_24h_price_stats("btcusdt").await {
        Ok(answer) => println!("24hr price stats: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_price("btcusdt").await {
        Ok(answer) => println!("Price: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_all_book_tickers().await {
        Ok(BookTickers::AllBookTickers(answer)) => println!("First book ticker: {:?}", answer[0]),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_book_ticker("btcusdt").await {
        Ok(answer) => println!("Book ticker: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_mark_prices().await {
        Ok(MarkPrices::AllMarkPrices(answer)) => println!("First mark Prices: {:?}", answer[0]),
        Err(e) => println!("Error: {:?}", e),
    }

    match market.get_all_liquidation_orders().await {
        Ok(LiquidationOrders::AllLiquidationOrders(answer)) => {
            println!("First liquidation order: {:?}", answer[0])
        }
        Err(e) => println!("Error: {:?}", e),
    }

    match market.open_interest("btcusdt").await {
        Ok(answer) => println!("Open interest: {:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}
