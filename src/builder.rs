/// Types for composing operation parameters.
use super::types;
use super::{
    encode_path,
    Error,
    HeaderMap,
    HeaderValue,
    ResponseValue, //ByteStream, RequestBuilderExt
};
///Builder for [`Client::get_instrument_candles`]
///
///[`Client::get_instrument_candles`]: super::Client::get_instrument_candles
#[derive(Debug, Clone)]
pub struct GetInstrumentCandles<'a> {
    client: &'a super::Client,
    instrument: Result<String, String>,
    alignment_timezone: Result<Option<String>, String>,
    count: Result<Option<i64>, String>,
    daily_alignment: Result<Option<i64>, String>,
    from: Result<Option<String>, String>,
    granularity: Result<Option<String>, String>,
    include_first: Result<Option<bool>, String>,
    price: Result<Option<String>, String>,
    smooth: Result<Option<bool>, String>,
    to: Result<Option<String>, String>,
    weekly_alignment: Result<Option<String>, String>,
}

impl<'a> GetInstrumentCandles<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            instrument: Err("instrument was not initialized".to_string()),
            alignment_timezone: Ok(None),
            count: Ok(None),
            daily_alignment: Ok(None),
            from: Ok(None),
            granularity: Ok(None),
            include_first: Ok(None),
            price: Ok(None),
            smooth: Ok(None),
            to: Ok(None),
            weekly_alignment: Ok(None),
        }
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn alignment_timezone<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.alignment_timezone = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for alignment_timezone failed".to_string());
        self
    }

    pub fn count<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.count = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for count failed".to_string());
        self
    }

    pub fn daily_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.daily_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for daily_alignment failed".to_string());
        self
    }

    pub fn from<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.from = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for from failed".to_string());
        self
    }

    pub fn granularity<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.granularity = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for granularity failed".to_string());
        self
    }

    pub fn include_first<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.include_first = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for include_first failed".to_string());
        self
    }

    pub fn price<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.price = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for price failed".to_string());
        self
    }

    pub fn smooth<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.smooth = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for smooth failed".to_string());
        self
    }

    pub fn to<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.to = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for to failed".to_string());
        self
    }

    pub fn weekly_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.weekly_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for weekly_alignment failed".to_string());
        self
    }

    ///Sends a `GET` request to `/instruments/{instrument}/candles`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetInstrumentCandlesResponse>,
        Error<types::GetInstrumentCandlesResponse>,
    > {
        let Self {
            client,
            instrument,
            alignment_timezone,
            count,
            daily_alignment,
            from,
            granularity,
            include_first,
            price,
            smooth,
            to,
            weekly_alignment,
        } = self;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let alignment_timezone = alignment_timezone.map_err(Error::InvalidRequest)?;
        let count = count.map_err(Error::InvalidRequest)?;
        let daily_alignment = daily_alignment.map_err(Error::InvalidRequest)?;
        let from = from.map_err(Error::InvalidRequest)?;
        let granularity = granularity.map_err(Error::InvalidRequest)?;
        let include_first = include_first.map_err(Error::InvalidRequest)?;
        let price = price.map_err(Error::InvalidRequest)?;
        let smooth = smooth.map_err(Error::InvalidRequest)?;
        let to = to.map_err(Error::InvalidRequest)?;
        let weekly_alignment = weekly_alignment.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/instruments/{}/candles",
            client.baseurl,
            encode_path(&instrument.to_string()),
        );
        let mut query = Vec::with_capacity(10usize);
        if let Some(v) = &alignment_timezone {
            query.push(("alignmentTimezone", v.to_string()));
        }
        if let Some(v) = &count {
            query.push(("count", v.to_string()));
        }
        if let Some(v) = &daily_alignment {
            query.push(("dailyAlignment", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &granularity {
            query.push(("granularity", v.to_string()));
        }
        if let Some(v) = &include_first {
            query.push(("includeFirst", v.to_string()));
        }
        if let Some(v) = &price {
            query.push(("price", v.to_string()));
        }
        if let Some(v) = &smooth {
            query.push(("smooth", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &weekly_alignment {
            query.push(("weeklyAlignment", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_instrument_order_book`]
///
///[`Client::get_instrument_order_book`]: super::Client::get_instrument_order_book
#[derive(Debug, Clone)]
pub struct GetInstrumentOrderBook<'a> {
    client: &'a super::Client,
    instrument: Result<String, String>,
    time: Result<Option<String>, String>,
}

impl<'a> GetInstrumentOrderBook<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            instrument: Err("instrument was not initialized".to_string()),
            time: Ok(None),
        }
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn time<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.time = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for time failed".to_string());
        self
    }

    ///Sends a `GET` request to `/instruments/{instrument}/orderBook`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetInstrumentOrderBookResponse>,
        Error<types::GetInstrumentOrderBookResponse>,
    > {
        let Self {
            client,
            instrument,
            time,
        } = self;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let time = time.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/instruments/{}/orderBook",
            client.baseurl,
            encode_path(&instrument.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &time {
            query.push(("time", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_instrument_position_book`]
///
///[`Client::get_instrument_position_book`]: super::Client::get_instrument_position_book
#[derive(Debug, Clone)]
pub struct GetInstrumentPositionBook<'a> {
    client: &'a super::Client,
    instrument: Result<String, String>,
    time: Result<Option<String>, String>,
}

impl<'a> GetInstrumentPositionBook<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            instrument: Err("instrument was not initialized".to_string()),
            time: Ok(None),
        }
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn time<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.time = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for time failed".to_string());
        self
    }

    ///Sends a `GET` request to `/instruments/{instrument}/positionBook`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetInstrumentPositionBookResponse>,
        Error<types::GetInstrumentPositionBookResponse>,
    > {
        let Self {
            client,
            instrument,
            time,
        } = self;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let time = time.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/instruments/{}/positionBook",
            client.baseurl,
            encode_path(&instrument.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &time {
            query.push(("time", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_positions`]
///
///[`Client::list_positions`]: super::Client::list_positions
#[derive(Debug, Clone)]
pub struct ListPositions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> ListPositions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/positions`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ListPositionsResponse>, Error<types::ListPositionsResponse>>
    {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/positions",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_open_positions`]
///
///[`Client::list_open_positions`]: super::Client::list_open_positions
#[derive(Debug, Clone)]
pub struct ListOpenPositions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> ListOpenPositions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/openPositions`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::ListOpenPositionsResponse>,
        Error<types::ListOpenPositionsResponse>,
    > {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/openPositions",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_position`]
///
///[`Client::get_position`]: super::Client::get_position
#[derive(Debug, Clone)]
pub struct GetPosition<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    instrument: Result<String, String>,
}

impl<'a> GetPosition<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            instrument: Err("instrument was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/positions/{instrument}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetPositionResponse>, Error<types::GetPositionResponse>> {
        let Self {
            client,
            account_id,
            instrument,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/positions/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&instrument.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::close_position`]
///
///[`Client::close_position`]: super::Client::close_position
#[derive(Debug, Clone)]
pub struct ClosePosition<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    instrument: Result<String, String>,

    body: Result<types::builder::ClosePositionBody, String>,
}

impl<'a> ClosePosition<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            instrument: Err("instrument was not initialized".to_string()),

            body: Ok(types::builder::ClosePositionBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::ClosePositionBody>,
        <V as std::convert::TryInto<types::ClosePositionBody>>::Error: std::fmt::Display,
    {
        self.body = value
            .try_into()
            .map(From::from)
            .map_err(|s| format!("conversion to `ClosePositionBody` for body failed: {}", s));
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(types::builder::ClosePositionBody) -> types::builder::ClosePositionBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/positions/{instrument}/close`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ClosePositionResponse>, Error<types::ClosePositionResponse>>
    {
        let Self {
            client,
            account_id,
            instrument,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let instrument = instrument.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| types::ClosePositionBody::try_from(v).map_err(|e| e.to_string()))
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/positions/{}/close",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&instrument.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_trades`]
///
///[`Client::list_trades`]: super::Client::list_trades
#[derive(Debug, Clone)]
pub struct ListTrades<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    before_id: Result<Option<String>, String>,
    count: Result<Option<i64>, String>,
    ids: Result<Option<Vec<String>>, String>,
    instrument: Result<Option<String>, String>,
    state: Result<Option<String>, String>,
}

impl<'a> ListTrades<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            before_id: Ok(None),
            count: Ok(None),
            ids: Ok(None),
            instrument: Ok(None),
            state: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn before_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.before_id = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for before_id failed".to_string());
        self
    }

    pub fn count<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.count = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for count failed".to_string());
        self
    }

    pub fn ids<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.ids = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
        self
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn state<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.state = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for state failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/trades`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ListTradesResponse>, Error<types::ListTradesResponse>> {
        let Self {
            client,
            account_id,
            before_id,
            count,
            ids,
            instrument,
            state,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let before_id = before_id.map_err(Error::InvalidRequest)?;
        let count = count.map_err(Error::InvalidRequest)?;
        let ids = ids.map_err(Error::InvalidRequest)?;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let state = state.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/trades",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(v) = &before_id {
            query.push(("beforeID", v.to_string()));
        }
        if let Some(v) = &count {
            query.push(("count", v.to_string()));
        }
        if let Some(v) = &ids {
            query.push(("ids", v.first().cloned().unwrap_or_default().to_string()));
            // TODO FIX
        }
        if let Some(v) = &instrument {
            query.push(("instrument", v.to_string()));
        }
        if let Some(v) = &state {
            query.push(("state", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_open_trades`]
///
///[`Client::list_open_trades`]: super::Client::list_open_trades
#[derive(Debug, Clone)]
pub struct ListOpenTrades<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> ListOpenTrades<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/openTrades`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ListOpenTradesResponse>, Error<types::ListOpenTradesResponse>>
    {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/openTrades",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_trade`]
///
///[`Client::get_trade`]: super::Client::get_trade
#[derive(Debug, Clone)]
pub struct GetTrade<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    trade_specifier: Result<String, String>,
}

impl<'a> GetTrade<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            trade_specifier: Err("trade_specifier was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn trade_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.trade_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for trade_specifier failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetTradeResponse>, Error<types::GetTradeResponse>> {
        let Self {
            client,
            account_id,
            trade_specifier,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let trade_specifier = trade_specifier.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/trades/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&trade_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::close_trade`]
///
///[`Client::close_trade`]: super::Client::close_trade
#[derive(Debug, Clone)]
pub struct CloseTrade<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    trade_specifier: Result<String, String>,

    body: Result<types::builder::CloseTradeBody, String>,
}

impl<'a> CloseTrade<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            trade_specifier: Err("trade_specifier was not initialized".to_string()),

            body: Ok(types::builder::CloseTradeBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn trade_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.trade_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for trade_specifier failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::CloseTradeBody>,
        <V as std::convert::TryInto<types::CloseTradeBody>>::Error: std::fmt::Display,
    {
        self.body = value
            .try_into()
            .map(From::from)
            .map_err(|s| format!("conversion to `CloseTradeBody` for body failed: {}", s));
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(types::builder::CloseTradeBody) -> types::builder::CloseTradeBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/close`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::CloseTradeResponse>, Error<types::CloseTradeResponse>> {
        let Self {
            client,
            account_id,
            trade_specifier,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let trade_specifier = trade_specifier.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| types::CloseTradeBody::try_from(v).map_err(|e| e.to_string()))
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/trades/{}/close",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&trade_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::set_trade_client_extensions`]
///
///[`Client::set_trade_client_extensions`]: super::Client::set_trade_client_extensions
#[derive(Debug, Clone)]
pub struct SetTradeClientExtensions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    trade_specifier: Result<String, String>,

    body: Result<types::builder::SetTradeClientExtensionsBody, String>,
}

impl<'a> SetTradeClientExtensions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            trade_specifier: Err("trade_specifier was not initialized".to_string()),

            body: Ok(types::builder::SetTradeClientExtensionsBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn trade_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.trade_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for trade_specifier failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::SetTradeClientExtensionsBody>,
        <V as std::convert::TryInto<types::SetTradeClientExtensionsBody>>::Error: std::fmt::Display,
    {
        self.body = value.try_into().map(From::from).map_err(|s| {
            format!(
                "conversion to `SetTradeClientExtensionsBody` for body failed: {}",
                s
            )
        });
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(
            types::builder::SetTradeClientExtensionsBody,
        ) -> types::builder::SetTradeClientExtensionsBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::SetTradeClientExtensionsResponse>,
        Error<types::SetTradeClientExtensionsResponse>,
    > {
        let Self {
            client,
            account_id,
            trade_specifier,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let trade_specifier = trade_specifier.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| {
                types::SetTradeClientExtensionsBody::try_from(v).map_err(|e| e.to_string())
            })
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/trades/{}/clientExtensions",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&trade_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::set_trade_dependent_orders`]
///
///[`Client::set_trade_dependent_orders`]: super::Client::set_trade_dependent_orders
#[derive(Debug, Clone)]
pub struct SetTradeDependentOrders<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    trade_specifier: Result<String, String>,

    body: Result<types::builder::SetTradeDependentOrdersBody, String>,
}

impl<'a> SetTradeDependentOrders<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            trade_specifier: Err("trade_specifier was not initialized".to_string()),

            body: Ok(types::builder::SetTradeDependentOrdersBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn trade_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.trade_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for trade_specifier failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::SetTradeDependentOrdersBody>,
        <V as std::convert::TryInto<types::SetTradeDependentOrdersBody>>::Error: std::fmt::Display,
    {
        self.body = value.try_into().map(From::from).map_err(|s| {
            format!(
                "conversion to `SetTradeDependentOrdersBody` for body failed: {}",
                s
            )
        });
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(
            types::builder::SetTradeDependentOrdersBody,
        ) -> types::builder::SetTradeDependentOrdersBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/orders`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::SetTradeDependentOrdersResponse>,
        Error<types::SetTradeDependentOrdersResponse>,
    > {
        let Self {
            client,
            account_id,
            trade_specifier,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let trade_specifier = trade_specifier.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| {
                types::SetTradeDependentOrdersBody::try_from(v).map_err(|e| e.to_string())
            })
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/trades/{}/orders",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&trade_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_accounts`]
///
///[`Client::list_accounts`]: super::Client::list_accounts
#[derive(Debug, Clone)]
pub struct ListAccounts<'a> {
    client: &'a super::Client,
}

impl<'a> ListAccounts<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self { client: client }
    }

    ///Sends a `GET` request to `/accounts`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ListAccountsResponse>, Error<types::ListAccountsResponse>>
    {
        let Self { client } = self;
        let url = format!("{}/accounts", client.baseurl,);
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_account`]
///
///[`Client::get_account`]: super::Client::get_account
#[derive(Debug, Clone)]
pub struct GetAccount<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> GetAccount<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetAccountResponse>, Error<types::GetAccountResponse>> {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_account_summary`]
///
///[`Client::get_account_summary`]: super::Client::get_account_summary
#[derive(Debug, Clone)]
pub struct GetAccountSummary<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> GetAccountSummary<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/summary`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetAccountSummaryResponse>,
        Error<types::GetAccountSummaryResponse>,
    > {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/summary",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_account_instruments`]
///
///[`Client::get_account_instruments`]: super::Client::get_account_instruments
#[derive(Debug, Clone)]
pub struct GetAccountInstruments<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    instruments: Result<Option<Vec<String>>, String>,
}

impl<'a> GetAccountInstruments<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            instruments: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn instruments<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.instruments = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `Vec < String >` for instruments failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/instruments`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetAccountInstrumentsResponse>,
        Error<types::GetAccountInstrumentsResponse>,
    > {
        let Self {
            client,
            account_id,
            instruments,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let instruments = instruments.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/instruments",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &instruments {
            query.push((
                "instruments",
                v.first().cloned().unwrap_or_default().to_string(),
            ));
            // TODO FIX
        }
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::configure_account`]
///
///[`Client::configure_account`]: super::Client::configure_account
#[derive(Debug, Clone)]
pub struct ConfigureAccount<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,

    body: Result<types::builder::ConfigureAccountBody, String>,
}

impl<'a> ConfigureAccount<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),

            body: Ok(types::builder::ConfigureAccountBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::ConfigureAccountBody>,
        <V as std::convert::TryInto<types::ConfigureAccountBody>>::Error: std::fmt::Display,
    {
        self.body = value.try_into().map(From::from).map_err(|s| {
            format!(
                "conversion to `ConfigureAccountBody` for body failed: {}",
                s
            )
        });
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(
            types::builder::ConfigureAccountBody,
        ) -> types::builder::ConfigureAccountBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PATCH` request to `/accounts/{accountID}/configuration`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::ConfigureAccountResponse>,
        Error<types::ConfigureAccountResponse>,
    > {
        let Self {
            client,
            account_id,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| types::ConfigureAccountBody::try_from(v).map_err(|e| e.to_string()))
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/configuration",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .patch(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_account_changes`]
///
///[`Client::get_account_changes`]: super::Client::get_account_changes
#[derive(Debug, Clone)]
pub struct GetAccountChanges<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    since_transaction_id: Result<Option<String>, String>,
}

impl<'a> GetAccountChanges<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            since_transaction_id: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn since_transaction_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.since_transaction_id = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for since_transaction_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/changes`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetAccountChangesResponse>,
        Error<types::GetAccountChangesResponse>,
    > {
        let Self {
            client,
            account_id,
            since_transaction_id,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let since_transaction_id = since_transaction_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/changes",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &since_transaction_id {
            query.push(("sinceTransactionID", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            416u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_transactions`]
///
///[`Client::list_transactions`]: super::Client::list_transactions
#[derive(Debug, Clone)]
pub struct ListTransactions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    from: Result<Option<String>, String>,
    page_size: Result<Option<i64>, String>,
    to: Result<Option<String>, String>,
    type_: Result<Option<Vec<String>>, String>,
}

impl<'a> ListTransactions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            from: Ok(None),
            page_size: Ok(None),
            to: Ok(None),
            type_: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn from<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.from = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for from failed".to_string());
        self
    }

    pub fn page_size<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.page_size = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for page_size failed".to_string());
        self
    }

    pub fn to<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.to = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for to failed".to_string());
        self
    }

    pub fn type_<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.type_ = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `Vec < String >` for type_ failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/transactions`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::ListTransactionsResponse>,
        Error<types::ListTransactionsResponse>,
    > {
        let Self {
            client,
            account_id,
            from,
            page_size,
            to,
            type_,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let from = from.map_err(Error::InvalidRequest)?;
        let page_size = page_size.map_err(Error::InvalidRequest)?;
        let to = to.map_err(Error::InvalidRequest)?;
        let type_ = type_.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/transactions",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &page_size {
            query.push(("pageSize", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &type_ {
            query.push(("type", v.first().cloned().unwrap_or_default().to_string()));
            // TODO FIX
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            416u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_transaction`]
///
///[`Client::get_transaction`]: super::Client::get_transaction
#[derive(Debug, Clone)]
pub struct GetTransaction<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    transaction_id: Result<String, String>,
}

impl<'a> GetTransaction<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            transaction_id: Err("transaction_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn transaction_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.transaction_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for transaction_id failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/transactions/{transactionID}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetTransactionResponse>, Error<types::GetTransactionResponse>>
    {
        let Self {
            client,
            account_id,
            transaction_id,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let transaction_id = transaction_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/transactions/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&transaction_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_transaction_range`]
///
///[`Client::get_transaction_range`]: super::Client::get_transaction_range
#[derive(Debug, Clone)]
pub struct GetTransactionRange<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    from: Result<String, String>,
    to: Result<String, String>,
    type_: Result<Option<Vec<String>>, String>,
}

impl<'a> GetTransactionRange<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            from: Err("from was not initialized".to_string()),
            to: Err("to was not initialized".to_string()),
            type_: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn from<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.from = value
            .try_into()
            .map_err(|_| "conversion to `String` for from failed".to_string());
        self
    }

    pub fn to<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.to = value
            .try_into()
            .map_err(|_| "conversion to `String` for to failed".to_string());
        self
    }

    pub fn type_<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.type_ = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `Vec < String >` for type_ failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/transactions/idrange`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetTransactionRangeResponse>,
        Error<types::GetTransactionRangeResponse>,
    > {
        let Self {
            client,
            account_id,
            from,
            to,
            type_,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let from = from.map_err(Error::InvalidRequest)?;
        let to = to.map_err(Error::InvalidRequest)?;
        let type_ = type_.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/transactions/idrange",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        query.push(("from", from.to_string()));
        query.push(("to", to.to_string()));
        if let Some(v) = &type_ {
            query.push(("type", v.first().cloned().unwrap_or_default().to_string()));
            // TODO FIX
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            416u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_transactions_since_id`]
///
///[`Client::get_transactions_since_id`]: super::Client::get_transactions_since_id
#[derive(Debug, Clone)]
pub struct GetTransactionsSinceId<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    id: Result<String, String>,
}

impl<'a> GetTransactionsSinceId<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            id: Err("id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.id = value
            .try_into()
            .map_err(|_| "conversion to `String` for id failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/transactions/sinceid`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetTransactionsSinceIdResponse>,
        Error<types::GetTransactionsSinceIdResponse>,
    > {
        let Self {
            client,
            account_id,
            id,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let id = id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/transactions/sinceid",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        query.push(("id", id.to_string()));

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            416u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::stream_transactions`]
///
///[`Client::stream_transactions`]: super::Client::stream_transactions
#[derive(Debug, Clone)]
pub struct StreamTransactions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> StreamTransactions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/transactions/stream`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::StreamTransactionsResponse>,
        Error<types::StreamTransactionsResponse>,
    > {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/transactions/stream",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_latest_candles`]
///
///[`Client::get_latest_candles`]: super::Client::get_latest_candles
#[derive(Debug, Clone)]
pub struct GetLatestCandles<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    alignment_timezone: Result<Option<String>, String>,
    candle_specifications: Result<Vec<String>, String>,
    daily_alignment: Result<Option<i64>, String>,
    smooth: Result<Option<bool>, String>,
    units: Result<Option<String>, String>,
    weekly_alignment: Result<Option<String>, String>,
}

impl<'a> GetLatestCandles<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            alignment_timezone: Ok(None),
            candle_specifications: Err("candle_specifications was not initialized".to_string()),
            daily_alignment: Ok(None),
            smooth: Ok(None),
            units: Ok(None),
            weekly_alignment: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn alignment_timezone<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.alignment_timezone = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for alignment_timezone failed".to_string());
        self
    }

    pub fn candle_specifications<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.candle_specifications = value.try_into().map_err(|_| {
            "conversion to `Vec < String >` for candle_specifications failed".to_string()
        });
        self
    }

    pub fn daily_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.daily_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for daily_alignment failed".to_string());
        self
    }

    pub fn smooth<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.smooth = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for smooth failed".to_string());
        self
    }

    pub fn units<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.units = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for units failed".to_string());
        self
    }

    pub fn weekly_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.weekly_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for weekly_alignment failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/candles/latest`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetLatestCandlesResponse>,
        Error<types::GetLatestCandlesResponse>,
    > {
        let Self {
            client,
            account_id,
            alignment_timezone,
            candle_specifications,
            daily_alignment,
            smooth,
            units,
            weekly_alignment,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let alignment_timezone = alignment_timezone.map_err(Error::InvalidRequest)?;
        let candle_specifications = candle_specifications.map_err(Error::InvalidRequest)?;
        let daily_alignment = daily_alignment.map_err(Error::InvalidRequest)?;
        let smooth = smooth.map_err(Error::InvalidRequest)?;
        let units = units.map_err(Error::InvalidRequest)?;
        let weekly_alignment = weekly_alignment.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/candles/latest",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(6usize);
        if let Some(v) = &alignment_timezone {
            query.push(("alignmentTimezone", v.to_string()));
        }
        query.push((
            "candleSpecifications",
            candle_specifications
                .first()
                .cloned()
                .unwrap_or_default()
                .to_string(),
        ));
        // TODO FIX
        if let Some(v) = &daily_alignment {
            query.push(("dailyAlignment", v.to_string()));
        }
        if let Some(v) = &smooth {
            query.push(("smooth", v.to_string()));
        }
        if let Some(v) = &units {
            query.push(("units", v.to_string()));
        }
        if let Some(v) = &weekly_alignment {
            query.push(("weeklyAlignment", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_prices`]
///
///[`Client::get_prices`]: super::Client::get_prices
#[derive(Debug, Clone)]
pub struct GetPrices<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    include_home_conversions: Result<Option<bool>, String>,
    include_units_available: Result<Option<bool>, String>,
    instruments: Result<Vec<String>, String>,
    since: Result<Option<String>, String>,
}

impl<'a> GetPrices<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            include_home_conversions: Ok(None),
            include_units_available: Ok(None),
            instruments: Err("instruments was not initialized".to_string()),
            since: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn include_home_conversions<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.include_home_conversions = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for include_home_conversions failed".to_string());
        self
    }

    pub fn include_units_available<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.include_units_available = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for include_units_available failed".to_string());
        self
    }

    pub fn instruments<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.instruments = value
            .try_into()
            .map_err(|_| "conversion to `Vec < String >` for instruments failed".to_string());
        self
    }

    pub fn since<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.since = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for since failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/pricing`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetPricesResponse>, Error<types::GetPricesResponse>> {
        let Self {
            client,
            account_id,
            include_home_conversions,
            include_units_available,
            instruments,
            since,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let include_home_conversions = include_home_conversions.map_err(Error::InvalidRequest)?;
        let include_units_available = include_units_available.map_err(Error::InvalidRequest)?;
        let instruments = instruments.map_err(Error::InvalidRequest)?;
        let since = since.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/pricing",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &include_home_conversions {
            query.push(("includeHomeConversions", v.to_string()));
        }
        if let Some(v) = &include_units_available {
            query.push(("includeUnitsAvailable", v.to_string()));
        }
        query.push((
            "instruments",
            instruments.first().cloned().unwrap_or_default().to_string(),
        ));
        // TODO FIX
        if let Some(v) = &since {
            query.push(("since", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::stream_pricing`]
///
///[`Client::stream_pricing`]: super::Client::stream_pricing
#[derive(Debug, Clone)]
pub struct StreamPricing<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    instruments: Result<Vec<String>, String>,
    snapshot: Result<Option<bool>, String>,
}

