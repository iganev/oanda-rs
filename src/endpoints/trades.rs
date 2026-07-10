//! Trade endpoints: listing, closing, and modifying open trades.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::{
    MarketOrderRejectTransaction, MarketOrderTransaction, OrderCancelRejectTransaction,
    OrderCancelTransaction, OrderFillTransaction, StopLossOrderRejectTransaction,
    StopLossOrderTransaction, TakeProfitOrderRejectTransaction, TakeProfitOrderTransaction,
    TradeClientExtensionsModifyRejectTransaction, TradeClientExtensionsModifyTransaction,
    TrailingStopLossOrderRejectTransaction, TrailingStopLossOrderTransaction,
};
use crate::models::{
    AccountId, ClientExtensions, InstrumentName, StopLossDetails, TakeProfitDetails, Trade,
    TradeId, TradeSpecifier, TradeStateFilter, TrailingStopLossDetails, TransactionId,
};

impl Client {
    /// Get a list of trades for an account.
    ///
    /// `GET /v3/accounts/{accountID}/trades`
    pub fn list_trades(&self, account_id: impl Into<AccountId>) -> ListTradesRequest {
        ListTradesRequest {
            client: self.clone(),
            account_id: account_id.into(),
            params: Vec::new(),
        }
    }

    /// Get the list of open trades for an account.
    ///
    /// `GET /v3/accounts/{accountID}/openTrades`
    pub async fn list_open_trades(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<ListTradesResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "openTrades"]))
            .await
    }

    /// Get the details of a specific trade in an account.
    ///
    /// `GET /v3/accounts/{accountID}/trades/{tradeSpecifier}`
    pub async fn trade(
        &self,
        account_id: impl Into<AccountId>,
        trade: impl Into<TradeSpecifier>,
    ) -> Result<TradeResponse, Error> {
        let account_id = account_id.into();
        let trade = trade.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "trades", trade.as_str()]))
            .await
    }

    /// Close (partially or fully) a specific open trade in an account.
    ///
    /// `PUT /v3/accounts/{accountID}/trades/{tradeSpecifier}/close`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// // Close a trade fully (the default) ...
    /// client.close_trade("101-004-1234567-001", "6543").send().await?;
    /// // ... or partially:
    /// client
    ///     .close_trade("101-004-1234567-001", "6543")
    ///     .units("50")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn close_trade(
        &self,
        account_id: impl Into<AccountId>,
        trade: impl Into<TradeSpecifier>,
    ) -> CloseTradeRequest {
        CloseTradeRequest {
            client: self.clone(),
            account_id: account_id.into(),
            trade: trade.into(),
            units: None,
        }
    }

    /// Update the client extensions for a trade.
    ///
    /// **Do not add, update, or delete the client extensions if your
    /// account is associated with MT4.**
    ///
    /// `PUT /v3/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions`
    pub async fn set_trade_client_extensions(
        &self,
        account_id: impl Into<AccountId>,
        trade: impl Into<TradeSpecifier>,
        extensions: ClientExtensions,
    ) -> Result<SetTradeClientExtensionsResponse, Error> {
        let account_id = account_id.into();
        let trade = trade.into();
        let request = self
            .put(&[
                "accounts",
                account_id.as_str(),
                "trades",
                trade.as_str(),
                "clientExtensions",
            ])
            .json(&SetTradeClientExtensionsBody {
                client_extensions: extensions,
            });
        self.execute(request).await
    }

    /// Create, replace and cancel the dependent orders (take-profit,
    /// stop-loss and trailing stop-loss) of a trade.
    ///
    /// Setting a detail replaces the existing dependent order (or creates
    /// one); setting it to "cancel" removes it. Details not set are left
    /// unmodified.
    ///
    /// `PUT /v3/accounts/{accountID}/trades/{tradeSpecifier}/orders`
    pub fn set_trade_dependent_orders(
        &self,
        account_id: impl Into<AccountId>,
        trade: impl Into<TradeSpecifier>,
    ) -> SetTradeDependentOrdersRequest {
        SetTradeDependentOrdersRequest {
            client: self.clone(),
            account_id: account_id.into(),
            trade: trade.into(),
            body: SetTradeDependentOrdersBody {
                take_profit: None,
                stop_loss: None,
                trailing_stop_loss: None,
            },
        }
    }
}

