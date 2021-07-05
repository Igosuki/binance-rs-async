use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
#[error("code: {code}, msg: {msg}")]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// First errors are technical errors
/// All unhandled binance content errors are BinanceError
/// The rest are binance content errors that are properly handled
/// Unhandled binance errors are Msg
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Qs(#[from] serde_qs::Error),
    #[error(transparent)]
    Tungstenite(#[from] tungstenite::Error),
    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    UTF8Err(#[from] std::str::Utf8Error),
    #[error("{response}")]
    BinanceError {
        #[from]
        response: BinanceContentError,
    },
    #[error("unknown symbol {0}")]
    UnknownSymbol(String),
    #[error("{msg}")]
    InvalidOrderError { msg: String },
    #[error("invalid price")]
    InvalidPrice,
    #[error("invalid period {0}")]
    InvalidPeriod(String),
    #[error("{0}")]
    Msg(String),
}

/// Custom error messages
pub mod error_messages {
    pub const INVALID_PRICE: &str = "Invalid price.";
}

pub type Result<T> = core::result::Result<T, Error>;
