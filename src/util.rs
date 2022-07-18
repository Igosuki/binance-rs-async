use std::collections::BTreeMap;

use chrono::{Duration, Utc};
use serde_json::Value;

use crate::errors::*;

pub fn build_request(parameters: &BTreeMap<String, String>) -> String {
    let mut request = String::new();
    for (key, value) in parameters {
        let param = format!("{}={}&", key, value);
        request.push_str(param.as_ref());
    }
    request.pop(); // remove last &

    request
}

pub fn build_request_p<S>(payload: S) -> Result<String>
where
    S: serde::Serialize,
{
    Ok(qs::to_string(&payload)?)
}

pub fn build_signed_request(mut parameters: BTreeMap<String, String>, recv_window: u64) -> Result<String> {
    if recv_window > 0 {
        parameters.insert("recvWindow".into(), recv_window.to_string());
    }

    if let Ok(timestamp) = get_timestamp() {
        parameters.insert("timestamp".into(), timestamp.to_string());

        let mut request = String::new();
        for (key, value) in &parameters {
            let param = format!("{}={}&", key, value);
            request.push_str(param.as_ref());
        }
        request.pop(); // remove last &

        Ok(request)
    } else {
        Err(Error::Msg("Failed to get timestamp".to_string()))
    }
}

pub fn build_signed_request_p<S>(payload: S, recv_window: u64) -> Result<String>
where
    S: serde::Serialize,
{
    let query_string = qs::to_string(&payload)?;
    let mut parameters: BTreeMap<String, String> = BTreeMap::new();

    if recv_window > 0 {
        parameters.insert("recvWindow".into(), recv_window.to_string());
    }

    if let Ok(timestamp) = get_timestamp() {
        parameters.insert("timestamp".into(), timestamp.to_string());

        let mut request = query_string;
        for (key, value) in &parameters {
            let param = format!("&{}={}", key, value);
            request.push_str(param.as_ref());
        }
        if let Some('&') = request.chars().last() {
            request.pop(); // remove last &
        }

        Ok(request)
    } else {
        Err(Error::Msg("Failed to get timestamp".to_string()))
    }
}

pub fn to_i64(v: &Value) -> i64 { v.as_i64().unwrap() }

pub fn to_f64(v: &Value) -> f64 { v.as_str().unwrap().parse().unwrap() }

pub fn get_timestamp() -> Result<u64> { Ok(Utc::now().timestamp_millis() as u64) }

/// Returns a duration in milliseconds for the `days`
pub fn days_millis(days: i64) -> i64 { Duration::days(days).num_milliseconds() }

lazy_static! {
    static ref TRUE: String = "TRUE".to_string();
}
lazy_static! {
    static ref FALSE: String = "FALSE".to_string();
}

pub fn bool_to_string(b: bool) -> String {
    if b {
        TRUE.to_string()
    } else {
        FALSE.to_string()
    }
}

pub fn bool_to_string_some(b: bool) -> Option<String> { Some(bool_to_string(b)) }
