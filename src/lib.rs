pub mod builder;
pub mod prelude;
mod progenitor_client;
pub mod types;

use progenitor_client::encode_path; //RequestBuilderExt
pub use progenitor_client::{ByteStream, Error, ResponseValue};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use types::AcceptDatetimeFormat;

pub enum ApiEnv {
    Practice,
    Trade,
}

pub const API_ENV_BASEURL_PRACTICE: &str = "https://api-fxpractice.oanda.com/v3";
pub const API_ENV_BASEURL_TRADE: &str = "https://api-fxtrade.oanda.com/v3";

#[derive(Clone, Debug)]
///Client for OANDA v20 REST API
///
///The full OANDA v20 REST API Specification. This specification defines how to
/// interact with v20 Accounts, Trades, Orders, Pricing and more.
///
///Version: 3.0.25
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) auth_token: String,
    pub(crate) accept_datetime_format: AcceptDatetimeFormat,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `env` is the `oanda_rs::ApiEnv` that will determine the base URL provided to the internal `reqwest::Client`
    /// `token` is the API token provisioned by OANDA
    /// `datetime_format` is `oanda_rs::types::AcceptDatetimeFormat` and lets you select the datetime format of the API
    pub fn new(env: ApiEnv, token: &str, datetime_format: Option<AcceptDatetimeFormat>) -> Self {
        let baseurl = match env {
            ApiEnv::Practice => API_ENV_BASEURL_PRACTICE,
            ApiEnv::Trade => API_ENV_BASEURL_TRADE,
        };

        let datetime_format = if let Some(datetime_format) = datetime_format {
            datetime_format
        } else {
            AcceptDatetimeFormat::Unix
        };

        #[cfg(not(target_arch = "wasm32"))]
        let client_builder = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };

        #[cfg(target_arch = "wasm32")]
        let client_builder = reqwest::ClientBuilder::new();

        Self::new_with_client(baseurl, token, datetime_format, client_builder)
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(
        baseurl: &str,
        token: &str,
        datetime_format: AcceptDatetimeFormat,
        client_builder: reqwest::ClientBuilder,
    ) -> Self {
        let mut default_headers = HeaderMap::with_capacity(2usize);
        default_headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))
                .unwrap_or(HeaderValue::from_static("")),
        );

        if let Ok(hv) = HeaderValue::try_from(datetime_format.to_string()) {
            default_headers.append("Accept-Datetime-Format", hv);
        }

        Self {
            baseurl: baseurl.to_string(),
            auth_token: token.to_string(),
            accept_datetime_format: datetime_format,
            client: client_builder
                .default_headers(default_headers)
                .build()
                .unwrap(),
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the Auth Token with which requests are made.
    pub fn auth_token(&self) -> &String {
        &self.auth_token
    }

    /// Get the AcceptDatetimeFormat value UNIX / RFC3339
    pub fn accept_datetime_format(&self) -> AcceptDatetimeFormat {
        self.accept_datetime_format
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "3.0.25"
    }
}

