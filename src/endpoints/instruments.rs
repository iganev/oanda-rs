//! Instrument endpoints: candlestick data and order/position books.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Error;
use crate::models::{
    AccountId, CandleSpecification, CandlestickGranularity, DateTime, DecimalNumber,
    InstrumentCandles, InstrumentName, OrderBook, PositionBook, PricingComponent, WeeklyAlignment,
};

impl Client {
    /// Fetch candlestick data for an instrument.
    ///
    /// `GET /v3/instruments/{instrument}/candles`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use oanda_rs::models::CandlestickGranularity;
    ///
    /// let candles = client
    ///     .candles("EUR_USD")
    ///     .granularity(CandlestickGranularity::H1)
    ///     .count(500)
    ///     .send()
    ///     .await?;
    /// println!("got {} candles", candles.candles.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn candles(&self, instrument: impl Into<InstrumentName>) -> CandlesRequest {
        CandlesRequest::new(self.clone(), None, instrument.into())
    }

    /// Fetch candlestick data for an instrument, scoped to an account: the
    /// data matches what the account would see, and volume-weighted average
    /// prices can be requested via
    /// [`units`](CandlesRequest::units).
    ///
    /// `GET /v3/accounts/{accountID}/instruments/{instrument}/candles`
    pub fn account_candles(
        &self,
        account_id: impl Into<AccountId>,
        instrument: impl Into<InstrumentName>,
    ) -> CandlesRequest {
        CandlesRequest::new(self.clone(), Some(account_id.into()), instrument.into())
    }

    /// Get the most recently completed candles within an account for the
    /// specified combinations of instrument, granularity, and price
    /// component.
    ///
    /// `GET /v3/accounts/{accountID}/candles/latest`
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run() -> Result<(), oanda_rs::Error> {
    /// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "token");
    /// use oanda_rs::models::{CandleSpecification, CandlestickGranularity, PricingComponent};
    ///
    /// let latest = client
    ///     .latest_candles(
    ///         "101-004-1234567-001",
    ///         [CandleSpecification::new("EUR_USD", CandlestickGranularity::S10)
    ///             .price(PricingComponent::BID.with_mid())],
    ///     )
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn latest_candles(
        &self,
        account_id: impl Into<AccountId>,
        specifications: impl IntoIterator<Item = CandleSpecification>,
    ) -> LatestCandlesRequest {
        LatestCandlesRequest {
            client: self.clone(),
            account_id: account_id.into(),
            specifications: specifications.into_iter().collect(),
            params: Vec::new(),
        }
    }

    /// Fetch an order book for an instrument.
    ///
    /// `GET /v3/instruments/{instrument}/orderBook`
    pub fn instrument_order_book(&self, instrument: impl Into<InstrumentName>) -> OrderBookRequest {
        OrderBookRequest {
            client: self.clone(),
            instrument: instrument.into(),
            time: None,
        }
    }

    /// Fetch a position book for an instrument.
    ///
    /// `GET /v3/instruments/{instrument}/positionBook`
    pub fn instrument_position_book(
        &self,
        instrument: impl Into<InstrumentName>,
    ) -> PositionBookRequest {
        PositionBookRequest {
            client: self.clone(),
            instrument: instrument.into(),
            time: None,
        }
    }
}

/// Builder for [`Client::candles`] and [`Client::account_candles`].
#[derive(Debug)]
pub struct CandlesRequest {
    client: Client,
    account_id: Option<AccountId>,
    instrument: InstrumentName,
    params: Vec<(&'static str, String)>,
}

impl CandlesRequest {
    fn new(client: Client, account_id: Option<AccountId>, instrument: InstrumentName) -> Self {
        CandlesRequest {
            client,
            account_id,
            instrument,
            params: Vec::new(),
        }
    }

    /// The price component(s) to get candlestick data for (default mid).
    pub fn price(mut self, price: PricingComponent) -> Self {
        self.params.push(("price", price.to_string()));
        self
    }

    /// The granularity of the candlesticks to fetch (default `S5`).
    pub fn granularity(mut self, granularity: CandlestickGranularity) -> Self {
        self.params.push(("granularity", granularity.to_string()));
        self
    }

    /// The number of candlesticks to return (default 500, maximum 5000).
    /// May not be specified when both `from` and `to` are provided.
    pub fn count(mut self, count: u32) -> Self {
        self.params.push(("count", count.to_string()));
        self
    }

    /// The start of the time range to fetch candlesticks for.
    pub fn from(mut self, from: impl Into<DateTime>) -> Self {
        self.params.push(("from", from.into().0));
        self
    }

    /// The end of the time range to fetch candlesticks for.
    pub fn to(mut self, to: impl Into<DateTime>) -> Self {
        self.params.push(("to", to.into().0));
        self
    }

    /// Whether the candlestick is "smoothed" (uses the previous candle's
    /// close as its open; default `false`).
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.params.push(("smooth", smooth.to_string()));
        self
    }

    /// Whether the candlestick covered by the `from` time should be
    /// included (default `true`).
    pub fn include_first(mut self, include_first: bool) -> Self {
        self.params
            .push(("includeFirst", include_first.to_string()));
        self
    }

    /// The hour of the day (in `alignment_timezone`) used for granularities
    /// with daily alignment (default 17).
    pub fn daily_alignment(mut self, hour: u8) -> Self {
        self.params.push(("dailyAlignment", hour.to_string()));
        self
    }

