//! Position endpoints: listing and closing positions.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::{
    MarketOrderRejectTransaction, MarketOrderTransaction, OrderCancelTransaction,
    OrderFillTransaction,
};
use crate::models::{AccountId, ClientExtensions, InstrumentName, Position, TransactionId};

impl Client {
    /// List all positions for an account. The positions returned are for
    /// every instrument that has had a position during the lifetime of the
    /// account.
    ///
    /// `GET /v3/accounts/{accountID}/positions`
    pub async fn list_positions(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<ListPositionsResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "positions"]))
            .await
    }

    /// List all open positions for an account. An open position is a
    /// position that currently has a trade open.
    ///
    /// `GET /v3/accounts/{accountID}/openPositions`
    pub async fn list_open_positions(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<ListPositionsResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "openPositions"]))
            .await
    }

    /// Get the details of a single instrument's position in an account.
    ///
    /// `GET /v3/accounts/{accountID}/positions/{instrument}`
    pub async fn position(
        &self,
        account_id: impl Into<AccountId>,
        instrument: impl Into<InstrumentName>,
    ) -> Result<PositionResponse, Error> {
        let account_id = account_id.into();
        let instrument = instrument.into();
        self.execute(self.get(&[
            "accounts",
            account_id.as_str(),
            "positions",
            instrument.as_str(),
        ]))
        .await
    }

    /// Close a position for a specific instrument.
    ///
    /// At least one of [`long_units`](ClosePositionRequest::long_units) /
    /// [`short_units`](ClosePositionRequest::short_units) should be set;
    /// OANDA defaults both to `ALL` when omitted.
    ///
    /// `PUT /v3/accounts/{accountID}/positions/{instrument}/close`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// // Close the whole long side of the EUR_USD position:
    /// client
    ///     .close_position("101-004-1234567-001", "EUR_USD")
    ///     .long_units("ALL")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn close_position(
        &self,
        account_id: impl Into<AccountId>,
        instrument: impl Into<InstrumentName>,
    ) -> ClosePositionRequest {
        ClosePositionRequest {
            client: self.clone(),
            account_id: account_id.into(),
            instrument: instrument.into(),
            body: ClosePositionBody {
                long_units: None,
                long_client_extensions: None,
                short_units: None,
                short_client_extensions: None,
            },
        }
    }
}

/// Response of [`Client::list_positions`] and
/// [`Client::list_open_positions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListPositionsResponse {
    /// The list of positions satisfying the request.
    #[serde(rename = "positions", default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<Position>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::position`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionResponse {
    /// The requested position.
    #[serde(rename = "position")]
    pub position: Position,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::close_position`].
#[derive(Debug)]
pub struct ClosePositionRequest {
    client: Client,
    account_id: AccountId,
    instrument: InstrumentName,
    body: ClosePositionBody,
}

#[derive(Debug, Serialize)]
struct ClosePositionBody {
    #[serde(rename = "longUnits", skip_serializing_if = "Option::is_none")]
    long_units: Option<String>,
    #[serde(
        rename = "longClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    long_client_extensions: Option<ClientExtensions>,
    #[serde(rename = "shortUnits", skip_serializing_if = "Option::is_none")]
    short_units: Option<String>,
    #[serde(
        rename = "shortClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    short_client_extensions: Option<ClientExtensions>,
}

impl ClosePositionRequest {
    /// How much of the long side to close: `"ALL"`, `"NONE"`, or a number
    /// of units as a decimal string.
    pub fn long_units(mut self, units: impl Into<String>) -> Self {
        self.body.long_units = Some(units.into());
        self
    }

    /// Client extensions to add to the market order used to close the long
    /// side.
    pub fn long_client_extensions(mut self, extensions: ClientExtensions) -> Self {
        self.body.long_client_extensions = Some(extensions);
        self
    }

    /// How much of the short side to close: `"ALL"`, `"NONE"`, or a number
    /// of units as a decimal string.
    pub fn short_units(mut self, units: impl Into<String>) -> Self {
        self.body.short_units = Some(units.into());
        self
    }

    /// Client extensions to add to the market order used to close the
    /// short side.
    pub fn short_client_extensions(mut self, extensions: ClientExtensions) -> Self {
        self.body.short_client_extensions = Some(extensions);
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<ClosePositionResponse, Error> {
        let request = self
            .client
            .put(&[
                "accounts",
                self.account_id.as_str(),
                "positions",
                self.instrument.as_str(),
                "close",
            ])
            .json(&self.body);
        self.client.execute(request).await
    }
}

/// Response of [`Client::close_position`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClosePositionResponse {
    /// The market order created to close the long side.
    #[serde(
        rename = "longOrderCreateTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_create_transaction: Option<MarketOrderTransaction>,
    /// The fill of the long-side close order.
    #[serde(
        rename = "longOrderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_fill_transaction: Option<OrderFillTransaction>,
    /// The cancellation of the long-side close order.
    #[serde(
        rename = "longOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_cancel_transaction: Option<OrderCancelTransaction>,
    /// The market order created to close the short side.
    #[serde(
        rename = "shortOrderCreateTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_create_transaction: Option<MarketOrderTransaction>,
    /// The fill of the short-side close order.
    #[serde(
        rename = "shortOrderFillTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_fill_transaction: Option<OrderFillTransaction>,
    /// The cancellation of the short-side close order.
    #[serde(
        rename = "shortOrderCancelTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_cancel_transaction: Option<OrderCancelTransaction>,
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

/// Typed view of the error body when [`Client::close_position`] is
/// rejected (HTTP 400/404). Recover it via
/// [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClosePositionRejectBody {
    /// The rejection of the market order created to close the long side.
    #[serde(
        rename = "longOrderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_order_reject_transaction: Option<MarketOrderRejectTransaction>,
    /// The rejection of the market order created to close the short side.
    #[serde(
        rename = "shortOrderRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_order_reject_transaction: Option<MarketOrderRejectTransaction>,
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
