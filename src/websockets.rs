use crate::errors::*;
use futures::StreamExt;
use serde_json::from_str;
use url::Url;

use crate::config::Config;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream};
use tungstenite::handshake::client::Response;

pub static STREAM_ENDPOINT: &str = "stream";
pub static WS_ENDPOINT: &str = "ws";
pub static OUTBOUND_ACCOUNT_INFO: &str = "outboundAccountInfo";
pub static OUTBOUND_ACCOUNT_POSITION: &str = "outboundAccountPosition";
pub static EXECUTION_REPORT: &str = "executionReport";
pub static KLINE: &str = "kline";
pub static AGGREGATED_TRADE: &str = "aggTrade";
pub static DEPTH_ORDERBOOK: &str = "depthUpdate";
pub static PARTIAL_ORDERBOOK: &str = "lastUpdateId";
pub static DAYTICKER: &str = "24hrTicker";

pub struct WebSockets<'a, WE> {
    pub socket: Option<(WebSocketStream<MaybeTlsStream<TcpStream>>, Response)>,
    handler: Box<dyn FnMut(WE) -> Result<()> + 'a>,
    conf: Config,
}

impl<'a, WE: serde::de::DeserializeOwned> WebSockets<'a, WE> {
    /// New websocket holder with default configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a,
    {
        Self::new_with_options(handler, Config::default())
    }

    /// New websocket holder with provided configuration
    /// # Examples
    /// see examples/binance_websockets.rs
    pub fn new_with_options<Callback>(handler: Callback, conf: Config) -> WebSockets<'a, WE>
    where
        Callback: FnMut(WE) -> Result<()> + 'a,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
            conf,
        }
    }
    /// Connect to a websocket endpoint
    pub async fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}/{}/{}", self.conf.ws_endpoint, WS_ENDPOINT, endpoint);
        let url = Url::parse(&wss)?;

        match connect_async(url).await {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => Err(Error::Msg(format!("Error during handshake {}", e))),
        }
    }

    /// Disconnect from the endpoint
    pub async fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None).await?;
            Ok(())
        } else {
            Err(Error::Msg("Not able to close the connection".to_string()))
        }
    }

    pub async fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.next().await.unwrap()?;

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
