#[macro_use]
extern crate log;

use env_logger::Builder;

use binance::account::*;
use binance::api::*;
use binance::config::Config;
use binance::errors::Error as BinanceLibError;
use binance::futures::rest_model::SymbolPrice;
use binance::general::*;
use binance::margin::Margin;
use binance::market::*;
use binance::rest_model::{InterestRateHistoryQuery, OrderSide, OrderType, TimeInForce};

#[tokio::main]
async fn main() {
    Builder::new().parse_default_env().init();
    info!("running margin endpoints");
    margin().await;
}

async fn margin() {
    let margin: Margin = Binance::new(None, None);
    let interest_rate_history = margin
        .interest_rate_history(InterestRateHistoryQuery {
            asset: "BTC".to_string(),
            vip_level: None,
            start_time: None,
            end_time: None,
            limit: None,
        })
        .await
        .unwrap();
    eprintln!("interest_rate_history = {:?}", interest_rate_history);
}
