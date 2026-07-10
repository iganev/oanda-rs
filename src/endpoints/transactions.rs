//! Transaction endpoints: paging, lookup, and range queries over an
//! account's transaction history.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::{Transaction, TransactionFilter};
use crate::models::{AccountId, DateTime, TransactionId};
use crate::streaming::{
    StreamConfig, StreamKind, TransactionKind, TransactionStream, stream_config_setters,
};

impl Client {
    /// Get a list of transaction pages that satisfy a time-based
    /// transaction query. The response contains page URLs, not
    /// transactions; fetch pages via
    /// [`Client::transactions_id_range`].
    ///
    /// `GET /v3/accounts/{accountID}/transactions`
    pub fn list_transactions(&self, account_id: impl Into<AccountId>) -> ListTransactionsRequest {
        ListTransactionsRequest {
            client: self.clone(),
            account_id: account_id.into(),
            params: Vec::new(),
        }
    }

    /// Get the details of a single account transaction.
    ///
    /// `GET /v3/accounts/{accountID}/transactions/{transactionID}`
    pub async fn transaction(
        &self,
        account_id: impl Into<AccountId>,
        transaction_id: impl Into<TransactionId>,
    ) -> Result<TransactionResponse, Error> {
        let account_id = account_id.into();
        let transaction_id = transaction_id.into();
        self.execute(self.get(&[
            "accounts",
            account_id.as_str(),
            "transactions",
            transaction_id.as_str(),
        ]))
        .await
    }

    /// Get a range of transactions for an account based on transaction
    /// IDs.
    ///
    /// `GET /v3/accounts/{accountID}/transactions/idrange`
    pub fn transactions_id_range(
        &self,
        account_id: impl Into<AccountId>,
        from: impl Into<TransactionId>,
        to: impl Into<TransactionId>,
    ) -> TransactionsIdRangeRequest {
        TransactionsIdRangeRequest {
            client: self.clone(),
            account_id: account_id.into(),
            from: from.into(),
            to: to.into(),
            types: None,
        }
    }

    /// Open a stream of the account's transactions (plus a heartbeat
    /// every 5 seconds).
    ///
    /// The returned [`TransactionStream`] manages its own connection: it
    /// detects stale connections via the heartbeat, reconnects with capped
    /// exponential backoff, and **back-fills transactions missed while
    /// disconnected** via [`Client::transactions_since_id`], deduplicating
    /// by transaction ID. See [`crate::streaming`] for details and tuning.
    ///
    /// Connects to the **streaming host**
    /// (`stream-fxpractice`/`stream-fxtrade`). OANDA allows at most 20
    /// active streams per IP.
    ///
    /// `GET /v3/accounts/{accountID}/transactions/stream`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use futures_util::StreamExt;
    /// use oanda_rs::models::transaction::TransactionStreamItem;
    ///
    /// let mut stream = client
    ///     .transaction_stream("101-004-1234567-001")
    ///     .send()
    ///     .await?;
    /// while let Some(item) = stream.next().await {
    ///     if let TransactionStreamItem::Transaction(tx) = item? {
    ///         println!("{:?}: {:?}", tx.type_name(), tx.id());
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn transaction_stream(&self, account_id: impl Into<AccountId>) -> TransactionStreamRequest {
        TransactionStreamRequest {
            client: self.clone(),
            account_id: account_id.into(),
            config: StreamConfig::default(),
        }
    }

    /// Get a range of transactions for an account starting at (but not
    /// including) a provided transaction ID.
    ///
    /// This is the canonical way to back-fill transactions missed while a
    /// transaction stream was disconnected.
    ///
    /// `GET /v3/accounts/{accountID}/transactions/sinceid`
    pub async fn transactions_since_id(
        &self,
        account_id: impl Into<AccountId>,
        id: impl Into<TransactionId>,
    ) -> Result<TransactionsResponse, Error> {
        let account_id = account_id.into();
        let id = id.into();
        let request = self
            .get(&["accounts", account_id.as_str(), "transactions", "sinceid"])
            .query(&[("id", id.as_str())]);
        self.execute(request).await
    }
}

