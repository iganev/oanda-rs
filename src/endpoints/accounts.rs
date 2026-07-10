//! Account endpoints: listing, summaries, tradeable instruments,
//! configuration, and change polling.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::{ClientConfigureRejectTransaction, ClientConfigureTransaction};
use crate::models::{
    Account, AccountChanges, AccountChangesState, AccountId, AccountProperties, AccountSummary,
    DecimalNumber, Instrument, InstrumentName, TransactionId,
};

impl Client {
    /// Get the full details for a single account, including a full list of
    /// its open trades, open positions, and pending orders.
    ///
    /// `GET /v3/accounts/{accountID}`
    pub async fn account(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<AccountResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str()]))
            .await
    }

    /// Poll an account for its current state and changes since a specified
    /// transaction ID.
    ///
    /// This is OANDA's recommended pattern for tracking account state:
    /// fetch [`Client::account`] once, then poll this endpoint with the
    /// last seen transaction ID and apply the returned
    /// [`AccountChanges`] to your snapshot.
    ///
    /// `GET /v3/accounts/{accountID}/changes`
    pub fn account_changes(&self, account_id: impl Into<AccountId>) -> AccountChangesRequest {
        AccountChangesRequest {
            client: self.clone(),
            account_id: account_id.into(),
            since_transaction_id: None,
        }
    }
    /// Get a list of all accounts authorized for the provided token.
    ///
    /// `GET /v3/accounts`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// for account in client.list_accounts().await?.accounts {
    ///     println!("{:?}", account.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_accounts(&self) -> Result<ListAccountsResponse, Error> {
        self.execute(self.get(&["accounts"])).await
    }

    /// Get a summary for a single account.
    ///
    /// `GET /v3/accounts/{accountID}/summary`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// let summary = client.account_summary("101-004-1234567-001").await?.account;
    /// println!("balance: {:?}", summary.balance);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn account_summary(
        &self,
        account_id: impl Into<AccountId>,
    ) -> Result<AccountSummaryResponse, Error> {
        let account_id = account_id.into();
        self.execute(self.get(&["accounts", account_id.as_str(), "summary"]))
            .await
    }

    /// Get the list of tradeable instruments for the given account. The
    /// list of tradeable instruments is dependent on the regulatory division
    /// that the account is located in.
    ///
    /// `GET /v3/accounts/{accountID}/instruments`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use oanda_rs::models::InstrumentName;
    ///
    /// // All instruments:
    /// let all = client.account_instruments("101-004-1234567-001").send().await?;
    /// // Or a specific subset:
    /// let some = client
    ///     .account_instruments("101-004-1234567-001")
    ///     .instruments([InstrumentName::EurUsd, InstrumentName::XauUsd])
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn account_instruments(
        &self,
        account_id: impl Into<AccountId>,
    ) -> AccountInstrumentsRequest {
        AccountInstrumentsRequest {
            client: self.clone(),
            account_id: account_id.into(),
            instruments: None,
        }
    }

    /// Set client-configurable properties of the account: its alias and
    /// margin rate.
    ///
    /// `PATCH /v3/accounts/{accountID}/configuration`
    ///
    /// # Errors
    ///
    /// Rejections (HTTP 400/403) carry a
    /// [`ClientConfigureRejectTransaction`]; recover it with
    /// [`ApiErrorBody::details::<ConfigureAccountRejectBody>`](crate::ApiErrorBody::details).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use oanda_rs::models::DecimalNumber;
    ///
    /// client
    ///     .configure_account("101-004-1234567-001")
    ///     .alias("my-strategy-account")
    ///     .margin_rate("0.02".parse::<DecimalNumber>().unwrap())
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn configure_account(&self, account_id: impl Into<AccountId>) -> ConfigureAccountRequest {
        ConfigureAccountRequest {
            client: self.clone(),
            account_id: account_id.into(),
            body: ConfigureAccountBody {
                alias: None,
                margin_rate: None,
            },
        }
    }
}