impl<'a> StreamPricing<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            instruments: Err("instruments was not initialized".to_string()),
            snapshot: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn instruments<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.instruments = value
            .try_into()
            .map_err(|_| "conversion to `Vec < String >` for instruments failed".to_string());
        self
    }

    pub fn snapshot<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.snapshot = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for snapshot failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/pricing/stream`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::StreamPricingResponse>, Error<types::StreamPricingResponse>>
    {
        let Self {
            client,
            account_id,
            instruments,
            snapshot,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let instruments = instruments.map_err(Error::InvalidRequest)?;
        let snapshot = snapshot.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/pricing/stream",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        query.push((
            "instruments",
            instruments.first().cloned().unwrap_or_default().to_string(),
        ));
        // TODO FIX
        if let Some(v) = &snapshot {
            query.push(("snapshot", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_account_instrument_candles`]
///
///[`Client::get_account_instrument_candles`]: super::Client::get_account_instrument_candles
#[derive(Debug, Clone)]
pub struct GetAccountInstrumentCandles<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    instrument: Result<String, String>,
    alignment_timezone: Result<Option<String>, String>,
    count: Result<Option<i64>, String>,
    daily_alignment: Result<Option<i64>, String>,
    from: Result<Option<String>, String>,
    granularity: Result<Option<String>, String>,
    include_first: Result<Option<bool>, String>,
    price: Result<Option<String>, String>,
    smooth: Result<Option<bool>, String>,
    to: Result<Option<String>, String>,
    units: Result<Option<String>, String>,
    weekly_alignment: Result<Option<String>, String>,
}

