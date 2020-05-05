use crate::errors::*;
use crate::model::*;
use serde_json::from_str;
use url::Url;

use std::sync::atomic::{AtomicBool, Ordering};
use tungstenite::client::AutoStream;
use tungstenite::handshake::client::Response;
use tungstenite::protocol::WebSocket;
use tungstenite::{connect, Message};

static WEBSOCKET_URL: &str = "wss://stream.binance.com:9443/ws/";

static KLINE: &str = "kline";

pub enum WebsocketEvent {
    SomeEvent(String),
}

pub struct WebSockets<'a> {
    pub socket: Option<(WebSocket<AutoStream>, Response)>,
    handler: Box<dyn FnMut(WebsocketEvent) -> Result<()> + 'a>,
}

impl<'a> WebSockets<'a> {
    pub fn new<Callback>(handler: Callback) -> WebSockets<'a>
    where
        Callback: FnMut(WebsocketEvent) -> Result<()> + 'a,
    {
        WebSockets {
            socket: None,
            handler: Box::new(handler),
        }
    }

    pub fn connect(&mut self, endpoint: &str) -> Result<()> {
        let wss: String = format!("{}{}", WEBSOCKET_URL, endpoint);
        let url = Url::parse(&wss)?;

        match connect(url) {
            Ok(answer) => {
                self.socket = Some(answer);
                Ok(())
            }
            Err(e) => {
                bail!(format!("Error during handshake {}", e));
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(ref mut socket) = self.socket {
            socket.0.close(None)?;
            Ok(())
        } else {
            bail!("Not able to close the connection");
        }
    }

    pub fn event_loop(&mut self, running: &AtomicBool) -> Result<()> {
        while running.load(Ordering::Relaxed) {
            if let Some(ref mut socket) = self.socket {
                let message = socket.0.read_message()?;
                let value: serde_json::Value = serde_json::from_str(message.to_text()?)?;

                match message {
                    Message::Text(msg) => {
                        self.handler(WebsocketEvent::SomeEvent(msg.clone()));
                    }
                    Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => {}
                    Message::Close(e) => {
                        bail!(format!("Disconnected {:?}", e));
                    }
                }
            }
        }
        Ok(())
    }
}
