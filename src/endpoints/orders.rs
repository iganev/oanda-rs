//! Order endpoints: creating, listing, replacing and cancelling orders.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::{
    OrderCancelRejectTransaction, OrderCancelTransaction,
    OrderClientExtensionsModifyRejectTransaction, OrderClientExtensionsModifyTransaction,
    OrderFillTransaction, Transaction,
};
use crate::models::{
    AccountId, ClientExtensions, InstrumentName, Order, OrderId, OrderRequest, OrderSpecifier,
    OrderStateFilter, TransactionId,
};

impl Client {
    /// Create an order for an account.
    ///
    /// `POST /v3/accounts/{accountID}/orders`
    ///
    /// # Errors
    ///
    /// Rejections (HTTP 400/404) carry an `orderRejectTransaction`; recover
    /// it with
    /// [`ApiErrorBody::details::<CreateOrderRejectBody>`](crate::ApiErrorBody::details).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use oanda_rs::models::{MarketOrderRequest, StopLossDetails, TakeProfitDetails};
    ///
    /// let response = client
    ///     .create_order(
    ///         "101-004-1234567-001",
    ///         MarketOrderRequest::new("EUR_USD", 100)
    ///             .take_profit_on_fill(TakeProfitDetails::at_price("1.1050".parse().unwrap()))
    ///             .stop_loss_on_fill(StopLossDetails::at_distance("0.0050".parse().unwrap())),
    ///     )
    ///     .await?;
    /// println!("created: {:?}", response.order_create_transaction.id());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_order(
        &self,
        account_id: impl Into<AccountId>,
        order: impl Into<OrderRequest>,
    ) -> Result<CreateOrderResponse, Error> {
        let account_id = account_id.into();
        let request = self
            .post(&["accounts", account_id.as_str(), "orders"])
            .json(&OrderRequestBody {
                order: order.into(),
            });
        let (mut response, headers): (CreateOrderResponse, _) =
            self.execute_with_headers(request).await?;
        response.location = crate::transport::header_str(&headers, "Location").map(str::to_owned);
        Ok(response)
    }

    /// Get a list of orders for an account.
    ///
    /// `GET /v3/accounts/{accountID}/orders`
    pub fn list_orders(&self, account_id: impl Into<AccountId>) -> ListOrdersRequest {
        ListOrdersRequest {
            client: self.clone(),
            account_id: account_id.into(),
            params: Vec::new(),
        }
    }

    /// List all pending orders in an account.
    ///
    /// `GET /v3/accounts/{accountID}/pendingOrders`
    pub async fn list_pending_orders(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<ListOrdersResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "pendingOrders"]))
            .await
    }

    /// Get details for a single order in an account.
    ///
    /// `GET /v3/accounts/{accountID}/orders/{orderSpecifier}`
    pub async fn order(
        &self,
        account_id: impl Into<AccountId>,
        order: impl Into<OrderSpecifier>,
    ) -> Result<OrderResponse, Error> {
        let account_id = account_id.into();
        let order = order.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "orders", order.as_str()]))
            .await
    }

    /// Replace an order in an account by simultaneously cancelling it and
    /// creating a replacement.
    ///
    /// `PUT /v3/accounts/{accountID}/orders/{orderSpecifier}`
    pub async fn replace_order(
        &self,
        account_id: impl Into<AccountId>,
        order: impl Into<OrderSpecifier>,
        replacement: impl Into<OrderRequest>,
    ) -> Result<ReplaceOrderResponse, Error> {
        let account_id = account_id.into();
        let order = order.into();
        let request = self
            .put(&["accounts", account_id.as_str(), "orders", order.as_str()])
            .json(&OrderRequestBody {
                order: replacement.into(),
            });
        let (mut response, headers): (ReplaceOrderResponse, _) =
            self.execute_with_headers(request).await?;
        response.location = crate::transport::header_str(&headers, "Location").map(str::to_owned);
        Ok(response)
    }

    /// Cancel a pending order in an account.
    ///
    /// `PUT /v3/accounts/{accountID}/orders/{orderSpecifier}/cancel`
    ///
    /// # Errors
    ///
    /// A failed cancellation (HTTP 404) carries an
    /// `orderCancelRejectTransaction`; recover it with
    /// [`ApiErrorBody::details::<CancelOrderRejectBody>`](crate::ApiErrorBody::details).
    pub async fn cancel_order(
        &self,
        account_id: impl Into<AccountId>,
        order: impl Into<OrderSpecifier>,
    ) -> Result<CancelOrderResponse, Error> {
        let account_id = account_id.into();
        let order = order.into();
        self.execute(self.put(&[
            "accounts",
            account_id.as_str(),
            "orders",
            order.as_str(),
            "cancel",
        ]))
        .await
    }

    /// Update the client extensions for an order in an account.
    ///
    /// **Do not set, modify, or delete `client_extensions` if the account
    /// is associated with MT4.**
    ///
    /// `PUT /v3/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions`
    pub fn set_order_client_extensions(
        &self,
        account_id: impl Into<AccountId>,
        order: impl Into<OrderSpecifier>,
    ) -> SetOrderClientExtensionsRequest {
        SetOrderClientExtensionsRequest {
            client: self.clone(),
            account_id: account_id.into(),
            order: order.into(),
            body: SetOrderClientExtensionsBody {
                client_extensions: None,
                trade_client_extensions: None,
            },
        }
    }
}

