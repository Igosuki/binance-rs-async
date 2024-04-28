use std::ops::Not;

use chrono::{Duration, Utc};
use serde_json::Value;

use crate::errors::*;

// pub fn build_request(parameters: &BTreeMap<String, String>) -> String {
pub fn build_request(parameters: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>) -> String {
    parameters
        .into_iter()
        .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref()))
        .collect::<Vec<_>>()
        .join("&")
}

pub fn build_request_p<S>(payload: S) -> Result<String>
where
    S: serde::Serialize,
{
    Ok(qs::to_string(&payload)?)
}

pub fn build_signed_request(
    parameters: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    recv_window: u64,
) -> Result<String> {
    let s = IntoIterator::into_iter([
        // Include recvWindow if window > 0
        if recv_window > 0 {
            Some(("recvWindow", recv_window))
        } else {
            None
        },
        // Always include timestamp
        Some(("timestamp", get_timestamp()?)),
    ])
    .flatten()
    .map(|(k, v)| format!("{k}={v}"))
    .chain(
        parameters
            .into_iter()
            .filter(|(k, _)| k.as_ref().is_empty().not())
            .map(|(k, v)| format!("{}={}", k.as_ref(), v.as_ref())),
    )
    .collect::<Vec<String>>()
    .join("&");

    Ok(s)
}

pub fn build_signed_request_p<S>(payload: S, recv_window: u64) -> Result<String>
where
    S: serde::Serialize,
{
    let query_string = qs::to_string(&payload)?;

    let s = IntoIterator::into_iter([
        // Include recvWindow if window > 0
        if recv_window > 0 {
            Some(("recvWindow", recv_window))
        } else {
            None
        },
        // Always include timestamp
        Some(("timestamp", get_timestamp()?)),
    ])
    .flatten()
    .map(|(k, v)| format!("{k}={v}"))
    .collect::<Vec<String>>()
    .join("&");

    let request = if query_string.is_empty() {
        s
    } else {
        format!("{s}&{query_string}")
    };
    Ok(request)
}

pub fn to_i64(v: &Value) -> i64 {
    // TODO: should this return result?
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    // TODO: should this return result?
    v.as_str().unwrap().parse().unwrap()
}

pub fn get_timestamp() -> Result<u64> {
    Ok(Utc::now().timestamp_millis() as u64)
}

/// Returns a duration in milliseconds for the `days`
pub fn days_millis(days: i64) -> i64 {
    Duration::days(days).num_milliseconds()
}

const TRUE: &str = "TRUE";
const FALSE: &str = "FALSE";

pub fn bool_to_string(b: bool) -> String {
    if b {
        TRUE.to_string()
    } else {
        FALSE.to_string()
    }
}

pub fn bool_to_string_some(b: bool) -> Option<String> {
    Some(bool_to_string(b))
}