impl Client {
    ///Get Candlesticks
    ///
    ///Fetch candlestick data for an instrument.
    ///
    ///Sends a `GET` request to `/instruments/{instrument}/candles`
    ///
    ///Arguments:
    /// - `instrument`: Name of the Instrument
    /// - `alignment_timezone`: The timezone to use for the dailyAlignment
    ///   parameter. Candlesticks with daily alignment will be aligned to the
    ///   dailyAlignment hour within the alignmentTimezone.  Note that the
    ///   returned times will still be represented in UTC.
    /// - `count`: The number of candlesticks to return in the reponse. Count
    ///   should not be specified if both the start and end parameters are
    ///   provided, as the time range combined with the graularity will
    ///   determine the number of candlesticks to return.
    /// - `daily_alignment`: The hour of the day (in the specified timezone) to
    ///   use for granularities that have daily alignments.
    /// - `from`: The start of the time range to fetch candlesticks for.
    /// - `granularity`: The granularity of the candlesticks to fetch
    /// - `include_first`: A flag that controls whether the candlestick that is
    ///   covered by the from time should be included in the results. This flag
    ///   enables clients to use the timestamp of the last completed candlestick
    ///   received to poll for future candlesticks but avoid receiving the
    ///   previous candlestick repeatedly.
    /// - `price`: The Price component(s) to get candlestick data for. Can
    ///   contain any combination of the characters "M" (midpoint candles) "B"
    ///   (bid candles) and "A" (ask candles).
    /// - `smooth`: A flag that controls whether the candlestick is "smoothed"
    ///   or not.  A smoothed candlestick uses the previous candle's close price
    ///   as its open price, while an unsmoothed candlestick uses the first
    ///   price from its time range as its open price.
    /// - `to`: The end of the time range to fetch candlesticks for.
    /// - `weekly_alignment`: The day of the week used for granularities that
    ///   have weekly alignment.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_instrument_candles()
    ///    .instrument(instrument)
    ///    .alignment_timezone(alignment_timezone)
    ///    .count(count)
    ///    .daily_alignment(daily_alignment)
    ///    .from(from)
    ///    .granularity(granularity)
    ///    .include_first(include_first)
    ///    .price(price)
    ///    .smooth(smooth)
    ///    .to(to)
    ///    .weekly_alignment(weekly_alignment)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_instrument_candles(&self) -> builder::GetInstrumentCandles {
        builder::GetInstrumentCandles::new(self)
    }

    ///Get Order Book
    ///
    ///Fetch an order book for an instrument.
    ///
    ///Sends a `GET` request to `/instruments/{instrument}/orderBook`
    ///
    ///Arguments:
    /// - `instrument`: Name of the Instrument
    /// - `time`: The time of the snapshot to fetch. If not specified, then the
    ///   most recent snapshot is fetched.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_instrument_order_book()
    ///    .instrument(instrument)
    ///    .time(time)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_instrument_order_book(&self) -> builder::GetInstrumentOrderBook {
        builder::GetInstrumentOrderBook::new(self)
    }

    ///Get Position Book
    ///
    ///Fetch a position book for an instrument.
    ///
    ///Sends a `GET` request to `/instruments/{instrument}/positionBook`
    ///
    ///Arguments:
    /// - `instrument`: Name of the Instrument
    /// - `time`: The time of the snapshot to fetch. If not specified, then the
    ///   most recent snapshot is fetched.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_instrument_position_book()
    ///    .instrument(instrument)
    ///    .time(time)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_instrument_position_book(&self) -> builder::GetInstrumentPositionBook {
        builder::GetInstrumentPositionBook::new(self)
    }

    ///List Positions
    ///
    ///List all Positions for an Account. The Positions returned are for every
    /// instrument that has had a position during the lifetime of an the
    /// Account.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/positions`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    ///```ignore
    /// let response = client.list_positions()
    ///    .account_id(account_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_positions(&self) -> builder::ListPositions {
        builder::ListPositions::new(self)
    }

    ///Open Positions
    ///
    ///List all open Positions for an Account. An open Position is a Position
    /// in an Account that currently has a Trade opened for it.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/openPositions`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    ///```ignore
    /// let response = client.list_open_positions()
    ///    .account_id(account_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_open_positions(&self) -> builder::ListOpenPositions {
        builder::ListOpenPositions::new(self)
    }

    ///Instrument Position
    ///
    ///Get the details of a single Instrument's Position in an Account. The
    /// Position may by open or not.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/positions/{instrument}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `instrument`: Name of the Instrument
    ///```ignore
    /// let response = client.get_position()
    ///    .account_id(account_id)
    ///    .instrument(instrument)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_position(&self) -> builder::GetPosition {
        builder::GetPosition::new(self)
    }

    ///Close Position
    ///
    ///Closeout the open Position for a specific instrument in an Account.
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/positions/{instrument}/close`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `instrument`: Name of the Instrument
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Representation of how to close the position
    ///```ignore
    /// let response = client.close_position()
    ///    .account_id(account_id)
    ///    .instrument(instrument)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn close_position(&self) -> builder::ClosePosition {
        builder::ClosePosition::new(self)
    }

    ///List Trades
    ///
    ///Get a list of Trades for an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/trades`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `before_id`: The maximum Trade ID to return. If not provided the most
    ///   recent Trades in the Account are returned.
    /// - `count`: The maximum number of Trades to return.
    /// - `ids`: List of Trade IDs to retrieve.
    /// - `instrument`: The instrument to filter the requested Trades by.
    /// - `state`: The state to filter the requested Trades by.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.list_trades()
    ///    .account_id(account_id)
    ///    .before_id(before_id)
    ///    .count(count)
    ///    .ids(ids)
    ///    .instrument(instrument)
    ///    .state(state)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_trades(&self) -> builder::ListTrades {
        builder::ListTrades::new(self)
    }

    ///List Open Trades
    ///
    ///Get the list of open Trades for an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/openTrades`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.list_open_trades()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_open_trades(&self) -> builder::ListOpenTrades {
        builder::ListOpenTrades::new(self)
    }

    ///Trade Details
    ///
    ///Get the details of a specific Trade in an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/trades/{tradeSpecifier}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `trade_specifier`: Specifier for the Trade
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_trade()
    ///    .account_id(account_id)
    ///    .trade_specifier(trade_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_trade(&self) -> builder::GetTrade {
        builder::GetTrade::new(self)
    }

    ///Close Trade
    ///
    ///Close (partially or fully) a specific open Trade in an Account
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/close`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `trade_specifier`: Specifier for the Trade
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Details of how much of the open Trade to close.
    ///```ignore
    /// let response = client.close_trade()
    ///    .account_id(account_id)
    ///    .trade_specifier(trade_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn close_trade(&self) -> builder::CloseTrade {
        builder::CloseTrade::new(self)
    }

    ///Set Trade Client Extensions
    ///
    ///Update the Client Extensions for a Trade. Do not add, update, or delete
    /// the Client Extensions if your account is associated with MT4.
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/clientExtensions`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `trade_specifier`: Specifier for the Trade
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Details of how to modify the Trade's Client Extensions.
    ///```ignore
    /// let response = client.set_trade_client_extensions()
    ///    .account_id(account_id)
    ///    .trade_specifier(trade_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_trade_client_extensions(&self) -> builder::SetTradeClientExtensions {
        builder::SetTradeClientExtensions::new(self)
    }

    ///Set Dependent Orders
    ///
    ///Create, replace and cancel a Trade's dependent Orders (Take Profit, Stop
    /// Loss and Trailing Stop Loss) through the Trade itself
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/trades/{tradeSpecifier}/orders`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `trade_specifier`: Specifier for the Trade
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Details of how to modify the Trade's dependent Orders.
    ///```ignore
    /// let response = client.set_trade_dependent_orders()
    ///    .account_id(account_id)
    ///    .trade_specifier(trade_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_trade_dependent_orders(&self) -> builder::SetTradeDependentOrders {
        builder::SetTradeDependentOrders::new(self)
    }

    ///List Accounts
    ///
    ///Get a list of all Accounts authorized for the provided token.
    ///
    ///Sends a `GET` request to `/accounts`
    ///
    ///```ignore
    /// let response = client.list_accounts()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_accounts(&self) -> builder::ListAccounts {
        builder::ListAccounts::new(self)
    }

    ///Account Details
    ///
    ///Get the full details for a single Account that a client has access to.
    /// Full pending Order, open Trade and open Position representations are
    /// provided.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_account()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account(&self) -> builder::GetAccount {
        builder::GetAccount::new(self)
    }

    ///Account Summary
    ///
    ///Get a summary for a single Account that a client has access to.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/summary`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_account_summary()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account_summary(&self) -> builder::GetAccountSummary {
        builder::GetAccountSummary::new(self)
    }

    ///Account Instruments
    ///
    ///Get the list of tradeable instruments for the given Account. The list of
    /// tradeable instruments is dependent on the regulatory division that the
    /// Account is located in, thus should be the same for all Accounts owned by
    /// a single user.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/instruments`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `instruments`: List of instruments to query specifically.
    ///```ignore
    /// let response = client.get_account_instruments()
    ///    .account_id(account_id)
    ///    .instruments(instruments)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account_instruments(&self) -> builder::GetAccountInstruments {
        builder::GetAccountInstruments::new(self)
    }

    ///Configure Account
    ///
    ///Set the client-configurable portions of an Account.
    ///
    ///Sends a `PATCH` request to `/accounts/{accountID}/configuration`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Representation of the Account configuration to set
    ///```ignore
    /// let response = client.configure_account()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn configure_account(&self) -> builder::ConfigureAccount {
        builder::ConfigureAccount::new(self)
    }

    ///Poll Account Updates
    ///
    ///Endpoint used to poll an Account for its current state and changes since
    /// a specified TransactionID.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/changes`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `since_transaction_id`: ID of the Transaction to get Account changes
    ///   since.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_account_changes()
    ///    .account_id(account_id)
    ///    .since_transaction_id(since_transaction_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account_changes(&self) -> builder::GetAccountChanges {
        builder::GetAccountChanges::new(self)
    }

    ///List Transactions
    ///
    ///Get a list of Transactions pages that satisfy a time-based Transaction
    /// query.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/transactions`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `from`: The starting time (inclusive) of the time range for the
    ///   Transactions being queried.
    /// - `page_size`: The number of Transactions to include in each page of the
    ///   results.
    /// - `to`: The ending time (inclusive) of the time range for the
    ///   Transactions being queried.
    /// - `type_`: A filter for restricting the types of Transactions to
    ///   retreive.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.list_transactions()
    ///    .account_id(account_id)
    ///    .from(from)
    ///    .page_size(page_size)
    ///    .to(to)
    ///    .type_(type_)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_transactions(&self) -> builder::ListTransactions {
        builder::ListTransactions::new(self)
    }

    ///Transaction Details
    ///
    ///Get the details of a single Account Transaction.
    ///
    ///Sends a `GET` request to
    /// `/accounts/{accountID}/transactions/{transactionID}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `transaction_id`: A Transaction ID
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_transaction()
    ///    .account_id(account_id)
    ///    .transaction_id(transaction_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_transaction(&self) -> builder::GetTransaction {
        builder::GetTransaction::new(self)
    }

    ///Transaction ID Range
    ///
    ///Get a range of Transactions for an Account based on the Transaction IDs.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/transactions/idrange`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `from`: The starting Transacion ID (inclusive) to fetch.
    /// - `to`: The ending Transaction ID (inclusive) to fetch.
    /// - `type_`: The filter that restricts the types of Transactions to
    ///   retreive.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_transaction_range()
    ///    .account_id(account_id)
    ///    .from(from)
    ///    .to(to)
    ///    .type_(type_)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_transaction_range(&self) -> builder::GetTransactionRange {
        builder::GetTransactionRange::new(self)
    }

    ///Transactions Since ID
    ///
    ///Get a range of Transactions for an Account starting at (but not
    /// including) a provided Transaction ID.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/transactions/sinceid`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `id`: The ID of the last Transacion fetched. This query will return
    ///   all Transactions newer than the TransactionID.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_transactions_since_id()
    ///    .account_id(account_id)
    ///    .id(id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_transactions_since_id(&self) -> builder::GetTransactionsSinceId {
        builder::GetTransactionsSinceId::new(self)
    }

    ///Transaction Stream
    ///
    ///Get a stream of Transactions for an Account starting from when the
    /// request is made.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/transactions/stream`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    ///```ignore
    /// let response = client.stream_transactions()
    ///    .account_id(account_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn stream_transactions(&self) -> builder::StreamTransactions {
        builder::StreamTransactions::new(self)
    }

    ///Latest account candles
    ///
    ///Get dancing bears and most recently completed candles within an Account
    /// for specified combinations of instrument, granularity, and price
    /// component.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/candles/latest`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `alignment_timezone`: The timezone to use for the dailyAlignment
    ///   parameter. Candlesticks with daily alignment will be aligned to the
    ///   dailyAlignment hour within the alignmentTimezone.  Note that the
    ///   returned times will still be represented in UTC.
    /// - `candle_specifications`: List of candle specifications to get pricing
    ///   for. A string containing the following, all delimited by ":"
    ///   characters: 1) InstrumentName 2) CandlestickGranularity 3)
    ///   PricingComponent e.g. EUR_USD:S10:BM
    /// - `daily_alignment`: The hour of the day (in the specified timezone) to
    ///   use for granularities that have daily alignments.
    /// - `smooth`: A flag that controls whether the candlestick is "smoothed"
    ///   or not.  A smoothed candlestick uses the previous candle's close price
    ///   as its open price, while an unsmoothed candlestick uses the first
    ///   price from its time range as its open price.
    /// - `units`: The number of units used to calculate the volume-weighted
    ///   average bid and ask prices in the returned candles.
    /// - `weekly_alignment`: The day of the week used for granularities that
    ///   have weekly alignment.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_latest_candles()
    ///    .account_id(account_id)
    ///    .alignment_timezone(alignment_timezone)
    ///    .candle_specifications(candle_specifications)
    ///    .daily_alignment(daily_alignment)
    ///    .smooth(smooth)
    ///    .units(units)
    ///    .weekly_alignment(weekly_alignment)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_latest_candles(&self) -> builder::GetLatestCandles {
        builder::GetLatestCandles::new(self)
    }

    ///Current Account Prices
    ///
    ///Get pricing information for a specified list of Instruments within an
    /// Account.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/pricing`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `include_home_conversions`: Flag that enables the inclusion of the
    ///   homeConversions field in the returned response. An entry will be
    ///   returned for each currency in the set of all base and quote currencies
    ///   present in the requested instruments list.
    /// - `include_units_available`: Flag that enables the inclusion of the
    ///   unitsAvailable field in the returned Price objects.
    /// - `instruments`: List of Instruments to get pricing for.
    /// - `since`: Date/Time filter to apply to the response. Only prices and
    ///   home conversions (if requested) with a time later than this filter
    ///   (i.e. the price has changed after the since time) will be provided,
    ///   and are filtered independently.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_prices()
    ///    .account_id(account_id)
    ///    .include_home_conversions(include_home_conversions)
    ///    .include_units_available(include_units_available)
    ///    .instruments(instruments)
    ///    .since(since)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_prices(&self) -> builder::GetPrices {
        builder::GetPrices::new(self)
    }

    ///Price Stream
    ///
    ///Get a stream of Account Prices starting from when the request is made.
    ///This pricing stream does not include every single price created for the
    /// Account, but instead will provide at most 4 prices per second (every 250
    /// milliseconds) for each instrument being requested. If more than one
    /// price is created for an instrument during the 250 millisecond window,
    /// only the price in effect at the end of the window is sent. This means
    /// that during periods of rapid price movement, subscribers to this stream
    /// will not be sent every price. Pricing windows for different
    /// connections to the price stream are not all aligned in the same way
    /// (i.e. they are not all aligned to the top of the second). This means
    /// that during periods of rapid price movement, different subscribers may
    /// observe different prices depending on their alignment.
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/pricing/stream`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `instruments`: List of Instruments to stream Prices for.
    /// - `snapshot`: Flag that enables/disables the sending of a pricing
    ///   snapshot when initially connecting to the stream.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.stream_pricing()
    ///    .account_id(account_id)
    ///    .instruments(instruments)
    ///    .snapshot(snapshot)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn stream_pricing(&self) -> builder::StreamPricing {
        builder::StreamPricing::new(self)
    }

    ///Get Candlesticks
    ///
    ///Fetch candlestick data for an instrument.
    ///
    ///Sends a `GET` request to
    /// `/accounts/{accountID}/instruments/{instrument}/candles`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `instrument`: Name of the Instrument
    /// - `alignment_timezone`: The timezone to use for the dailyAlignment
    ///   parameter. Candlesticks with daily alignment will be aligned to the
    ///   dailyAlignment hour within the alignmentTimezone.  Note that the
    ///   returned times will still be represented in UTC.
    /// - `count`: The number of candlesticks to return in the response. Count
    ///   should not be specified if both the start and end parameters are
    ///   provided, as the time range combined with the granularity will
    ///   determine the number of candlesticks to return.
    /// - `daily_alignment`: The hour of the day (in the specified timezone) to
    ///   use for granularities that have daily alignments.
    /// - `from`: The start of the time range to fetch candlesticks for.
    /// - `granularity`: The granularity of the candlesticks to fetch
    /// - `include_first`: A flag that controls whether the candlestick that is
    ///   covered by the from time should be included in the results. This flag
    ///   enables clients to use the timestamp of the last completed candlestick
    ///   received to poll for future candlesticks but avoid receiving the
    ///   previous candlestick repeatedly.
    /// - `price`: The Price component(s) to get candlestick data for. Can
    ///   contain any combination of the characters "M" (midpoint candles) "B"
    ///   (bid candles) and "A" (ask candles).
    /// - `smooth`: A flag that controls whether the candlestick is "smoothed"
    ///   or not.  A smoothed candlestick uses the previous candle's close price
    ///   as its open price, while an unsmoothed candlestick uses the first
    ///   price from its time range as its open price.
    /// - `to`: The end of the time range to fetch candlesticks for.
    /// - `units`: The number of units used to calculate the volume-weighted
    ///   average bid and ask prices in the returned candles.
    /// - `weekly_alignment`: The day of the week used for granularities that
    ///   have weekly alignment.
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_account_instrument_candles()
    ///    .account_id(account_id)
    ///    .instrument(instrument)
    ///    .alignment_timezone(alignment_timezone)
    ///    .count(count)
    ///    .daily_alignment(daily_alignment)
    ///    .from(from)
    ///    .granularity(granularity)
    ///    .include_first(include_first)
    ///    .price(price)
    ///    .smooth(smooth)
    ///    .to(to)
    ///    .units(units)
    ///    .weekly_alignment(weekly_alignment)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account_instrument_candles(&self) -> builder::GetAccountInstrumentCandles {
        builder::GetAccountInstrumentCandles::new(self)
    }

    ///List Orders
    ///
    ///Get a list of Orders for an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/orders`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `before_id`: The maximum Order ID to return. If not provided the most
    ///   recent Orders in the Account are returned
    /// - `count`: The maximum number of Orders to return
    /// - `ids`: List of Order IDs to retrieve
    /// - `instrument`: The instrument to filter the requested orders by
    /// - `state`: The state to filter the requested Orders by
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.list_orders()
    ///    .account_id(account_id)
    ///    .before_id(before_id)
    ///    .count(count)
    ///    .ids(ids)
    ///    .instrument(instrument)
    ///    .state(state)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_orders(&self) -> builder::ListOrders {
        builder::ListOrders::new(self)
    }

    ///Create Order
    ///
    ///Create an Order for an Account
    ///
    ///Sends a `POST` request to `/accounts/{accountID}/orders`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`
    ///```ignore
    /// let response = client.create_order()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_order(&self) -> builder::CreateOrder {
        builder::CreateOrder::new(self)
    }

    ///Pending Orders
    ///
    ///List all pending Orders in an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/pendingOrders`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.list_pending_orders()
    ///    .account_id(account_id)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_pending_orders(&self) -> builder::ListPendingOrders {
        builder::ListPendingOrders::new(self)
    }

    ///Get Order
    ///
    ///Get details for a single Order in an Account
    ///
    ///Sends a `GET` request to `/accounts/{accountID}/orders/{orderSpecifier}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `order_specifier`: The Order Specifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    ///```ignore
    /// let response = client.get_order()
    ///    .account_id(account_id)
    ///    .order_specifier(order_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_order(&self) -> builder::GetOrder {
        builder::GetOrder::new(self)
    }

    ///Replace Order
    ///
    ///Replace an Order in an Account by simultaneously cancelling it and
    /// creating a replacement Order
    ///
    ///Sends a `PUT` request to `/accounts/{accountID}/orders/{orderSpecifier}`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `order_specifier`: The Order Specifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `client_request_id`: Client specified RequestID to be sent with
    ///   request.
    /// - `body`: Specification of the replacing Order. The replacing order must
    ///   have the same type as the replaced Order.
    ///```ignore
    /// let response = client.replace_order()
    ///    .account_id(account_id)
    ///    .order_specifier(order_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .client_request_id(client_request_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn replace_order(&self) -> builder::ReplaceOrder {
        builder::ReplaceOrder::new(self)
    }

    ///Cancel Order
    ///
    ///Cancel a pending Order in an Account
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}/cancel`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `order_specifier`: The Order Specifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `client_request_id`: Client specified RequestID to be sent with
    ///   request.
    ///```ignore
    /// let response = client.cancel_order()
    ///    .account_id(account_id)
    ///    .order_specifier(order_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .client_request_id(client_request_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cancel_order(&self) -> builder::CancelOrder {
        builder::CancelOrder::new(self)
    }

    ///Set Order Extensions
    ///
    ///Update the Client Extensions for an Order in an Account. Do not set,
    /// modify, or delete clientExtensions if your account is associated with
    /// MT4.
    ///
    ///Sends a `PUT` request to
    /// `/accounts/{accountID}/orders/{orderSpecifier}/clientExtensions`
    ///
    ///Arguments:
    /// - `account_id`: Account Identifier
    /// - `order_specifier`: The Order Specifier
    /// - `accept_datetime_format`: Format of DateTime fields in the request and
    ///   response.
    /// - `body`: Representation of the replacing Order
    ///```ignore
    /// let response = client.set_order_client_extensions()
    ///    .account_id(account_id)
    ///    .order_specifier(order_specifier)
    ///    .accept_datetime_format(accept_datetime_format)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_order_client_extensions(&self) -> builder::SetOrderClientExtensions {
        builder::SetOrderClientExtensions::new(self)
    }
}
