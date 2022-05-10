use crate::account::OrderCancellation;
use crate::client::Client;
use crate::errors::*;
use crate::futures::{OrderRequest, OrderSide, OrderType, TimeInForce, PairQuery, rest_model::Transaction};
use crate::rest_model::{OrderSide, TimeInForce, PairAndWindowQuery, PairQuery};
use crate::futures::rest_model::{AccountBalance, CanceledOrder, ChangeLeverageResponse, OrderType, Position, Transaction};


#[derive(Clone)]
pub struct DeliveryAccount {
    pub client: Client,
    pub recv_window: u64,
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
        qty: impl Into<f64>,
        price: f64,
        time_in_force: TimeInForce,
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
        };
        self.post_order(order).await
    }

    pub async fn limit_sell(
        &self,
        symbol: impl Into<String>,
        qty: impl Into<f64>,
        price: f64,
        time_in_force: TimeInForce,
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