/// Builder for [`Client::transaction_stream`].
#[derive(Debug)]
pub struct TransactionStreamRequest {
    client: Client,
    account_id: AccountId,
    config: StreamConfig,
}

impl TransactionStreamRequest {
    stream_config_setters!();

    /// Connects and returns the managed stream. Fails fast when the
    /// initial connection is rejected (e.g. bad token or account).
    pub async fn send(self) -> Result<TransactionStream, Error> {
        let mut kind = TransactionKind {
            client: self.client,
            account_id: self.account_id,
            last_seen: None,
        };
        let initial = kind.connect(false).await?;
        Ok(TransactionStream::new(kind, self.config, initial))
    }
}

/// Builder for [`Client::list_transactions`].
#[derive(Debug)]
pub struct ListTransactionsRequest {
    client: Client,
    account_id: AccountId,
    params: Vec<(&'static str, String)>,
}

impl ListTransactionsRequest {
    /// The starting time (inclusive) of the time range (default: account
    /// creation time).
    pub fn from(mut self, from: impl Into<DateTime>) -> Self {
        self.params.push(("from", from.into().0));
        self
    }

    /// The ending time (inclusive) of the time range (default: request
    /// time).
    pub fn to(mut self, to: impl Into<DateTime>) -> Self {
        self.params.push(("to", to.into().0));
        self
    }

    /// The number of transactions to include in each page (default 100,
    /// maximum 1000).
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.params.push(("pageSize", page_size.to_string()));
        self
    }

    /// Filters the transactions by type.
    pub fn types<I>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = TransactionFilter>,
    {
        let joined = types
            .into_iter()
            .map(|t| t.as_str().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.params.push(("type", joined));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<ListTransactionsResponse, Error> {
        let request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "transactions"])
            .query(&self.params);
        self.client.execute(request).await
    }
}

/// Response of [`Client::list_transactions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ListTransactionsResponse {
    /// The starting time provided in the request.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<DateTime>,
    /// The ending time provided in the request.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<DateTime>,
    /// The page size provided in the request.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// The transaction-type filter provided in the request.
    #[serde(rename = "type", default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<TransactionFilter>,
    /// The number of transactions that are contained in the pages.
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// The list of URLs that represent idrange queries providing the data
    /// for each page in the query results.
    #[serde(rename = "pages", default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<String>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Response of [`Client::transaction`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransactionResponse {
    /// The details of the requested transaction.
    #[serde(rename = "transaction")]
    pub transaction: Transaction,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}

/// Builder for [`Client::transactions_id_range`].
#[derive(Debug)]
pub struct TransactionsIdRangeRequest {
    client: Client,
    account_id: AccountId,
    from: TransactionId,
    to: TransactionId,
    types: Option<Vec<TransactionFilter>>,
}

impl TransactionsIdRangeRequest {
    /// Filters the transactions by type.
    pub fn types<I>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = TransactionFilter>,
    {
        self.types = Some(types.into_iter().collect());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<TransactionsResponse, Error> {
        let mut request = self
            .client
            .get(&[
                "accounts",
                self.account_id.as_str(),
                "transactions",
                "idrange",
            ])
            .query(&[("from", self.from.as_str()), ("to", self.to.as_str())]);
        if let Some(types) = &self.types {
            let joined = types
                .iter()
                .map(|t| t.as_str().to_owned())
                .collect::<Vec<_>>()
                .join(",");
            request = request.query(&[("type", joined)]);
        }
        self.client.execute(request).await
    }
}

/// Response of [`Client::transactions_id_range`] and
/// [`Client::transactions_since_id`]: a list of full transaction objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransactionsResponse {
    /// The list of transactions that satisfy the request.
    #[serde(
        rename = "transactions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transactions: Vec<Transaction>,
    /// The ID of the most recent transaction created for the account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,
}