/// Builder for [`Client::list_trades`].
#[derive(Debug)]
pub struct ListTradesRequest {
    client: Client,
    account_id: AccountId,
    params: Vec<(&'static str, String)>,
}

impl ListTradesRequest {
    /// Restricts the response to the given trade IDs.
    pub fn ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<TradeId>,
    {
        let joined = ids
            .into_iter()
            .map(|id| id.into().0)
            .collect::<Vec<_>>()
            .join(",");
        self.params.push(("ids", joined));
        self
    }

    /// The state to filter the requested trades by (default `OPEN`).
    pub fn state(mut self, state: TradeStateFilter) -> Self {
        self.params.push(("state", state.to_string()));
        self
    }

    /// The instrument to filter the requested trades by.
    pub fn instrument(mut self, instrument: impl Into<InstrumentName>) -> Self {
        self.params
            .push(("instrument", instrument.into().as_str().to_owned()));
        self
    }

    /// The maximum number of trades to return (default 50, maximum 500).
    pub fn count(mut self, count: u32) -> Self {
        self.params.push(("count", count.to_string()));
        self
    }

    /// The maximum trade ID to return: only trades with IDs at or below
    /// this are returned.
    pub fn before_id(mut self, before_id: impl Into<TradeId>) -> Self {
        self.params.push(("beforeID", before_id.into().0));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<ListTradesResponse, Error> {
        let request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "trades"])
            .query(&self.params);
        self.client.execute(request).await
    }
}

