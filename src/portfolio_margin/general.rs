use crate::client::*;
use crate::errors::*;
use crate::portfolio_margin::rest_model::*;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct PortfolioMarginGeneral {
    pub client: Client,
    pub recv_window: u64,
}

impl PortfolioMarginGeneral {
    /// Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.client.get::<()>("/papi/v1/ping", None).await?;
        Ok("pong".into())
    }

    /// Get unified account information
    pub async fn account_information(&self) -> Result<AccountInformation> {
        let request = crate::util::build_signed_request(BTreeMap::<String, String>::new(), self.recv_window)?;
        self.client.get_signed_d("/papi/v1/account", &request).await
    }

    /// Get account balance per asset
    pub async fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        let request = crate::util::build_signed_request(BTreeMap::<String, String>::new(), self.recv_window)?;
        self.client.get_signed_d("/papi/v1/balance", &request).await
    }

    /// Get the maximum borrowable amount for an asset
    pub async fn max_borrowable(&self, asset: impl Into<String>) -> Result<MaxBorrowable> {
        let mut params = BTreeMap::<String, String>::new();
        params.insert("asset".into(), asset.into());
        let request = crate::util::build_signed_request(params, self.recv_window)?;
        self.client.get_signed_d("/papi/v1/margin/maxBorrowable", &request).await
    }
}
