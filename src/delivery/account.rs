use crate::account::OrderCancellation;
use crate::client::Client;
use crate::errors::*;
use crate::futures::rest_model::{Transaction, OrderType, CanceledOrder};
use crate::rest_model::{OrderSide, TimeInForce, PairQuery};
use crate::futures::account::{PositionSide, WorkingType, serialize_opt_as_uppercase};


#[derive(Clone)]
pub struct DeliveryAccount {
    pub client: Client,
    pub recv_window: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub position_side: Option<PositionSide>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub time_in_force: Option<TimeInForce>,
    #[serde(rename = "quantity")]
    pub qty: Option<i32>,
    pub reduce_only: Option<bool>,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub close_position: Option<bool>,
    pub activation_price: Option<f64>,
    pub callback_rate: Option<f64>,
    pub working_type: Option<WorkingType>,
    #[serde(serialize_with = "serialize_opt_as_uppercase")]
    pub price_protect: Option<bool>,
    pub new_client_order_id: Option<String>,
}


impl DeliveryAccount {
    async fn post_order(&self, order: OrderRequest) -> Result<Transaction> {
        self.client
            .post_signed_p("/dapi/v1/order", order, self.recv_window)
            .await
    }

    pub async fn limit_buy(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<i32>,
        price: f64,
        time_in_force: TimeInForce,
        client_order_id: impl Into<String>,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Buy,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: Some(client_order_id.into())
        };
        self.post_order(order).await
    }

    pub async fn limit_sell(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<i32>,
        price: f64,
        time_in_force: TimeInForce,
        client_order_id: impl Into<String>,
    ) -> Result<Transaction> {
        let order = OrderRequest {
            symbol: symbol.into(),
            side: OrderSide::Sell,
            position_side: None,
            order_type: OrderType::Limit,
            time_in_force: Some(time_in_force),
            qty: Some(qty.into()),
            reduce_only: None,
            price: Some(price),
            stop_price: None,
            close_position: None,
            activation_price: None,
            callback_rate: None,
            working_type: None,
            price_protect: None,
            new_client_order_id: Some(client_order_id.into())
        };
        self.post_order(order).await
    }

    /// Place a cancellation order
    pub async fn cancel_order(&self, o: OrderCancellation) -> Result<CanceledOrder> {
        let recv_window = o.recv_window.unwrap_or(self.recv_window);
        self.client.delete_signed_p("/dapi/v1/order", &o, recv_window).await
    }

    pub async fn cancel_all_open_orders<S>(&self, symbol: S) -> Result<()>
    where
        S: Into<String>,
    {
        self.client
            .delete_signed_p(
                "/dapi/v1/allOpenOrders",
                PairQuery { symbol: symbol.into() },
                self.recv_window,
            )
            .await?;
        Ok(())
    }
}
