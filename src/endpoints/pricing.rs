//! Pricing endpoints: current prices and the pricing stream.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::{AccountId, ClientPrice, DateTime, HomeConversions, InstrumentName};
use crate::streaming::{
    PricingKind, PricingStream, StreamConfig, StreamKind, stream_config_setters,
};

impl Client {
    /// Get pricing information for a list of instruments within an
    /// account.
    ///
    /// `GET /v3/accounts/{accountID}/pricing`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// let pricing = client
    ///     .prices("101-004-1234567-001", ["EUR_USD", "USD_JPY"])
    ///     .send()
    ///     .await?;
    /// for price in pricing.prices {
    ///     println!("{:?}: {:?}", price.instrument, price.closeout_bid);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn prices<I>(&self, account_id: impl Into<AccountId>, instruments: I) -> PricesRequest
    where
        I: IntoIterator,
        I::Item: Into<InstrumentName>,
    {
        PricesRequest {
            client: self.clone(),
            account_id: account_id.into(),
            instruments: instruments.into_iter().map(Into::into).collect(),
            params: Vec::new(),
        }
    }

    /// Open a stream of price updates (plus a heartbeat every 5 seconds)
    /// for the given instruments.
    ///
    /// The returned [`PricingStream`] manages its own connection: it
    /// detects stale connections via the heartbeat, reconnects with capped
    /// exponential backoff, and requests a fresh price snapshot on every
    /// reconnect. See [`crate::streaming`] for details and tuning.
    ///
    /// Connects to the **streaming host**
    /// (`stream-fxpractice`/`stream-fxtrade`). OANDA allows at most 20
    /// active streams per IP; prices are throttled to at most 4 updates
    /// per second per instrument.
    ///
    /// `GET /v3/accounts/{accountID}/pricing/stream`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use futures_util::StreamExt;
    /// use oanda_rs::models::PriceStreamItem;
    ///
    /// let mut stream = client
    ///     .pricing_stream("101-004-1234567-001", ["EUR_USD"])
    ///     .send()
    ///     .await?;
    /// while let Some(item) = stream.next().await {
    ///     match item? {
    ///         PriceStreamItem::Price(price) => println!("{:?}", price.closeout_bid),
    ///         PriceStreamItem::Heartbeat(_) => {}
    ///         _ => {}
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn pricing_stream<I>(
        &self,
        account_id: impl Into<AccountId>,
        instruments: I,
    ) -> PricingStreamRequest
    where
        I: IntoIterator,
        I::Item: Into<InstrumentName>,
    {
        PricingStreamRequest {
            client: self.clone(),
            account_id: account_id.into(),
            instruments: instruments.into_iter().map(Into::into).collect(),
            snapshot: None,
            config: StreamConfig::default(),
        }
    }
}

/// Builder for [`Client::prices`].
#[derive(Debug)]
pub struct PricesRequest {
    client: Client,
    account_id: AccountId,
    instruments: Vec<InstrumentName>,
    params: Vec<(&'static str, String)>,
}

impl PricesRequest {
    /// Only return prices and home conversions updated after this time
    /// (the response's `time` field can be fed back here when polling).
    pub fn since(mut self, since: impl Into<DateTime>) -> Self {
        self.params.push(("since", since.into().0));
        self
    }

    /// Also include home currency conversion factors in the response.
    pub fn include_home_conversions(mut self, include: bool) -> Self {
        self.params
            .push(("includeHomeConversions", include.to_string()));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<PricesResponse, Error> {
        let request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "pricing"])
            .query(&[(
                "instruments",
                super::accounts::join_names(&self.instruments),
            )])
            .query(&self.params);
        self.client.execute(request).await
    }
}

/// Response of [`Client::prices`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PricesResponse {
    /// The list of price objects requested.
    #[serde(rename = "prices", default, skip_serializing_if = "Vec::is_empty")]
    pub prices: Vec<ClientPrice>,
    /// The home currency conversion factors requested (only present when
    /// `includeHomeConversions` was set).
    #[serde(
        rename = "homeConversions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub home_conversions: Vec<HomeConversions>,
    /// The most recent time any of the prices/conversion factors were
    /// updated; usable as the `since` parameter of the next poll.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
}

/// Builder for [`Client::pricing_stream`].
#[derive(Debug)]
pub struct PricingStreamRequest {
    client: Client,
    account_id: AccountId,
    instruments: Vec<InstrumentName>,
    snapshot: Option<bool>,
    config: StreamConfig,
}

impl PricingStreamRequest {
    /// Whether to send a pricing snapshot of all requested instruments
    /// when the stream (first) connects (OANDA defaults to `true`).
    /// Reconnects always request a snapshot regardless of this setting.
    pub fn snapshot(mut self, snapshot: bool) -> Self {
        self.snapshot = Some(snapshot);
        self
    }

    stream_config_setters!();

    /// Connects and returns the managed stream. Fails fast when the
    /// initial connection is rejected (e.g. bad token or account).
    pub async fn send(self) -> Result<PricingStream, Error> {
        let mut kind = PricingKind {
            client: self.client,
            account_id: self.account_id,
            instruments: super::accounts::join_names(&self.instruments),
            snapshot: self.snapshot,
        };
        let initial = kind.connect(false).await?;
        Ok(PricingStream::new(kind, self.config, initial))
    }
}
