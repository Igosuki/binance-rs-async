use crate::account::*;
use crate::client::*;
use crate::general::*;
use crate::margin::Margin;
use crate::market::*;
use crate::userstream::*;

//#[derive(Clone)]
pub trait Binance {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Self;

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> Self;
}

// TODO: builder pattern
impl Binance for General {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> General {
        General {
            client: Client::new(api_key, secret_key),
        }
    }

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> General {
        General {
            client: Client::new_with_host(api_key, secret_key, host),
        }
    }
}

impl Binance for Account {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Account {
        Account {
            client: Client::new(api_key, secret_key),
            recv_window: 5000,
        }
    }

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> Account {
        Account {
            client: Client::new_with_host(api_key, secret_key, host),
            recv_window: 5000,
        }
    }
}

impl Binance for Market {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Market {
        Market {
            client: Client::new(api_key, secret_key),
            recv_window: 5000,
        }
    }

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> Market {
        Market {
            client: Client::new_with_host(api_key, secret_key, host),
            recv_window: 5000,
        }
    }
}

impl Binance for UserStream {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> UserStream {
        UserStream {
            client: Client::new(api_key, secret_key),
            recv_window: 5000,
        }
    }

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> UserStream {
        UserStream {
            client: Client::new_with_host(api_key, secret_key, host),
            recv_window: 5000,
        }
    }
}

impl Binance for Margin {
    fn new(api_key: Option<String>, secret_key: Option<String>) -> Margin {
        Margin {
            client: Client::new(api_key, secret_key),
            recv_window: 5000,
        }
    }

    fn new_with_host(api_key: Option<String>, secret_key: Option<String>, host: Option<String>) -> Margin {
        Margin {
            client: Client::new_with_host(api_key, secret_key, host),
            recv_window: 5000,
        }
    }
}
