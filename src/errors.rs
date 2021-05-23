use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

error_chain! {
    errors {
        BinanceError(response: BinanceContentError)
        InvalidOrderError(msg: String)
        InvalidPrice
     }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        Qs(serde_qs::Error);
        Tungstenite(tungstenite::Error);
        TimestampError(std::time::SystemTimeError);
        UTF8Err(std::str::Utf8Error);
    }
}

pub mod error_messages {
    pub const INVALID_PRICE: &str = "Invalid price.";
}
