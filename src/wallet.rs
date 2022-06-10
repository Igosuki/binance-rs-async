use std::collections::HashMap;
use crate::client::*;
use crate::errors::*;
use crate::rest_model::*;

static SAPI_V1_SYSTEM_STATUS: &str = "/sapi/v1/system/status";
static SAPI_V1_CAPITAL_CONFIG_GETALL: &str = "/sapi/v1/capital/config/getall";
static SAPI_V1_ACCOUNTSNAPSHOT: &str = "/sapi/v1/accountSnapshot";
static SAPI_V1_ACCOUNT_DISABLEFASTWITHDRAWSWITCH: &str = "/sapi/v1/account/disableFastWithdrawSwitch";
static SAPI_V1_ACCOUNT_ENABLEFASTWITHDRAWSWITCH: &str = "/sapi/v1/account/enableFastWithdrawSwitch";
static SAPI_V1_CAPITAL_WITHDRAW_APPLY: &str = "/sapi/v1/capital/withdraw/apply";
static SAPI_V1_CAPITAL_DEPOSIT_HISREC: &str = "/sapi/v1/capital/deposit/hisrec";
static SAPI_V1_CAPITAL_WITHDRAW_HISTORY: &str = "/sapi/v1/capital/withdraw/history";
static SAPI_V1_CAPITAL_DEPOSIT_ADDRESS: &str = "/sapi/v1/capital/deposit/address";
static SAPI_V1_ACCOUNT_STATUS: &str = "/sapi/v1/account/status";
static SAPI_V1_ACCOUNT_APITRADINGSTATUS: &str = "/sapi/v1/account/apiTradingStatus";
static SAPI_V1_ASSET_DRIBBLET: &str = "/sapi/v1/asset/dribblet";
static SAPI_V1_ASSET_DUSTBTC: &str = "/sapi/v1/asset/dust-btc";
static SAPI_V1_ASSET_DUST: &str = "/sapi/v1/asset/dust";
static SAPI_V1_ASSET_ASSETDIVIDEND: &str = "/sapi/v1/asset/assetDividend";
static SAPI_V1_ASSET_ASSETDETAIL: &str = "/sapi/v1/asset/assetDetail";
static SAPI_V1_ASSET_TRADEFEE: &str = "/sapi/v1/asset/tradeFee";
static SAPI_V1_ASSET_TRANSFER: &str = "/sapi/v1/asset/transfer";
static SAPI_V1_ASSET_GETFUNDINGASSET: &str = "/sapi/v1/asset/get-funding-asset";
static SAPI_V1_ASSET_APIRESTRICTIONS: &str = "/sapi/v1/account/apiRestrictions";

/// This struct acts as a gateway for all wallet endpoints.
/// Preferably use the trait [`Binance`] to get an instance.
#[derive(Clone)]
pub struct Wallet {
    pub client: Client,
    pub recv_window: u64,
}

impl Wallet {
    /// Fetch system status.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let system_status = tokio_test::block_on(wallet.system_status());
    /// assert!(system_status.is_ok(), "{:?}", system_status);
    /// ```
    pub async fn system_status(&self) -> Result<SystemStatus>
    {
        self.client
            .get_p(SAPI_V1_SYSTEM_STATUS, "")
            .await
    }

