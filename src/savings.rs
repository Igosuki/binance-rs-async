use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;
use crate::util::*;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    pub coin: String,
    pub deposit_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub free: f64,
    #[serde(with = "string_or_float")]
    pub freeze: f64,
    #[serde(with = "string_or_float")]
    pub ipoable: f64,
    #[serde(with = "string_or_float")]
    pub ipoing: f64,
    pub is_legal_money: bool,
    #[serde(with = "string_or_float")]
    pub locked: f64,
    pub name: String,
    pub network_list: Vec<Network>,
    #[serde(with = "string_or_float")]
    pub storage: f64,
    pub trading: bool,
    pub withdraw_all_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdrawing: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Network {
    pub address_regex: String,
    pub coin: String,
    /// shown only when "depositEnable" is false.
    pub deposit_desc: Option<String>,
    pub deposit_enable: bool,
    pub is_default: bool,
    pub memo_regex: String,
    /// min number for balance confirmation
    pub min_confirm: u32,
    pub name: String,
    pub network: String,
    pub reset_address_status: bool,
    pub special_tips: Option<String>,
    /// confirmation number for balance unlock
    pub un_lock_confirm: u32,
    /// shown only when "withdrawEnable" is false.
    pub withdraw_desc: Option<String>,
    pub withdraw_enable: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    #[serde(with = "string_or_float")]
    pub withdraw_min: f64,
    // pub insert_time: Option<u64>, //commented out for now, because they are not inside the actual response (only the api doc example)
    // pub update_time: Option<u64>,
    pub withdraw_integer_multiple: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetail {
    #[serde(with = "string_or_float")]
    pub min_withdraw_amount: f64,
    /// false if ALL of networks' are false
    pub deposit_status: bool,
    #[serde(with = "string_or_float")]
    pub withdraw_fee: f64,
    /// false if ALL of networks' are false
    pub withdraw_status: bool,
    /// reason
    pub deposit_tip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddress {
    pub address: String,
    pub coin: String,
    pub tag: String,
    pub url: String,
}

#[derive(Clone)]
pub struct Savings {
    pub client: Client,
    pub recv_window: u64,
}

impl Savings {
    /// Get all coins available for deposit and withdrawal
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, savings::*, config::*};
    /// let savings: Savings = Binance::new_with_env(&Config::testnet());
    /// let coins = tokio_test::block_on(savings.get_all_coins());
    /// assert!(coins.is_ok(), "{:?}", coins)
    /// ```
    pub async fn get_all_coins(&self) -> Result<Vec<CoinInfo>> {
        let request = build_signed_request([("", "")], self.recv_window)?;
        self.client
            .get_signed_d("/sapi/v1/capital/config/getall", request.as_str())
            .await
    }

    /// Fetch details of assets supported on Binance.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, savings::*, config::*};
    /// let savings: Savings = Binance::new_with_env(&Config::testnet());
    /// let coins = tokio_test::block_on(savings.asset_detail(Some("CTR")));
    /// assert!(coins.is_ok(), "{:?}", coins)
    /// ```
    pub async fn asset_detail(&self, asset: Option<&str>) -> Result<BTreeMap<String, AssetDetail>> {
        let parameters = if let Some(asset) = asset {
            [("asset", asset)]
        } else {
            [("", "")]
        };

        let request = build_signed_request(parameters, self.recv_window)?;
        self.client
            .get_signed_d("/sapi/v1/asset/assetDetail", request.as_str())
            .await
    }

    /// Fetch deposit address with network.
    ///
    /// You can get the available networks using `get_all_coins`.
    /// If no network is specified, the address for the default network is returned.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, savings::*, config::*};
    /// let savings: Savings = Binance::new_with_env(&Config::testnet());
    /// let coins = tokio_test::block_on(savings.deposit_address("CTR", None));
    /// assert!(coins.is_ok(), "{:?}", coins)
    /// ```
    pub async fn deposit_address<S>(&self, coin: S, network: Option<&str>) -> Result<DepositAddress>
    where
        S: AsRef<str>,
    {
        let request = if let Some(network) = network {
            let parameters = [("network", network), ("coin", coin.as_ref())];
            build_signed_request(parameters, self.recv_window)?
        } else {
            let parameters = [("coin", coin.as_ref())];
            build_signed_request(parameters, self.recv_window)?
        };

        self.client
            .get_signed_d("/sapi/v1/capital/deposit/address", request.as_str())
            .await
    }
}
