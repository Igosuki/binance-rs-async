use crate::errors::*;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn build_signed_request(
    mut parameters: BTreeMap<String, String>,
    recv_window: u64,
) -> Result<String> {
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
        bail!("Failed to get timestamp")
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
        request.pop(); // remove last &

        Ok(request)
    } else {
        bail!("Failed to get timestamp")
    }
}

pub fn to_i64(v: &Value) -> i64 {
    v.as_i64().unwrap()
}

pub fn to_f64(v: &Value) -> f64 {
    v.as_str().unwrap().parse().unwrap()
}

fn get_timestamp() -> Result<u64> {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH)?;

    Ok(since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000)
}
