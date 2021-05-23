use hex::encode as hex_encode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Response;
use reqwest::StatusCode;
use ring::hmac;
use serde_json::from_str;
use serde::de;
use std::time::Duration;

use crate::errors::*;
use crate::util::{build_request_p, build_signed_request_p};
use crate::errors::error_messages;

static API1_HOST: &str = "https://api.binance.com";

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_host(api_key, secret_key, None)
    }

    /// Returns a client based on the specified host and credentials
    /// Credentials do not need to be specified when using public endpoints
    /// If host is unspecified, defaults to Binance's public api host
    pub fn new_with_host(
        api_key: Option<String>,
        secret_key: Option<String>,
        host: Option<String>,
    ) -> Self {
        let builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        let builder = builder.timeout(Duration::from_secs(2));
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            inner: builder.build().unwrap(),
            host: host.unwrap_or(API1_HOST.to_string()),
        }
    }

    pub async fn get_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let response = self
            .inner
            .clone()
            .get(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn get_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: Option<P>,
    ) -> Result<T> {
        let req = if let Some(p) = payload {
            build_request_p(p)?
        } else {
            String::new()
        };
        let string = self.get_signed(endpoint, &req).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    pub async fn post_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let response = self
            .inner
            .clone()
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn post_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: P,
        recv_window: u64,
    ) -> Result<T> {
        let request = build_signed_request_p(payload, recv_window)?;
        let string = self.post_signed(endpoint, &request).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    pub async fn delete_signed_p<T: de::DeserializeOwned, P: serde::Serialize>(
        &self,
        endpoint: &str,
        payload: P,
        recv_window: u64,
    ) -> Result<T> {
        let request = build_signed_request_p(payload, recv_window)?;
        let string = self.delete_signed(endpoint, &request).await?;
        let data: &str = string.as_str();
        let t = from_str(data)?;
        Ok(t)
    }

    pub async fn delete_signed(&self, endpoint: &str, request: &str) -> Result<String> {
        let url = self.sign_request(endpoint, request);
        let response = self
            .inner
            .clone()
            .delete(url.as_str())
            .headers(self.build_headers(true)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn get(&self, endpoint: &str, request: &str) -> Result<String> {
        let mut url: String = format!("{}{}", self.host, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let response = reqwest::get(url.as_str()).await?;

        self.handler(response).await
    }

    pub async fn post(&self, endpoint: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, endpoint);

        let response = self
            .inner
            .clone()
            .post(url.as_str())
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn put(&self, endpoint: &str, listen_key: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, endpoint);
        let data: String = format!("listenKey={}", listen_key);

        let response = self
            .inner
            .clone()
            .put(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn delete(&self, endpoint: &str, listen_key: &str) -> Result<String> {
        let url: String = format!("{}{}", self.host, endpoint);
        let data: String = format!("listenKey={}", listen_key);

        let response = self
            .inner
            .clone()
            .delete(url.as_str())
            .headers(self.build_headers(false)?)
            .body(data)
            .send()
            .await?;

        self.handler(response).await
    }

    // Request must be signed
    fn sign_request(&self, endpoint: &str, request: &str) -> String {
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hex_encode(hmac::sign(&signed_key, request.as_bytes()).as_ref());

        let request_body: String = format!("{}&signature={}", request, signature);
        let url: String = format!("{}{}?{}", self.host, endpoint, request_body);

        url
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custon_headers = HeaderMap::new();

        custon_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custon_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custon_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custon_headers)
    }

    async fn handler(&self, response: Response) -> Result<String> {
        match response.status() {
            StatusCode::OK => {
                let body = response.bytes().await?;
                let result = std::str::from_utf8(&body);
                Ok(result?.to_string())
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                Err(Error::Msg("Internal Server Error".to_string()))
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                Err(Error::Msg("Service Unavailable".to_string()))
            }
            StatusCode::UNAUTHORIZED => {
                Err(Error::Msg("Unauthorized".to_string()))
            }
            StatusCode::BAD_REQUEST => {
                let error: BinanceContentError = response.json().await?;
                Err(handle_content_error(error).into())
            }
            s => {
                Err(Error::Msg(format!("Received response: {:?}", s)))
            }
        }
    }
}

fn handle_content_error(error: BinanceContentError) -> crate::errors::Error {
    match (error.code, error.msg.as_ref()) {
        (-1013, error_messages::INVALID_PRICE) => Error::InvalidPrice,
        _ => Error::BinanceError { response: error }
    }
}
