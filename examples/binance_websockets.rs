#[macro_use]
extern crate tokio;

use binance::api::*;
use binance::userstream::*;
use binance::websockets::*;
use binance::ws_model::{CombinedStreamEvent, WebsocketEvent, WebsocketEventUntag};
use futures::future::BoxFuture;
use futures::stream::StreamExt;
use serde_json::from_str;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let (logger_tx, mut logger_rx) = tokio::sync::mpsc::unbounded_channel::<WebsocketEvent>();
    let (close_tx, mut close_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
    let wait_loop = tokio::spawn(async move {
        'hello: loop {
            select! {
                event = logger_rx.recv() => println!("{event:?}"),
                _ = close_rx.recv() => break 'hello
            }
        }
    });
    // private api
    //user_stream().await;
    //user_stream_websocket().await;
    // public api
    let streams: Vec<BoxFuture<'static, ()>> = vec![
        Box::pin(market_websocket(logger_tx.clone())),
        Box::pin(kline_websocket(logger_tx.clone())),
        Box::pin(all_trades_websocket(logger_tx.clone())),
        Box::pin(last_price(logger_tx.clone())),
        Box::pin(book_ticker(logger_tx.clone())),
        Box::pin(combined_orderbook(logger_tx.clone())),
        Box::pin(custom_event_loop(logger_tx.clone())),
        Box::pin(mark_price_websocket(logger_tx)),
    ];

    for stream in streams {
        tokio::spawn(stream);
    }

    select! {
        _ = wait_loop => { println!("Finished!") }
        _ = tokio::signal::ctrl_c() => {
            println!("Closing websocket stream...");
            close_tx.send(true).unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

#[allow(dead_code)]
async fn user_stream() {
    let api_key_user = Some("YOUR_API_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user.clone(), None);

    if let Ok(answer) = user_stream.start().await {
        println!("Data Stream Started ...");
        let listen_key = answer.listen_key;

        match user_stream.keep_alive(&listen_key).await {
            Ok(msg) => println!("Keepalive user data stream: {msg:?}"),
            Err(e) => println!("Error: {e}"),
        }

        match user_stream.close(&listen_key).await {
            Ok(msg) => println!("Close user data stream: {msg:?}"),
            Err(e) => println!("Error: {e}"),
        }
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

#[allow(dead_code)]
async fn user_stream_websocket() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let api_key_user = Some("YOUR_KEY".into());
    let user_stream: UserStream = Binance::new(api_key_user, None);

    if let Ok(answer) = user_stream.start().await {
        let listen_key = answer.listen_key;

        let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
            if let WebsocketEvent::OrderUpdate(trade) = event {
                println!(
                    "Symbol: {}, Side: {:?}, Price: {}, Execution Type: {:?}",
                    trade.symbol, trade.side, trade.price, trade.execution_type
                );
            };

            Ok(())
        });

        web_socket.connect(&listen_key).await.unwrap(); // check error
        if let Err(e) = web_socket.event_loop(&keep_running).await {
            println!("Error: {e}");
        }
        user_stream.close(&listen_key).await.unwrap();
        web_socket.disconnect().await.unwrap();
        println!("Userstrem closed and disconnected");
    } else {
        println!("Not able to start an User Stream (Check your API_KEY)");
    }
}

#[allow(dead_code)]
async fn market_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = agg_trade_stream("ethbtc");
    let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
        logger_tx.send(event.clone()).unwrap();
        match event {
            WebsocketEvent::Trade(trade) => {
                println!("Symbol: {}, price: {}, qty: {}", trade.symbol, trade.price, trade.qty);
            }
            WebsocketEvent::DepthOrderBook(depth_order_book) => {
                println!(
                    "Symbol: {}, Bids: {:?}, Ask: {:?}",
                    depth_order_book.symbol, depth_order_book.bids, depth_order_book.asks
                );
            }
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn all_trades_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade = all_ticker_stream();
    // NB: you may not ask for both arrays type streams and object type streams at the same time, this holds true in binance connections anyways
    // You cannot connect to multiple things for a single socket
    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> = WebSockets::new(|events: Vec<WebsocketEvent>| {
        for tick_events in events {
            logger_tx.send(tick_events.clone()).unwrap();
            if let WebsocketEvent::DayTicker(tick_event) = tick_events {
                println!(
                    "Symbol: {}, price: {}, qty: {}",
                    tick_event.symbol, tick_event.best_bid, tick_event.best_bid_qty
                );
            }
        }

        Ok(())
    });

    web_socket.connect(agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn kline_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let kline = kline_stream("ethbtc", "1m");
    let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
        logger_tx.send(event.clone()).unwrap();
        if let WebsocketEvent::Kline(kline_event) = event {
            println!(
                "Symbol: {}, high: {}, low: {}",
                kline_event.kline.symbol, kline_event.kline.low, kline_event.kline.high
            );
        }

        Ok(())
    });

    web_socket.connect(&kline).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn mark_price_websocket(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let mark_price = mark_price_stream("btcusdt", 1);
    let mut web_socket: WebSockets<'_, WebsocketEvent> = WebSockets::new(|event: WebsocketEvent| {
        logger_tx.send(event.clone()).unwrap();
        if let WebsocketEvent::MarkPriceUpdate(mark_price_event) = event {
            println!(
                "Symbol: {}, mark price: {}",
                mark_price_event.symbol, mark_price_event.mark_price
            );
        }

        Ok(())
    });

    if let Err(e) = web_socket.connect_futures(&mark_price).await {
        println!("Connection error: {e}");
    }

    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn last_price(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let all_ticker = all_ticker_stream();
    let btcusdt: RwLock<f32> = RwLock::new("0".parse().unwrap());

    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> = WebSockets::new(|events: Vec<WebsocketEvent>| {
        for tick_events in events {
            logger_tx.send(tick_events.clone()).unwrap();
            if let WebsocketEvent::DayTicker(tick_event) = tick_events {
                if tick_event.symbol == "BTCUSDT" {
                    let mut btcusdt = btcusdt.write().unwrap();
                    *btcusdt = tick_event.average_price.parse::<f32>().unwrap();
                    let btcusdt_close: f32 = tick_event.current_close.parse().unwrap();
                    println!("{btcusdt} - {btcusdt_close}");

                    if btcusdt_close as i32 == 7000 {
                        // Break the event loop
                        keep_running.store(false, Ordering::Relaxed);
                    }
                }
            }
        }

        Ok(())
    });

    web_socket.connect(all_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn book_ticker(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let book_ticker: String = book_ticker_stream("btcusdt");

    let mut web_socket: WebSockets<'_, WebsocketEventUntag> = WebSockets::new(|events: WebsocketEventUntag| {
        if let WebsocketEventUntag::WebsocketEvent(we) = &events {
            logger_tx.send(we.clone()).unwrap();
        }
        if let WebsocketEventUntag::BookTicker(tick_event) = events {
            println!("{tick_event:?}")
        }
        Ok(())
    });

    web_socket.connect(&book_ticker).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn combined_orderbook(logger_tx: UnboundedSender<WebsocketEvent>) {
    let keep_running = AtomicBool::new(true);
    let streams: Vec<String> = vec!["btcusdt", "ethusdt"]
        .into_iter()
        .map(|symbol| partial_book_depth_stream(symbol, 5, 1000))
        .collect();
    let mut web_socket: WebSockets<'_, CombinedStreamEvent<_>> =
        WebSockets::new(|event: CombinedStreamEvent<WebsocketEventUntag>| {
            if let WebsocketEventUntag::WebsocketEvent(we) = &event.data {
                logger_tx.send(we.clone()).unwrap();
            }
            let data = event.data;
            if let WebsocketEventUntag::Orderbook(orderbook) = data {
                println!("{orderbook:?}")
            }
            Ok(())
        });

    web_socket.connect_multiple(streams).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
    web_socket.disconnect().await.unwrap();
    println!("disconnected");
}

#[allow(dead_code)]
async fn custom_event_loop(logger_tx: UnboundedSender<WebsocketEvent>) {
    let streams: Vec<String> = vec!["btcusdt", "ethusdt"]
        .into_iter()
        .map(|symbol| partial_book_depth_stream(symbol, 5, 1000))
        .collect();
    let mut web_socket: WebSockets<'_, CombinedStreamEvent<_>> =
        WebSockets::new(|event: CombinedStreamEvent<WebsocketEventUntag>| {
            if let WebsocketEventUntag::WebsocketEvent(we) = &event.data {
                logger_tx.send(we.clone()).unwrap();
            }
            let data = event.data;
            if let WebsocketEventUntag::Orderbook(orderbook) = data {
                println!("{orderbook:?}")
            }
            Ok(())
        });
    web_socket.connect_multiple(streams).await.unwrap(); // check error
    loop {
        if let Some((ref mut socket, _)) = web_socket.socket {
            if let Ok(message) = socket.next().await.unwrap() {
                match message {
                    Message::Text(msg) => {
                        if msg.is_empty() {
                            continue;
                        }
                        let event: CombinedStreamEvent<WebsocketEventUntag> = from_str(msg.as_str()).unwrap();
                        eprintln!("event = {event:?}");
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => {}
                    Message::Close(e) => {
                        eprintln!("closed stream = {e:?}");
                        break;
                    }
                }
            }
        }
    }
}
