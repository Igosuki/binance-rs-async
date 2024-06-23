use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::sync::atomic::AtomicBool;

use binance::websockets::*;
use binance::ws_model::WebsocketEvent;

#[tokio::main]
async fn main() {
    save_all_trades_websocket().await;
}

async fn save_all_trades_websocket() {
    struct WebSocketHandler {
        wrt: Writer<File>,
    }

    impl WebSocketHandler {
        pub fn new(local_wrt: Writer<File>) -> Self {
            WebSocketHandler { wrt: local_wrt }
        }

        // serialize DayTickerEvent as CSV records
        pub fn write_to_file(&mut self, events: Vec<WebsocketEvent>) -> Result<(), Box<dyn Error>> {
            for event in events {
                self.wrt.serialize(event)?;
            }
            Ok(())
        }
    }

    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let file_path = std::path::Path::new("test.csv");
    let local_wrt = csv::Writer::from_path(file_path).unwrap();

    let mut web_socket_handler = WebSocketHandler::new(local_wrt);
    let agg_trade: String = "!ticker@arr".to_string();
    let mut web_socket: WebSockets<'_, Vec<WebsocketEvent>> = WebSockets::new(|events: Vec<WebsocketEvent>| {
        // You can break the event_loop if some condition is met be setting keep_running to false
        // keep_running.store(false, Ordering::Relaxed);
        if let Err(error) = web_socket_handler.write_to_file(events) {
            println!("{error}");
        }
        Ok(())
    });

    web_socket.connect(&agg_trade).await.unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running).await {
        println!("Error: {e}");
    }
}
