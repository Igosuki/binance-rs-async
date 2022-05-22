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
    pub async fn system_status<S, F>(&self) -> Result<SystemStatus>
        where
            S: Into<String>,
            F: Into<f64>,
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
            .get_signed_p(SAPI_V1_CAPITAL_CONFIG_GETALL, None, self.recv_window)
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
    pub async fn daily_account_snapshot(&self, query: AccountSnapshotQuery) -> Result<SnapshotVos> {
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
            .post_signed_p(SAPI_V1_ACCOUNT_DISABLEFASTWITHDRAWSWITCH, None, self.recv_window)
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
            .post_signed_p(SAPI_V1_ACCOUNT_ENABLEFASTWITHDRAWSWITCH, None, self.recv_window)
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
    /// let query: WithdrawHistoryQuery = WithdrawHistoryQuery::default();
    /// let records = tokio_test::block_on(wallet.withdraw_history(query));
    /// assert!(records.is_ok(), "{:?}", records);
    /// ```
    pub async fn withdraw_history(&self, query: WithdrawHistoryQuery) -> Result<Vec<WithdrawRecord>> {
        self.client
            .get_signed_p(SAPI_V1_CAPITAL_WITHDRAW_HISTORY, Some(query), self.recv_window)
            .await
    }

    /// Withdraw History
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
}