impl<'a> GetAccountInstrumentCandles<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            instrument: Err("instrument was not initialized".to_string()),
            alignment_timezone: Ok(None),
            count: Ok(None),
            daily_alignment: Ok(None),
            from: Ok(None),
            granularity: Ok(None),
            include_first: Ok(None),
            price: Ok(None),
            smooth: Ok(None),
            to: Ok(None),
            units: Ok(None),
            weekly_alignment: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn alignment_timezone<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.alignment_timezone = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for alignment_timezone failed".to_string());
        self
    }

    pub fn count<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.count = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for count failed".to_string());
        self
    }

    pub fn daily_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.daily_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for daily_alignment failed".to_string());
        self
    }

    pub fn from<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.from = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for from failed".to_string());
        self
    }

    pub fn granularity<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.granularity = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for granularity failed".to_string());
        self
    }

    pub fn include_first<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.include_first = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for include_first failed".to_string());
        self
    }

    pub fn price<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.price = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for price failed".to_string());
        self
    }

    pub fn smooth<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<bool>,
    {
        self.smooth = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `bool` for smooth failed".to_string());
        self
    }

    pub fn to<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.to = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for to failed".to_string());
        self
    }

    pub fn units<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.units = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for units failed".to_string());
        self
    }

    pub fn weekly_alignment<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.weekly_alignment = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for weekly_alignment failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/instruments/{instrument}/candles`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::GetAccountInstrumentCandlesResponse>,
        Error<types::GetAccountInstrumentCandlesResponse>,
    > {
        let Self {
            client,
            account_id,
            instrument,
            alignment_timezone,
            count,
            daily_alignment,
            from,
            granularity,
            include_first,
            price,
            smooth,
            to,
            units,
            weekly_alignment,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let alignment_timezone = alignment_timezone.map_err(Error::InvalidRequest)?;
        let count = count.map_err(Error::InvalidRequest)?;
        let daily_alignment = daily_alignment.map_err(Error::InvalidRequest)?;
        let from = from.map_err(Error::InvalidRequest)?;
        let granularity = granularity.map_err(Error::InvalidRequest)?;
        let include_first = include_first.map_err(Error::InvalidRequest)?;
        let price = price.map_err(Error::InvalidRequest)?;
        let smooth = smooth.map_err(Error::InvalidRequest)?;
        let to = to.map_err(Error::InvalidRequest)?;
        let units = units.map_err(Error::InvalidRequest)?;
        let weekly_alignment = weekly_alignment.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/instruments/{}/candles",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&instrument.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &alignment_timezone {
            query.push(("alignmentTimezone", v.to_string()));
        }
        if let Some(v) = &count {
            query.push(("count", v.to_string()));
        }
        if let Some(v) = &daily_alignment {
            query.push(("dailyAlignment", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &granularity {
            query.push(("granularity", v.to_string()));
        }
        if let Some(v) = &include_first {
            query.push(("includeFirst", v.to_string()));
        }
        if let Some(v) = &price {
            query.push(("price", v.to_string()));
        }
        if let Some(v) = &smooth {
            query.push(("smooth", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &units {
            query.push(("units", v.to_string()));
        }
        if let Some(v) = &weekly_alignment {
            query.push(("weeklyAlignment", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_orders`]
