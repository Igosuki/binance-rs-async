use std::collections::BTreeMap;

#[allow(unused_imports)]
use chrono::{Duration, TimeZone, Utc};
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

pub fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

pub fn get_timestamp() -> Result<u64> {
    Ok(Utc::now().timestamp_millis() as u64)
}

/// a duration of some days:
///     defaut Sum(days) = 90 days = 7776000000
///
/// # Examples
/// ```
/// let sum = duration_by(Some(90));
/// assert!(true, "= 90days duration: {:?}", sum);
/// ```
pub fn duration_of(days: Option<i64>) -> i64 {
    // default = 90 days
    Duration::days(days.unwrap_or(90)).num_milliseconds()
}

/// a timestamp: before some days.
///     like: a timestamp 5 years ago.
/// # Examples
/// ```
/// use binance::util::ago_from;
/// let ago_at = ago_from(None, Some(5*360));
/// assert!(true, "≈ 5 year ago, timestamp: {:?}", ago_at);
/// ```
pub fn ago_from(start_at: Option<i64>, days: Option<i64>) -> i64 {
    // default = from now
    let start = start_at.unwrap_or(Utc::now().timestamp_millis());

    // default ≈ 2years
    let duration = Duration::days(days.unwrap_or(360 * 2));

    // default ≈ 2years ago
    let ago_at = start - duration.num_milliseconds();
    ago_at
}

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

pub fn bool_to_string_some(b: bool) -> Option<String> {
    Some(bool_to_string(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_by() {
        let ts_90_days = duration_of(Some(90 as i64));
        let ts_365days = duration_of(Some(365 as i64));

        // 90 days duration timestamp: 7776000000
        assert_eq!(duration_of(None), 7776000000);
        assert_eq!(ts_90_days, 7776000000);

        // 365 days duration timestamp: 31536000000
        assert_eq!(ts_365days, 31536000000);
    }

    #[test]
    fn test_ago_by() {
        // default = from now, ≈ 2 years ago
        let ago_at = ago_from(None, None);
        let ago_2years = Utc.timestamp_millis(ago_at).to_rfc3339();

        // from now, ≈ 5 years ago
        let ago_5years_at = ago_from(None, Some(5 * 360));
        let ago_5years = Utc.timestamp_millis(ago_5years_at).to_rfc3339();

        println!("≈ 2 years ago: {}, ts={}", ago_2years, ago_at);
        println!("≈ 5 years ago: {}, ts={}", ago_5years, ago_5years_at);
    }
}