#[derive(Debug, Serialize)]
struct OrderRequestBody {
    order: OrderRequest,
}

/// Response of [`Client::create_order`] (HTTP 201).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateOrderResponse {
    /// The transaction that created the order.
    #[serde(rename = "orderCreateTransaction")]
    pub order_create_transaction: Transaction,
    /// The transaction that filled the order (only for market orders, or
    /// orders that were immediately filled).
    #[serde(
        rename = "orderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The transaction that cancelled the order (only when the order was
    /// immediately cancelled).
    #[serde(
        rename = "orderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction that reissued the order (only when the order was
    /// partially filled and reissued).
    #[serde(
        rename = "orderReissueTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_transaction: Option<Transaction>,
    /// The transaction that rejected the reissue of the order.
    #[serde(
        rename = "orderReissueRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_reject_transaction: Option<Transaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
    /// The `Location` response header: the URL of the created order.
    #[serde(skip)]
    pub location: Option<String>,
}

/// Typed view of the error body when [`Client::create_order`] or
/// [`Client::replace_order`] is rejected (HTTP 400/404). Recover it via
/// [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateOrderRejectBody {
    /// The transaction that rejected the creation of the order.
    #[serde(
        rename = "orderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reject_transaction: Option<Transaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::list_orders`].
#[derive(Debug)]
pub struct ListOrdersRequest {
    client: Client,
    account_id: AccountId,
    params: Vec<(&'static str, String)>,
}

impl ListOrdersRequest {
    /// Restricts the response to the given order IDs.
    pub fn ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<OrderId>,
    {
        let joined = ids
            .into_iter()
            .map(|id| id.into().0)
            .collect::<Vec<_>>()
            .join(",");
        self.params.push(("ids", joined));
        self
    }

    /// The state to filter the requested orders by (default `PENDING`).
    pub fn state(mut self, state: OrderStateFilter) -> Self {
        self.params.push(("state", state.to_string()));
        self
    }

    /// The instrument to filter the requested orders by.
    pub fn instrument(mut self, instrument: impl Into<InstrumentName>) -> Self {
        self.params
            .push(("instrument", instrument.into().as_str().to_owned()));
        self
    }

    /// The maximum number of orders to return (default 50, maximum 500).
    pub fn count(mut self, count: u32) -> Self {
        self.params.push(("count", count.to_string()));
        self
    }