///
///[`Client::list_orders`]: super::Client::list_orders
#[derive(Debug, Clone)]
pub struct ListOrders<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    before_id: Result<Option<String>, String>,
    count: Result<Option<i64>, String>,
    ids: Result<Option<Vec<String>>, String>,
    instrument: Result<Option<String>, String>,
    state: Result<Option<String>, String>,
}

impl<'a> ListOrders<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            before_id: Ok(None),
            count: Ok(None),
            ids: Ok(None),
            instrument: Ok(None),
            state: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn before_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.before_id = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for before_id failed".to_string());
        self
    }

    pub fn count<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<i64>,
    {
        self.count = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `i64` for count failed".to_string());
        self
    }

    pub fn ids<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<Vec<String>>,
    {
        self.ids = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `Vec < String >` for ids failed".to_string());
        self
    }

    pub fn instrument<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.instrument = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for instrument failed".to_string());
        self
    }

    pub fn state<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.state = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for state failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/orders`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ListOrdersResponse>, Error<types::ListOrdersResponse>> {
        let Self {
            client,
            account_id,
            before_id,
            count,
            ids,
            instrument,
            state,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let before_id = before_id.map_err(Error::InvalidRequest)?;
        let count = count.map_err(Error::InvalidRequest)?;
        let ids = ids.map_err(Error::InvalidRequest)?;
        let instrument = instrument.map_err(Error::InvalidRequest)?;
        let state = state.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/orders",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );
        let mut query = Vec::with_capacity(5usize);
        if let Some(v) = &before_id {
            query.push(("beforeID", v.to_string()));
        }
        if let Some(v) = &count {
            query.push(("count", v.to_string()));
        }
        if let Some(v) = &ids {
            query.push(("ids", v.first().cloned().unwrap_or_default().to_string()));
            // TODO FIX
        }
        if let Some(v) = &instrument {
            query.push(("instrument", v.to_string()));
        }
        if let Some(v) = &state {
            query.push(("state", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::create_order`]
