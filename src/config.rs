#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub delivery_rest_api_endpoint: String,
    pub delivery_ws_endpoint: String,

    pub recv_window: u64,
}

impl Config {
    /// Configure binance with default production endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// ```
    pub fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com:9443".into(),

            delivery_rest_api_endpoint: "https://dapi.binance.com".into(),
            delivery_ws_endpoint: "wss://dstream.binance.com:9443".into(),

            recv_window: 5000,
        }
    }

    /// Configure binance with all testnet endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::testnet();
    /// ```
    pub fn testnet() -> Config {
        Config::default()
            .set_rest_api_endpoint("https://testnet.binance.vision")
            .set_ws_endpoint("wss://testnet.binance.vision")
            .set_futures_rest_api_endpoint("https://testnet.binancefuture.com")
            .set_futures_ws_endpoint("wss://testnet.binancefuture.com")
            .set_delivery_rest_api_endpoint("https://testnet.binancefuture.com")
            .set_delivery_ws_endpoint("wss://testnet.binancefuture.com")
    }

    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }
    pub fn set_futures_rest_api_endpoint<T: Into<String>>(mut self, futures_rest_api_endpoint: T) -> Self {
        self.futures_rest_api_endpoint = futures_rest_api_endpoint.into();
        self
    }

    pub fn set_futures_ws_endpoint<T: Into<String>>(mut self, futures_ws_endpoint: T) -> Self {
        self.futures_ws_endpoint = futures_ws_endpoint.into();
        self
    }

    pub fn set_delivery_rest_api_endpoint<T: Into<String>>(mut self, delivery_rest_api_endpoint: T) -> Self {
        self.delivery_rest_api_endpoint = delivery_rest_api_endpoint.into();
        self
    }

    pub fn set_delivery_ws_endpoint<T: Into<String>>(mut self, delivery_ws_endpoint: T) -> Self {
        self.delivery_ws_endpoint = delivery_ws_endpoint.into();
        self
    }

    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }
}