    /// The maximum order ID to return: only orders with IDs at or below
    /// this are returned.
    pub fn before_id(mut self, before_id: impl Into<OrderId>) -> Self {
        self.params.push(("beforeID", before_id.into().0));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<ListOrdersResponse, Error> {
        let request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "orders"])
            .query(&self.params);
        self.client.execute(request).await
    }
}

/// Response of [`Client::list_orders`] and [`Client::list_pending_orders`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListOrdersResponse {
    /// The list of orders satisfying the request.
    #[serde(rename = "orders", default, skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<Order>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::order`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderResponse {
    /// The details of the requested order.
    #[serde(rename = "order")]
    pub order: Order,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::replace_order`] (HTTP 201).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReplaceOrderResponse {
    /// The transaction that cancelled the order to be replaced.
    #[serde(
        rename = "orderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction that created the replacing order.
    #[serde(
        rename = "orderCreateTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_transaction: Option<Transaction>,
    /// The transaction that filled the replacing order (only when it was
    /// immediately filled).
    #[serde(
        rename = "orderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The transaction that reissued the replacing order.
    #[serde(
        rename = "orderReissueTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_transaction: Option<Transaction>,
    /// The transaction that rejected the reissue of the replacing order.
    #[serde(
        rename = "orderReissueRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reissue_reject_transaction: Option<Transaction>,
    /// The transaction that cancelled the replacing order (only when it
    /// was immediately cancelled).
    #[serde(
        rename = "replacingOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub replacing_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
    /// The `Location` response header: the URL of the replacing order.
    #[serde(skip)]
    pub location: Option<String>,
}

/// Response of [`Client::cancel_order`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelOrderResponse {
    /// The transaction that cancelled the order.
    #[serde(
        rename = "orderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Typed view of the error body when [`Client::cancel_order`] fails
/// (HTTP 404). Recover it via
/// [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CancelOrderRejectBody {
    /// The transaction that rejected the cancellation of the order.
    #[serde(
        rename = "orderCancelRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::set_order_client_extensions`].
#[derive(Debug)]
pub struct SetOrderClientExtensionsRequest {
    client: Client,
    account_id: AccountId,
    order: OrderSpecifier,
    body: SetOrderClientExtensionsBody,
}

#[derive(Debug, Serialize)]
struct SetOrderClientExtensionsBody {
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    client_extensions: Option<ClientExtensions>,
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    trade_client_extensions: Option<ClientExtensions>,
}

impl SetOrderClientExtensionsRequest {
    /// The client extensions to update for the order.
    pub fn client_extensions(mut self, extensions: ClientExtensions) -> Self {
        self.body.client_extensions = Some(extensions);
        self
    }

    /// The client extensions to update for the trade created when the
    /// order fills.
    pub fn trade_client_extensions(mut self, extensions: ClientExtensions) -> Self {
        self.body.trade_client_extensions = Some(extensions);
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<SetOrderClientExtensionsResponse, Error> {
        let request = self
            .client
            .put(&[
                "accounts",
                self.account_id.as_str(),
                "orders",
                self.order.as_str(),
                "clientExtensions",
            ])
            .json(&self.body);
        self.client.execute(request).await
    }
}

/// Response of [`Client::set_order_client_extensions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetOrderClientExtensionsResponse {
    /// The transaction that modified the client extensions.
    #[serde(
        rename = "orderClientExtensionsModifyTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_client_extensions_modify_transaction: Option<OrderClientExtensionsModifyTransaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Typed view of the error body when
/// [`Client::set_order_client_extensions`] is rejected (HTTP 400/404).
/// Recover it via [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetOrderClientExtensionsRejectBody {
    /// The transaction that rejected the modification.
    #[serde(
        rename = "orderClientExtensionsModifyRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_client_extensions_modify_reject_transaction:
        Option<OrderClientExtensionsModifyRejectTransaction>,
    /// The IDs of all transactions created while satisfying the request.
    #[serde(
        rename = "relatedTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_transaction_ids: Vec<TransactionId>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}
