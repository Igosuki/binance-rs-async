use std::collections::BTreeMap;

use crate::client::Client;
use crate::errors::*;
use crate::options::rest_model::*;
use crate::rest_model::PairQuery;
use crate::util::{build_signed_request, build_signed_request_p};

#[derive(Clone)]
pub struct OptionsAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderIdQuery {
    pub symbol: String,
    pub order_id: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExerciseRecordQuery {
    pub symbol: String,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u16>,
}

impl OptionsAccount {
    /// Get account asset information
    pub async fn account_information(&self) -> Result<Vec<AccountInformation>> {
        let request = build_signed_request(BTreeMap::<String, String>::new(), self.recv_window)?;
        self.client.get_signed_d("/eapi/v1/account", &request).await
    }

    /// Place an options order
    pub async fn place_order(&self, order: OptionOrderRequest) -> Result<OptionOrder> {
        self.client.post_signed_p("/eapi/v1/order", order, self.recv_window).await
    }

    /// Get an options order by symbol and order id
    pub async fn get_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<OptionOrder> {
        self.client
            .get_signed_p(
                "/eapi/v1/order",
                Some(OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                }),
                self.recv_window,
            )
            .await
    }

    /// Cancel an options order
    pub async fn cancel_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<OptionOrder> {
        self.client
            .delete_signed_p(
                "/eapi/v1/order",
                OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                },
                self.recv_window,
            )
            .await
    }

    /// Get all open options orders for a symbol
    pub async fn get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<OptionOrder>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/eapi/v1/openOrders", &payload).await
    }

    /// Cancel all open options orders for a symbol
    pub async fn cancel_all_open_orders(&self, symbol: impl Into<String>) -> Result<()> {
        self.client
            .delete_signed_p::<(), _>(
                "/eapi/v1/allOpenOrders",
                PairQuery { symbol: symbol.into() },
                self.recv_window,
            )
            .await?;
        Ok(())
    }

    /// Get options position information for a symbol
    pub async fn get_position(&self, symbol: impl Into<String>) -> Result<Vec<OptionPosition>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/eapi/v1/position", &payload).await
    }

    /// Get exercise records for a symbol
    pub async fn get_exercise_record(
        &self,
        symbol: impl Into<String>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Vec<ExerciseRecord>> {
        self.client
            .get_signed_p(
                "/eapi/v1/exerciseRecord",
                Some(ExerciseRecordQuery {
                    symbol: symbol.into(),
                    start_time,
                    end_time,
                    limit,
                }),
                self.recv_window,
            )
            .await
    }
}