///
///[`Client::create_order`]: super::Client::create_order
#[derive(Debug, Clone)]
pub struct CreateOrder<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,

    body: Result<types::builder::CreateOrderBody, String>,
}

impl<'a> CreateOrder<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),

            body: Ok(types::builder::CreateOrderBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::CreateOrderBody>,
        <V as std::convert::TryInto<types::CreateOrderBody>>::Error: std::fmt::Display,
    {
        self.body = value
            .try_into()
            .map(From::from)
            .map_err(|s| format!("conversion to `CreateOrderBody` for body failed: {}", s));
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(types::builder::CreateOrderBody) -> types::builder::CreateOrderBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `POST` request to `/accounts/{accountID}/orders`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::CreateOrderResponse>, Error<types::CreateOrderResponse>> {
        let Self {
            client,
            account_id,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| types::CreateOrderBody::try_from(v).map_err(|e| e.to_string()))
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/orders",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            403u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::list_pending_orders`]
///
///[`Client::list_pending_orders`]: super::Client::list_pending_orders
#[derive(Debug, Clone)]
pub struct ListPendingOrders<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
}

impl<'a> ListPendingOrders<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    ///Sends a `GET` request to `/accounts/{accountID}/pendingOrders`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::ListPendingOrdersResponse>,
        Error<types::ListPendingOrdersResponse>,
    > {
        let Self { client, account_id } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/pendingOrders",
            client.baseurl,
            encode_path(&account_id.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::get_order`]
///
///[`Client::get_order`]: super::Client::get_order
#[derive(Debug, Clone)]
pub struct GetOrder<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    order_specifier: Result<String, String>,
}

impl<'a> GetOrder<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            order_specifier: Err("order_specifier was not initialized".to_string()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn order_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.order_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for order_specifier failed".to_string());
        self
    }

    ///Sends a `GET` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::GetOrderResponse>, Error<types::GetOrderResponse>> {
        let Self {
            client,
            account_id,
            order_specifier,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let order_specifier = order_specifier.map_err(Error::InvalidRequest)?;

        let url = format!(
            "{}/accounts/{}/orders/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&order_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::replace_order`]
///
///[`Client::replace_order`]: super::Client::replace_order
#[derive(Debug, Clone)]
pub struct ReplaceOrder<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    order_specifier: Result<String, String>,

    client_request_id: Result<Option<String>, String>,
    body: Result<types::builder::ReplaceOrderBody, String>,
}

impl<'a> ReplaceOrder<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            order_specifier: Err("order_specifier was not initialized".to_string()),

            client_request_id: Ok(None),
            body: Ok(types::builder::ReplaceOrderBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn order_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.order_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for order_specifier failed".to_string());
        self
    }

    pub fn client_request_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.client_request_id = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for client_request_id failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::ReplaceOrderBody>,
        <V as std::convert::TryInto<types::ReplaceOrderBody>>::Error: std::fmt::Display,
    {
        self.body = value
            .try_into()
            .map(From::from)
            .map_err(|s| format!("conversion to `ReplaceOrderBody` for body failed: {}", s));
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(types::builder::ReplaceOrderBody) -> types::builder::ReplaceOrderBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::ReplaceOrderResponse>, Error<types::ReplaceOrderResponse>>
    {
        let Self {
            client,
            account_id,
            order_specifier,

            client_request_id,
            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let order_specifier = order_specifier.map_err(Error::InvalidRequest)?;

        let client_request_id = client_request_id.map_err(Error::InvalidRequest)?;
        let body = body
            .and_then(|v| types::ReplaceOrderBody::try_from(v).map_err(|e| e.to_string()))
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/orders/{}",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&order_specifier.to_string()),
        );
        let mut headers = HeaderMap::with_capacity(3usize);
        headers.append(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &client.auth_token))
                .unwrap_or(HeaderValue::from_static("")),
        );

        if let Ok(hv) = HeaderValue::try_from(&client.accept_datetime_format.to_string()) {
            headers.append("Accept-Datetime-Format", hv);
        }
        if let Some(v) = client_request_id {
            headers.append("ClientRequestID", HeaderValue::try_from(v)?);
        }
        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .headers(headers)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::cancel_order`]
///
///[`Client::cancel_order`]: super::Client::cancel_order
#[derive(Debug, Clone)]
pub struct CancelOrder<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    order_specifier: Result<String, String>,

    client_request_id: Result<Option<String>, String>,
}

impl<'a> CancelOrder<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            order_specifier: Err("order_specifier was not initialized".to_string()),

            client_request_id: Ok(None),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn order_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.order_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for order_specifier failed".to_string());
        self
    }

    pub fn client_request_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.client_request_id = value
            .try_into()
            .map(Some)
            .map_err(|_| "conversion to `String` for client_request_id failed".to_string());
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}/cancel`
    pub async fn send(
        self,
    ) -> Result<ResponseValue<types::CancelOrderResponse>, Error<types::CancelOrderResponse>> {
        let Self {
            client,
            account_id,
            order_specifier,

            client_request_id,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let order_specifier = order_specifier.map_err(Error::InvalidRequest)?;

        let client_request_id = client_request_id.map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/orders/{}/cancel",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&order_specifier.to_string()),
        );
        let mut headers = HeaderMap::with_capacity(3usize);
        headers.append(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", &client.auth_token))
                .unwrap_or(HeaderValue::from_static("")),
        );

        if let Ok(hv) = HeaderValue::try_from(&client.accept_datetime_format.to_string()) {
            headers.append("Accept-Datetime-Format", hv);
        }
        if let Some(v) = client_request_id {
            headers.append("ClientRequestID", HeaderValue::try_from(v)?);
        }
        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .headers(headers)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

///Builder for [`Client::set_order_client_extensions`]
///
///[`Client::set_order_client_extensions`]: super::Client::set_order_client_extensions
#[derive(Debug, Clone)]
pub struct SetOrderClientExtensions<'a> {
    client: &'a super::Client,
    account_id: Result<String, String>,
    order_specifier: Result<String, String>,

    body: Result<types::builder::SetOrderClientExtensionsBody, String>,
}

impl<'a> SetOrderClientExtensions<'a> {
    pub fn new(client: &'a super::Client) -> Self {
        Self {
            client: client,
            account_id: Err("account_id was not initialized".to_string()),
            order_specifier: Err("order_specifier was not initialized".to_string()),

            body: Ok(types::builder::SetOrderClientExtensionsBody::default()),
        }
    }

    pub fn account_id<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.account_id = value
            .try_into()
            .map_err(|_| "conversion to `String` for account_id failed".to_string());
        self
    }

    pub fn order_specifier<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<String>,
    {
        self.order_specifier = value
            .try_into()
            .map_err(|_| "conversion to `String` for order_specifier failed".to_string());
        self
    }

    pub fn body<V>(mut self, value: V) -> Self
    where
        V: std::convert::TryInto<types::SetOrderClientExtensionsBody>,
        <V as std::convert::TryInto<types::SetOrderClientExtensionsBody>>::Error: std::fmt::Display,
    {
        self.body = value.try_into().map(From::from).map_err(|s| {
            format!(
                "conversion to `SetOrderClientExtensionsBody` for body failed: {}",
                s
            )
        });
        self
    }

    pub fn body_map<F>(mut self, f: F) -> Self
    where
        F: std::ops::FnOnce(
            types::builder::SetOrderClientExtensionsBody,
        ) -> types::builder::SetOrderClientExtensionsBody,
    {
        self.body = self.body.map(f);
        self
    }

    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions`
    pub async fn send(
        self,
    ) -> Result<
        ResponseValue<types::SetOrderClientExtensionsResponse>,
        Error<types::SetOrderClientExtensionsResponse>,
    > {
        let Self {
            client,
            account_id,
            order_specifier,

            body,
        } = self;
        let account_id = account_id.map_err(Error::InvalidRequest)?;
        let order_specifier = order_specifier.map_err(Error::InvalidRequest)?;

        let body = body
            .and_then(|v| {
                types::SetOrderClientExtensionsBody::try_from(v).map_err(|e| e.to_string())
            })
            .map_err(Error::InvalidRequest)?;
        let url = format!(
            "{}/accounts/{}/orders/{}/clientExtensions",
            client.baseurl,
            encode_path(&account_id.to_string()),
            encode_path(&order_specifier.to_string()),
        );

        #[allow(unused_mut)]
        let mut request = client
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = client.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            401u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            404u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            405u16 => Err(Error::ErrorResponse(
                ResponseValue::from_response(response).await?,
            )),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