/// Response of [`Client::account`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountResponse {
    /// The full details of the requested account.
    #[serde(rename = "account")]
    pub account: Account,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::account_changes`].
#[derive(Debug)]
pub struct AccountChangesRequest {
    client: Client,
    account_id: AccountId,
    since_transaction_id: Option<TransactionId>,
}

impl AccountChangesRequest {
    /// The ID of the transaction to get account changes since.
    pub fn since_transaction_id(mut self, id: impl Into<TransactionId>) -> Self {
        self.since_transaction_id = Some(id.into());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<AccountChangesResponse, Error> {
        let mut request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "changes"]);
        if let Some(id) = &self.since_transaction_id {
            request = request.query(&[("sinceTransactionID", id.as_str())]);
        }
        self.client.execute(request).await
    }
}

/// Response of [`Client::account_changes`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountChangesResponse {
    /// The changes to the account's orders, trades and positions since the
    /// specified transaction ID. Only present when a `sinceTransactionID`
    /// was provided.
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<AccountChanges>,
    /// The account's current price-dependent state.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<AccountChangesState>,
    /// The ID of the last transaction created for the account (to be used
    /// as the `sinceTransactionID` of the next poll).
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::list_accounts`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListAccountsResponse {
    /// The list of accounts the client is authorized to access and their
    /// associated properties.
    #[serde(rename = "accounts", default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountProperties>,
}

/// Response of [`Client::account_summary`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountSummaryResponse {
    /// The summary of the requested account.
    #[serde(rename = "account")]
    pub account: AccountSummary,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::account_instruments`].
#[derive(Debug)]
pub struct AccountInstrumentsRequest {
    client: Client,
    account_id: AccountId,
    instruments: Option<Vec<InstrumentName>>,
}

impl AccountInstrumentsRequest {
    /// Restricts the response to the given instruments (defaults to all
    /// instruments tradeable by the account).
    pub fn instruments<I>(mut self, instruments: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<InstrumentName>,
    {
        self.instruments = Some(instruments.into_iter().map(Into::into).collect());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<AccountInstrumentsResponse, Error> {
        let mut request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "instruments"]);
        if let Some(instruments) = &self.instruments {
            request = request.query(&[("instruments", join_names(instruments))]);
        }
        self.client.execute(request).await
    }
}

/// Response of [`Client::account_instruments`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountInstrumentsResponse {
    /// The requested list of instruments.
    #[serde(rename = "instruments", default, skip_serializing_if = "Vec::is_empty")]
    pub instruments: Vec<Instrument>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::configure_account`].
#[derive(Debug)]
pub struct ConfigureAccountRequest {
    client: Client,
    account_id: AccountId,
    body: ConfigureAccountBody,
}

#[derive(Debug, Serialize)]
struct ConfigureAccountBody {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    margin_rate: Option<DecimalNumber>,
}

impl ConfigureAccountRequest {
    /// Sets the client-defined alias for the account.
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.body.alias = Some(alias.into());
        self
    }

    /// Sets the margin rate override for the account.
    pub fn margin_rate(mut self, margin_rate: impl Into<DecimalNumber>) -> Self {
        self.body.margin_rate = Some(margin_rate.into());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<ConfigureAccountResponse, Error> {
        let request = self
            .client
            .patch(&["accounts", self.account_id.as_str(), "configuration"])
            .json(&self.body);
        self.client.execute(request).await
    }
}

/// Response of [`Client::configure_account`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConfigureAccountResponse {
    /// The transaction that configured the account.
    #[serde(
        rename = "clientConfigureTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_configure_transaction: Option<ClientConfigureTransaction>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Typed view of the error body returned when
/// [`Client::configure_account`] is rejected (HTTP 400/403). Recover it via
/// [`ApiErrorBody::details`](crate::ApiErrorBody::details).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConfigureAccountRejectBody {
    /// The transaction that rejected the account configuration.
    #[serde(
        rename = "clientConfigureRejectTransaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_configure_reject_transaction: Option<ClientConfigureRejectTransaction>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Joins instrument names into OANDA's comma-separated list format.
pub(crate) fn join_names(instruments: &[InstrumentName]) -> String {
    instruments
        .iter()
        .map(InstrumentName::as_str)
        .collect::<Vec<_>>()
        .join(",")
}
