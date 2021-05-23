use crate::errors::*;
use serde_json::from_str;
use url::Url;

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{connect, Message};

pub static WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";
pub static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
pub static EXECUTION_REPORT: &str = "executionReport";
pub static KLINE: &str = "kline";
pub static AGGREGATED_TRADE: &str = "aggTrade";
pub static DEPTH_ORDERBOOK: &str = "depthUpdate";
pub static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
pub static DAYTICKER: &str = "24hrTicker";

pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a>,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    /// Connect to a websocket endpoint
    pub fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                Err(Error::Msg(format!("Error during handshake {}", e)))
            }
        }
    }

    /// Disconnect from the endpoint
    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                match message {
                    Message::Text(msg) => {
                        if msg.is_empty() {
                            return Ok(());
                        }
                        let event: WE = from_str(msg.as_str())?;
                        (self.handler)(event)?;
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => {}
                    Message::Close(e) => {
                        return Err(Error::Msg(format!("Disconnected {:?}", e)));
                    }
                }
            }
        }
        Ok(())
    }
}
