pub static DATA_REST_ENDPOINT: &str = "https://api.binance.com";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub rest_api_endpoint: String,
    pub ws_endpoint: String,

    pub futures_rest_api_endpoint: String,
    pub futures_ws_endpoint: String,

    pub recv_window: u64,

    pub binance_us_api: bool,

    pub timeout: Option<u64>,
}

impl Config {
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
    }

    /// Sets the rest api endpoint. Defaults to <https://api.binance.com>.
    ///
    /// # Arguments
    ///
    /// * `rest_api_endpoint`:
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_rest_api_endpoint("http://myendpoint:8080");
    /// ```
    pub fn set_rest_api_endpoint<T: Into<String>>(mut self, rest_api_endpoint: T) -> Self {
        self.rest_api_endpoint = rest_api_endpoint.into();
        self
    }

    /// Sets the websocket endpoint. Defaults to "wss://stream.binance.com:9443".
    ///
    /// # Arguments
    ///
    /// * `ws_endpoint`:
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_ws_endpoint("ws://myendpoint:8080");
    /// ```
    pub fn set_ws_endpoint<T: Into<String>>(mut self, ws_endpoint: T) -> Self {
        self.ws_endpoint = ws_endpoint.into();
        self
    }

    /// Sets the futures rest api endpoint. Defaults to <https://fapi.binance.com>.
    ///
    /// # Arguments
    ///
    /// * `futures_rest_api_endpoint`:
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_futures_rest_api_endpoint("http://myendpoint:8080");
    /// ```
    pub fn set_futures_rest_api_endpoint<T: Into<String>>(mut self, futures_rest_api_endpoint: T) -> Self {
        self.futures_rest_api_endpoint = futures_rest_api_endpoint.into();
        self
    }

    /// Sets the futures websocket endpoint. Defaults to "wss://fstream.binance.com".
    ///
    /// # Arguments
    ///
    /// * `futures_ws_endpoint`:
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_futures_ws_endpoint("ws://myendpoint:8080");
    /// ```
    pub fn set_futures_ws_endpoint<T: Into<String>>(mut self, futures_ws_endpoint: T) -> Self {
        self.futures_ws_endpoint = futures_ws_endpoint.into();
        self
    }

    /// Sets the 'receive window'. The receive window is the number of milliseconds after timestamp
    /// the request is valid for.
    ///
    /// # Arguments
    ///
    /// * `recv_window`: The receive window, in milliseconds. Defaults to 5000.
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_recv_window(300);
    /// ```
    pub fn set_recv_window(mut self, recv_window: u64) -> Self {
        self.recv_window = recv_window;
        self
    }

    /// Sets the client timeout
    ///
    /// # Arguments
    ///
    /// * `timeout`: The timeout, in seconds
    ///
    /// returns: Config
    ///
    /// # Examples
    ///
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// config.set_timeout(3);
    /// ```
    pub fn set_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

impl Default for Config {
    /// Configure binance with default production endpoints
    /// # Examples
    /// ```
    /// use binance::config::Config;
    /// let config = Config::default();
    /// ```
    fn default() -> Config {
        Config {
            rest_api_endpoint: "https://api.binance.com".into(),
            ws_endpoint: "wss://stream.binance.com:9443".into(),

            futures_rest_api_endpoint: "https://fapi.binance.com".into(),
            futures_ws_endpoint: "wss://fstream.binance.com".into(),

            recv_window: 5000,
            binance_us_api: false,

            timeout: None,
        }
    }
}
