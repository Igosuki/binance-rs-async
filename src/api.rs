use crate::account::*;
use crate::client::*;
use crate::config::Config;
use crate::general::*;
use crate::market::*;
use crate::userstream::*;

pub trait Binance: Sized {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Self::new_with_config(api_key, secret_key, &Config::default())
    }

    /// Create a binance API using environment variables for credentials
    /// BINANCE_API_KEY=<your api key>
    /// BINANCE_API_SECRET_KEY=<your secret key>
    fn new_with_env(config: &Config) -> Self {
        let api_key = std::env::var("BINANCE_API_KEY").ok();
        let secret = std::env::var("BINANCE_API_SECRET_KEY").ok();
        Self::new_with_config(api_key, secret, config)
    }

    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self;
}

impl Binance for General {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> General {
        General {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
        }
    }
}

impl Binance for Account {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Account {
        Account {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "savings_api")]
impl Binance for crate::savings::Savings {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for Market {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Market {
        Market {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

impl Binance for UserStream {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::general::FuturesGeneral {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::market::FuturesMarket {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "futures_api")]
impl Binance for crate::futures::account::FuturesAccount {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.futures_rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "delivery_api")]
impl Binance for crate::delivery::account::DeliveryAccount {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.delivery_rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}

#[cfg(feature = "margin_api")]
impl Binance for crate::margin::Margin {
    fn new_with_config(api_key: Option<String>, secret_key: Option<String>, config: &Config) -> Self {
        Self {
            client: Client::new(api_key, secret_key, config.rest_api_endpoint.clone()),
            recv_window: config.recv_window,
        }
    }
}