    /// Get information of coins (available for deposit and withdraw) for user.
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.all_coin_info());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn all_coin_info(&self) -> Result<WalletCoinInfo> {
        self.client
            .get_signed_p(SAPI_V1_CAPITAL_CONFIG_GETALL, Option::<String>::None, self.recv_window)
            .await
    }

    /// Daily account snapshot
    /// The query time period must be less then 30 days
    /// Support query within the last one month only
    /// If startTimeand endTime not sent, return records of the last 7 days by default
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: AccountSnapshotQuery = AccountSnapshotQuery {start_time: None, end_time: None, limit: None, account_type: AccountSnapshotType::Spot};
    /// let records = tokio_test::block_on(wallet.daily_account_snapshot(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn daily_account_snapshot(&self, query: AccountSnapshotQuery) -> Result<SnapshotVosReply> {
        self.client
            .get_signed_p(SAPI_V1_ACCOUNTSNAPSHOT, Some(query), self.recv_window)
            .await
    }

    /// Disable Fast Withdraw Switch
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.disable_fast_withdraw_switch());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn disable_fast_withdraw_switch(&self) -> Result<()> {
        self.client
            .post_signed_p(SAPI_V1_ACCOUNT_DISABLEFASTWITHDRAWSWITCH, Option::<String>::None, self.recv_window)
            .await
    }

    /// Enable Fast Withdraw Switch
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.enable_fast_withdraw_switch());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn enable_fast_withdraw_switch(&self) -> Result<()> {
        self.client
            .post_signed_p(SAPI_V1_ACCOUNT_ENABLEFASTWITHDRAWSWITCH, Option::<String>::None, self.recv_window)
            .await
    }

    /// Apply for Withdrawal
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: CoinWithdrawalQuery = CoinWithdrawalQuery::default();
    /// let records = tokio_test::block_on(wallet.withdraw(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn withdraw(&self, query: CoinWithdrawalQuery) -> Result<()> {
        self.client
            .post_signed_p(SAPI_V1_CAPITAL_WITHDRAW_APPLY, Some(query), self.recv_window)
            .await
    }

    /// Deposit History
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: DepositHistoryQuery = DepositHistoryQuery::default();
    /// let records = tokio_test::block_on(wallet.deposit_history(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn deposit_history(&self, query: DepositHistoryQuery) -> Result<Vec<DepositRecord>> {
        self.client
            .get_signed_p(SAPI_V1_CAPITAL_DEPOSIT_HISREC, Some(query), self.recv_window)
            .await
    }

    /// Withdraw History
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: WithdrawalHistoryQuery = WithdrawalHistoryQuery::default();
    /// let records = tokio_test::block_on(wallet.withdraw_history(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn withdraw_history(&self, query: WithdrawalHistoryQuery) -> Result<Vec<WithdrawalRecord>> {
        self.client
            .get_signed_p(SAPI_V1_CAPITAL_WITHDRAW_HISTORY, Some(query), self.recv_window)
            .await
    }

    /// Deposit address
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: DepositAddressQuery = DepositAddressQuery::default();
    /// let records = tokio_test::block_on(wallet.deposit_address(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn deposit_address(&self, query: DepositAddressQuery) -> Result<DepositAddress> {
        self.client
            .get_signed_p(SAPI_V1_CAPITAL_DEPOSIT_ADDRESS, Some(query), self.recv_window)
            .await
    }

    /// Universal Transfer
    ///
    /// from_symbol must be sent when transfer_type are IsolatedmarginMargin and IsolatedmarginIsolatedmargin
    /// to_symbol must be sent when transfer_type are MarginIsolatedmargin and IsolatedmarginIsolatedmargin
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.universal_transfer("BTC".to_string(), 1.0, None, None, UniversalTransferType::FundingMain));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn universal_transfer(
        &self,
        asset: String,
        amount: f64,
        from_symbol: Option<String>,
        to_symbol: Option<String>,
        transfer_type: UniversalTransferType,
    ) -> Result<TransactionId> {
        let transfer = UniversalTransfer {
            asset: asset.into(),
            amount: amount.into(),
            from_symbol: from_symbol.map(Into::<String>::into),
            to_symbol: to_symbol.map(Into::<String>::into),
            transfer_type,
        };
        self.client
            .post_signed_p(SAPI_V1_ASSET_TRANSFER, transfer, self.recv_window)
            .await
    }

    /// Universal Transfer
    ///
    /// from_symbol must be sent when transfer_type are IsolatedmarginMargin and IsolatedmarginIsolatedmargin
    /// to_symbol must be sent when transfer_type are MarginIsolatedmargin and IsolatedmarginIsolatedmargin
    /// Support query within the last 6 months only
    /// If query.start_time and query.end_time not sent, return records of the last 7 days by default
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let query: UniversalTransferHistoryQuery = UniversalTransferHistoryQuery { start_time: None, end_time: None, transfer_type: UniversalTransferType::FundingMain, current: None, from_symbol: None, to_symbol: None, size: None };
    /// let records = tokio_test::block_on(wallet.universal_transfer_history(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn universal_transfer_history(
        &self,
        query: UniversalTransferHistoryQuery,
    ) -> Result<RecordsQueryResult<UniversalTransferRecord>> {
        self.client
            .get_signed_p(SAPI_V1_ASSET_TRANSFER, Some(query), self.recv_window)
            .await
    }

    /// Current account status
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.account_status());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn account_status(&self) -> Result<AccountStatus> {
        self.client
            .get_signed_p(SAPI_V1_ACCOUNT_STATUS, Option::<String>::None, self.recv_window)
            .await
    }

    /// Current api trading status
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.api_trading_status());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn api_trading_status(&self) -> Result<ApiTradingStatus> {
        self.client
            .get_signed_p(SAPI_V1_ACCOUNT_APITRADINGSTATUS, Option::<String>::None, self.recv_window)
            .await
    }

    /// Dust Log
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.dust_log(None, None));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn dust_log(&self, start_time: Option<u64>, end_time: Option<u64>) -> Result<DustLog> {
        let mut query = HashMap::new();
        query.insert("start_time", start_time);
        query.insert("end_time", end_time);
        self.client
            .get_signed_p(SAPI_V1_ASSET_DRIBBLET, Some(query), self.recv_window)
            .await
    }

    /// Assets convertible to BNB
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.convertible_assets());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn convertible_assets(&self) -> Result<ConvertibleAssets> {
        self.client
            .get_signed_p(SAPI_V1_ASSET_DUSTBTC, Option::<String>::None, self.recv_window)
            .await
    }

    /// Dust Transfer
    ///
    /// Convert dust assets to bnb
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.dust_transfer(vec!["BTC".to_string()]));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn dust_transfer(&self, assets: Vec<String>) -> Result<DustTransfer> {
        let mut params = HashMap::new();
        params.insert("assets", assets);
        self.client
            .post_signed_p(SAPI_V1_ASSET_DUST, Some(params), self.recv_window)
            .await
    }

    /// Asset Dividend Record
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.asset_dividends(AssetDividendQuery::default()));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn asset_dividends(&self, query: AssetDividendQuery) -> Result<RecordsQueryResult<AssetDividend>> {
        self.client
            .get_signed_p(SAPI_V1_ASSET_ASSETDIVIDEND, Some(query), self.recv_window)
            .await
    }

    /// Trade Fees
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.trade_fees(None));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn trade_fees(&self, symbol: Option<String>) -> Result<TradeFees> {
        let mut query = HashMap::new();
        query.insert("symbol", symbol);
        self.client
            .get_signed_p(SAPI_V1_ASSET_TRADEFEE, Some(query), self.recv_window)
            .await
    }

    /// Funding Wallet
    ///
    /// Currently supports querying the following business assets：Binance Pay, Binance Card, Binance Gift Card, Stock Token
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.funding_wallet(None, None));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn funding_wallet(&self, asset: Option<String>, need_btc_valuation: Option<bool>) -> Result<WalletFundings> {
        let mut query = HashMap::new();
        query.insert("asset", asset);
        query.insert("need_btc_valuation", need_btc_valuation.map(|b| format!("{}", b)));
        self.client
            .post_signed_p(SAPI_V1_ASSET_GETFUNDINGASSET, Some(query), self.recv_window)
            .await
    }

    /// Api Key Permissions
    ///
    /// # Examples
    /// ```rust,no_run
    /// use binance::{api::*, wallet::*, config::*, rest_model::*};
    /// let wallet: Wallet = Binance::new_with_env(&Config::testnet());
    /// let records = tokio_test::block_on(wallet.api_key_permissions());
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn api_key_permissions(&self) -> Result<ApiKeyPermissions> {
        self.client
            .post_signed_p(SAPI_V1_ASSET_APIRESTRICTIONS, Option::<String>::None, self.recv_window)
            .await
    }
}