/// Response of [`Client::list_trades`] and [`Client::list_open_trades`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListTradesResponse {
    /// The list of trades satisfying the request.
    #[serde(rename = "trades", default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<Trade>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::trade`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeResponse {
    /// The details of the requested trade.
    #[serde(rename = "trade")]
    pub trade: Trade,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::close_trade`].
#[derive(Debug)]
pub struct CloseTradeRequest {
    client: Client,
    account_id: AccountId,
    trade: TradeSpecifier,
    units: Option<String>,
}

#[derive(Debug, Serialize)]
struct CloseTradeBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<String>,
}

impl CloseTradeRequest {
    /// How much of the trade to close: either `"ALL"` (the default) or a
    /// number of units as a decimal string (e.g. `"50"`).
    pub fn units(mut self, units: impl Into<String>) -> Self {
        self.units = Some(units.into());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<CloseTradeResponse, Error> {
        let request = self
            .client
            .put(&[
                "accounts",
                self.account_id.as_str(),
                "trades",
                self.trade.as_str(),
                "close",
            ])
            .json(&CloseTradeBody { units: self.units });
        self.client.execute(request).await
    }
}

/// Response of [`Client::close_trade`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CloseTradeResponse {
    /// The market order created to close the trade.
    #[serde(
        rename = "orderCreateTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_transaction: Option<MarketOrderTransaction>,
    /// The fill of the close-trade market order.
    #[serde(
        rename = "orderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction: Option<OrderFillTransaction>,
    /// The cancellation of the close-trade market order (when it could not
    /// be filled).
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

/// Typed view of the error body when [`Client::close_trade`] is rejected
/// (HTTP 400/404). Recover it via
/// [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CloseTradeRejectBody {
    /// The market order reject transaction.
    #[serde(
        rename = "orderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_reject_transaction: Option<MarketOrderRejectTransaction>,
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

#[derive(Debug, Serialize)]
struct SetTradeClientExtensionsBody {
    #[serde(rename = "clientExtensions")]
    client_extensions: ClientExtensions,
}

/// Response of [`Client::set_trade_client_extensions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetTradeClientExtensionsResponse {
    /// The transaction that updated the trade's client extensions.
    #[serde(
        rename = "tradeClientExtensionsModifyTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify_transaction: Option<TradeClientExtensionsModifyTransaction>,
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
/// [`Client::set_trade_client_extensions`] is rejected (HTTP 400/404).
/// Recover it via [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetTradeClientExtensionsRejectBody {
    /// The transaction that rejected the modification.
    #[serde(
        rename = "tradeClientExtensionsModifyRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify_reject_transaction:
        Option<TradeClientExtensionsModifyRejectTransaction>,
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

/// Builder for [`Client::set_trade_dependent_orders`].
#[derive(Debug)]
pub struct SetTradeDependentOrdersRequest {
    client: Client,
    account_id: AccountId,
    trade: TradeSpecifier,
    body: SetTradeDependentOrdersBody,
}

#[derive(Debug, Serialize)]
struct SetTradeDependentOrdersBody {
    #[serde(rename = "takeProfit", skip_serializing_if = "Option::is_none")]
    take_profit: Option<Option<TakeProfitDetails>>,
    #[serde(rename = "stopLoss", skip_serializing_if = "Option::is_none")]
    stop_loss: Option<Option<StopLossDetails>>,
    #[serde(rename = "trailingStopLoss", skip_serializing_if = "Option::is_none")]
    trailing_stop_loss: Option<Option<TrailingStopLossDetails>>,
}

impl SetTradeDependentOrdersRequest {
    /// Creates or replaces the trade's take-profit order.
    pub fn take_profit(mut self, details: TakeProfitDetails) -> Self {
        self.body.take_profit = Some(Some(details));
        self
    }

    /// Cancels the trade's take-profit order (sends `null`).
    pub fn cancel_take_profit(mut self) -> Self {
        self.body.take_profit = Some(None);
        self
    }

    /// Creates or replaces the trade's stop-loss order.
    pub fn stop_loss(mut self, details: StopLossDetails) -> Self {
        self.body.stop_loss = Some(Some(details));
        self
    }

    /// Cancels the trade's stop-loss order (sends `null`).
    pub fn cancel_stop_loss(mut self) -> Self {
        self.body.stop_loss = Some(None);
        self
    }

    /// Creates or replaces the trade's trailing stop-loss order.
    pub fn trailing_stop_loss(mut self, details: TrailingStopLossDetails) -> Self {
        self.body.trailing_stop_loss = Some(Some(details));
        self
    }

    /// Cancels the trade's trailing stop-loss order (sends `null`).
    pub fn cancel_trailing_stop_loss(mut self) -> Self {
        self.body.trailing_stop_loss = Some(None);
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<SetTradeDependentOrdersResponse, Error> {
        let request = self
            .client
            .put(&[
                "accounts",
                self.account_id.as_str(),
                "trades",
                self.trade.as_str(),
                "orders",
            ])
            .json(&self.body);
        self.client.execute(request).await
    }
}

/// Response of [`Client::set_trade_dependent_orders`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetTradeDependentOrdersResponse {
    /// The transaction cancelling the trade's previous take-profit order.
    #[serde(
        rename = "takeProfitOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction creating the trade's new take-profit order.
    #[serde(
        rename = "takeProfitOrderTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_transaction: Option<TakeProfitOrderTransaction>,
    /// The fill of the new take-profit order (only when it was immediately
    /// filled).
    #[serde(
        rename = "takeProfitOrderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_fill_transaction: Option<OrderFillTransaction>,
    /// The cancellation of the new take-profit order (only when it was
    /// immediately cancelled).
    #[serde(
        rename = "takeProfitOrderCreatedCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction cancelling the trade's previous stop-loss order.
    #[serde(
        rename = "stopLossOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction creating the trade's new stop-loss order.
    #[serde(
        rename = "stopLossOrderTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_transaction: Option<StopLossOrderTransaction>,
    /// The fill of the new stop-loss order (only when it was immediately
    /// filled).
    #[serde(
        rename = "stopLossOrderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_fill_transaction: Option<OrderFillTransaction>,
    /// The cancellation of the new stop-loss order (only when it was
    /// immediately cancelled).
    #[serde(
        rename = "stopLossOrderCreatedCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_created_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction cancelling the trade's previous trailing stop-loss
    /// order.
    #[serde(
        rename = "trailingStopLossOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The transaction creating the trade's new trailing stop-loss order.
    #[serde(
        rename = "trailingStopLossOrderTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_transaction: Option<TrailingStopLossOrderTransaction>,
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
/// [`Client::set_trade_dependent_orders`] is rejected (HTTP 400). Recover
/// it via [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SetTradeDependentOrdersRejectBody {
    /// The rejection of the cancellation of the trade's take-profit order.
    #[serde(
        rename = "takeProfitOrderCancelRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The rejection of the creation of a new take-profit order.
    #[serde(
        rename = "takeProfitOrderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub take_profit_order_reject_transaction: Option<TakeProfitOrderRejectTransaction>,
    /// The rejection of the cancellation of the trade's stop-loss order.
    #[serde(
        rename = "stopLossOrderCancelRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The rejection of the creation of a new stop-loss order.
    #[serde(
        rename = "stopLossOrderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_loss_order_reject_transaction: Option<StopLossOrderRejectTransaction>,
    /// The rejection of the cancellation of the trade's trailing stop-loss
    /// order.
    #[serde(
        rename = "trailingStopLossOrderCancelRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_cancel_reject_transaction: Option<OrderCancelRejectTransaction>,
    /// The rejection of the creation of a new trailing stop-loss order.
    #[serde(
        rename = "trailingStopLossOrderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_reject_transaction: Option<TrailingStopLossOrderRejectTransaction>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}