    /// The timezone used for `daily_alignment` (default
    /// `America/New_York`).
    pub fn alignment_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.params.push(("alignmentTimezone", timezone.into()));
        self
    }

    /// The day of the week used for granularities with weekly alignment
    /// (default `Friday`).
    pub fn weekly_alignment(mut self, alignment: WeeklyAlignment) -> Self {
        self.params.push(("weeklyAlignment", alignment.to_string()));
        self
    }

    /// The number of units used to calculate the volume-weighted average
    /// bid/ask prices. Only supported on the account-scoped
    /// [`Client::account_candles`] endpoint.
    pub fn units(mut self, units: impl Into<DecimalNumber>) -> Self {
        self.params.push(("units", units.into().to_string()));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<InstrumentCandles, Error> {
        let request = match &self.account_id {
            Some(account_id) => self.client.get(&[
                "accounts",
                account_id.as_str(),
                "instruments",
                self.instrument.as_str(),
                "candles",
            ]),
            None => self
                .client
                .get(&["instruments", self.instrument.as_str(), "candles"]),
        };
        self.client.execute(request.query(&self.params)).await
    }
}

/// Builder for [`Client::latest_candles`].
#[derive(Debug)]
pub struct LatestCandlesRequest {
    client: Client,
    account_id: AccountId,
    specifications: Vec<CandleSpecification>,
    params: Vec<(&'static str, String)>,
}

impl LatestCandlesRequest {
    /// The number of units used to calculate the volume-weighted average
    /// bid/ask prices.
    pub fn units(mut self, units: impl Into<DecimalNumber>) -> Self {
        self.params.push(("units", units.into().to_string()));
        self
    }

    /// Whether the candlestick is "smoothed" (default `false`).
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.params.push(("smooth", smooth.to_string()));
        self
    }

    /// The hour of the day (in `alignment_timezone`) used for granularities
    /// with daily alignment (default 17).
    pub fn daily_alignment(mut self, hour: u8) -> Self {
        self.params.push(("dailyAlignment", hour.to_string()));
        self
    }

    /// The timezone used for `daily_alignment` (default
    /// `America/New_York`).
    pub fn alignment_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.params.push(("alignmentTimezone", timezone.into()));
        self
    }

    /// The day of the week used for granularities with weekly alignment
    /// (default `Friday`).
    pub fn weekly_alignment(mut self, alignment: WeeklyAlignment) -> Self {
        self.params.push(("weeklyAlignment", alignment.to_string()));
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<LatestCandlesResponse, Error> {
        let specs = self
            .specifications
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let request = self
            .client
            .get(&["accounts", self.account_id.as_str(), "candles", "latest"])
            .query(&[("candleSpecifications", specs)])
            .query(&self.params);
        self.client.execute(request).await
    }
}

/// Response of [`Client::latest_candles`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LatestCandlesResponse {
    /// The latest candle sets for each candle specification.
    #[serde(
        rename = "latestCandles",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub latest_candles: Vec<InstrumentCandles>,
}

/// Builder for [`Client::instrument_order_book`].
#[derive(Debug)]
pub struct OrderBookRequest {
    client: Client,
    instrument: InstrumentName,
    time: Option<DateTime>,
}

impl OrderBookRequest {
    /// The time of the snapshot to fetch. If not specified, then the most
    /// recent snapshot is fetched.
    pub fn time(mut self, time: impl Into<DateTime>) -> Self {
        self.time = Some(time.into());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<OrderBookResponse, Error> {
        let mut request = self
            .client
            .get(&["instruments", self.instrument.as_str(), "orderBook"]);
        if let Some(time) = &self.time {
            request = request.query(&[("time", time.as_str())]);
        }
        let (mut response, headers): (OrderBookResponse, _) =
            self.client.execute_with_headers(request).await?;
        response.link = crate::transport::header_str(&headers, "Link").map(str::to_owned);
        Ok(response)
    }
}

/// Response of [`Client::instrument_order_book`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderBookResponse {
    /// The instrument's order book.
    #[serde(rename = "orderBook")]
    pub order_book: OrderBook,
    /// The `Link` response header, containing links to the next/previous
    /// order book snapshots (when a `time` was requested).
    #[serde(skip)]
    pub link: Option<String>,
}

/// Builder for [`Client::instrument_position_book`].
#[derive(Debug)]
pub struct PositionBookRequest {
    client: Client,
    instrument: InstrumentName,
    time: Option<DateTime>,
}

impl PositionBookRequest {
    /// The time of the snapshot to fetch. If not specified, then the most
    /// recent snapshot is fetched.
    pub fn time(mut self, time: impl Into<DateTime>) -> Self {
        self.time = Some(time.into());
        self
    }

    /// Performs the request.
    pub async fn send(self) -> Result<PositionBookResponse, Error> {
        let mut request =
            self.client
                .get(&["instruments", self.instrument.as_str(), "positionBook"]);
        if let Some(time) = &self.time {
            request = request.query(&[("time", time.as_str())]);
        }
        let (mut response, headers): (PositionBookResponse, _) =
            self.client.execute_with_headers(request).await?;
        response.link = crate::transport::header_str(&headers, "Link").map(str::to_owned);
        Ok(response)
    }
}

/// Response of [`Client::instrument_position_book`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionBookResponse {
    /// The instrument's position book.
    #[serde(rename = "positionBook")]
    pub position_book: PositionBook,
    /// The `Link` response header, containing links to the next/previous
    /// position book snapshots (when a `time` was requested).
    #[serde(skip)]
    pub link: Option<String>,
}
