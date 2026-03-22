use std::collections::BTreeMap;

use crate::client::Client;
use crate::errors::*;
use crate::portfolio_margin::rest_model::{CmOrder, CmOrderRequest, CmPosition, MarginOrder, MarginOrderRequest,
                                          RepayResult, UmOrder, UmOrderRequest, UmPosition};
use crate::rest_model::PairQuery;
use crate::util::{build_signed_request, build_signed_request_p};

#[derive(Clone)]
pub struct PortfolioMarginAccount {
    pub client: Client,
    pub recv_window: u64,
}

// Helper struct for cancellation by order id
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OrderIdQuery {
    pub symbol: String,
    pub order_id: u64,
}

impl PortfolioMarginAccount {
    /// Place a UM futures order
    pub async fn um_place_order(&self, order: UmOrderRequest) -> Result<UmOrder> {
        self.client.post_signed_p("/papi/v1/um/order", order, self.recv_window).await
    }

    /// Get a UM futures order by symbol and order id
    pub async fn um_get_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<UmOrder> {
        self.client
            .get_signed_p(
                "/papi/v1/um/order",
                Some(OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get all open UM futures orders for a symbol
    pub async fn um_get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<UmOrder>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/papi/v1/um/openOrders", &payload).await
    }

    /// Cancel a UM futures order
    pub async fn um_cancel_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<UmOrder> {
        self.client
            .delete_signed_p(
                "/papi/v1/um/order",
                OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                },
                self.recv_window,
            )
            .await
    }

    /// Cancel all open UM futures orders for a symbol
    pub async fn um_cancel_all_open_orders(&self, symbol: impl Into<String>) -> Result<()> {
        self.client
            .delete_signed_p::<(), _>(
                "/papi/v1/um/allOpenOrders",
                PairQuery { symbol: symbol.into() },
                self.recv_window,
            )
            .await?;
        Ok(())
    }

    /// Get UM futures position information for a symbol
    pub async fn um_position_information(&self, symbol: impl Into<String>) -> Result<Vec<UmPosition>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/papi/v1/um/positionRisk", &payload).await
    }

    /// Place a CM futures order
    pub async fn cm_place_order(&self, order: CmOrderRequest) -> Result<CmOrder> {
        self.client.post_signed_p("/papi/v1/cm/order", order, self.recv_window).await
    }

    /// Get a CM futures order by symbol and order id
    pub async fn cm_get_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<CmOrder> {
        self.client
            .get_signed_p(
                "/papi/v1/cm/order",
                Some(OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                }),
                self.recv_window,
            )
            .await
    }

    /// Get all open CM futures orders for a symbol
    pub async fn cm_get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<CmOrder>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/papi/v1/cm/openOrders", &payload).await
    }

    /// Cancel a CM futures order
    pub async fn cm_cancel_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<CmOrder> {
        self.client
            .delete_signed_p(
                "/papi/v1/cm/order",
                OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                },
                self.recv_window,
            )
            .await
    }

    /// Cancel all open CM futures orders for a symbol
    pub async fn cm_cancel_all_open_orders(&self, symbol: impl Into<String>) -> Result<()> {
        self.client
            .delete_signed_p::<(), _>(
                "/papi/v1/cm/allOpenOrders",
                PairQuery { symbol: symbol.into() },
                self.recv_window,
            )
            .await?;
        Ok(())
    }

    /// Get CM futures position information for a symbol
    pub async fn cm_position_information(&self, symbol: impl Into<String>) -> Result<Vec<CmPosition>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/papi/v1/cm/positionRisk", &payload).await
    }

    /// Place a margin order
    pub async fn margin_place_order(&self, order: MarginOrderRequest) -> Result<MarginOrder> {
        self.client.post_signed_p("/papi/v1/margin/order", order, self.recv_window).await
    }

    /// Get all open margin orders for a symbol
    pub async fn margin_get_open_orders(&self, symbol: impl Into<String>) -> Result<Vec<MarginOrder>> {
        let payload = build_signed_request_p(PairQuery { symbol: symbol.into() }, self.recv_window)?;
        self.client.get_signed("/papi/v1/margin/openOrders", &payload).await
    }

    /// Cancel a margin order
    pub async fn margin_cancel_order(&self, symbol: impl Into<String>, order_id: u64) -> Result<MarginOrder> {
        self.client
            .delete_signed_p(
                "/papi/v1/margin/order",
                OrderIdQuery {
                    symbol: symbol.into(),
                    order_id,
                },
                self.recv_window,
            )
            .await
    }

    /// Borrow margin funds
    pub async fn margin_borrow(&self, asset: impl Into<String>, amount: f64) -> Result<RepayResult> {
        let mut params = BTreeMap::<String, String>::new();
        params.insert("asset".into(), asset.into());
        params.insert("amount".into(), amount.to_string());
        let request = build_signed_request(params, self.recv_window)?;
        self.client.post_signed_d("/papi/v1/marginLoan", &request).await
    }

    /// Repay margin loan
    pub async fn margin_repay(&self, asset: impl Into<String>, amount: f64) -> Result<RepayResult> {
        let mut params = BTreeMap::<String, String>::new();
        params.insert("asset".into(), asset.into());
        params.insert("amount".into(), amount.to_string());
        let request = build_signed_request(params, self.recv_window)?;
        self.client.post_signed_d("/papi/v1/repayLoan", &request).await
    }
}
