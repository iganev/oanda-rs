//! The OANDA API client.

use std::fmt;
use std::sync::Arc;

use reqwest::Url;

use crate::error::Error;
use crate::models::AcceptDatetimeFormat;
use crate::rate_limit::RateLimiter;

/// Default REST rate limit (requests/second). OANDA rejects above 120/s per
/// IP; the default keeps comfortable headroom.
const DEFAULT_REST_RATE_LIMIT: u32 = 100;
/// OANDA allows at most 2 new connections per second per IP.
const CONNECTIONS_PER_SECOND: u32 = 2;

/// The OANDA environment (host pair) a [`Client`] talks to.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Environment {
    /// The fxTrade Practice (demo) environment:
    /// `api-fxpractice.oanda.com` / `stream-fxpractice.oanda.com`.
    Practice,
    /// The fxTrade live environment:
    /// `api-fxtrade.oanda.com` / `stream-fxtrade.oanda.com`.
    Live,
    /// Custom host pair, mainly for tests and proxies. Both URLs are used
    /// as-is (the `/v3` prefix is **not** appended).
    Custom {
        /// Base URL for REST requests.
        rest: Url,
        /// Base URL for the pricing/transactions streams.
        stream: Url,
    },
}

impl Environment {
    fn rest_base(&self) -> Url {
        match self {
            Environment::Practice => Url::parse("https://api-fxpractice.oanda.com/v3").unwrap(),
            Environment::Live => Url::parse("https://api-fxtrade.oanda.com/v3").unwrap(),
            Environment::Custom { rest, .. } => rest.clone(),
        }
    }

    fn stream_base(&self) -> Url {
        match self {
            Environment::Practice => Url::parse("https://stream-fxpractice.oanda.com/v3").unwrap(),
            Environment::Live => Url::parse("https://stream-fxtrade.oanda.com/v3").unwrap(),
            Environment::Custom { stream, .. } => stream.clone(),
        }
    }
}

pub(crate) struct Inner {
    pub(crate) http: reqwest::Client,
    pub(crate) rest_base: Url,
    pub(crate) stream_base: Url,
    pub(crate) token: String,
    pub(crate) datetime_format: AcceptDatetimeFormat,
    pub(crate) rest_limiter: Option<RateLimiter>,
    pub(crate) conn_limiter: Option<RateLimiter>,
}

/// An asynchronous OANDA v20 API client.
///
/// The client is cheap to clone (all clones share one connection pool and
/// one rate limiter) and is `Send + Sync`, so a single instance can be
/// shared freely between concurrent tokio tasks:
///
/// ```no_run
/// use oanda_rs::{Client, Environment};
///
/// let client = Client::new(Environment::Practice, "my-token");
/// let for_task = client.clone(); // shares pool + rate limiter
/// ```
///
/// Rate limiting is built in and enabled by default (100 REST requests/s,
/// 2 stream connections/s — under OANDA's 120/s and 2/s caps). See
/// [`Client::builder`] for tuning.
#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<Inner>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("rest_base", &self.inner.rest_base.as_str())
            .field("stream_base", &self.inner.stream_base.as_str())
            .field("token", &"<redacted>")
            .field("datetime_format", &self.inner.datetime_format)
            .finish()
    }
}

impl Client {
    /// Creates a client for `environment` authenticating with the given
    /// personal access token, using default settings.
    ///
    /// Use [`Client::builder`] to customize the datetime format, rate
    /// limits, or the underlying HTTP client.
    ///
    /// # Panics
    ///
    /// Panics if the system TLS backend cannot be initialized (same
    /// condition as [`reqwest::Client::new`]).
    pub fn new(environment: Environment, token: impl Into<String>) -> Client {
        Client::builder()
            .environment(environment)
            .token(token)
            .build()
            .expect("default client configuration is valid")
    }

    /// Starts building a client with custom configuration.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// The datetime wire format this client requests via the
    /// `Accept-Datetime-Format` header.
    pub fn datetime_format(&self) -> AcceptDatetimeFormat {
        self.inner.datetime_format
    }
}

/// Configures and builds a [`Client`].
///
/// ```no_run
/// use oanda_rs::{Client, Environment};
/// use oanda_rs::models::AcceptDatetimeFormat;
///
/// let client = Client::builder()
///     .environment(Environment::Practice)
///     .token("my-token")
///     .datetime_format(AcceptDatetimeFormat::Unix)
///     .rest_rate_limit(50)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug)]
pub struct ClientBuilder {
    environment: Environment,
    token: Option<String>,
    datetime_format: AcceptDatetimeFormat,
    http: Option<reqwest::Client>,
    user_agent: String,
    rest_rate_limit: u32,
    rate_limiting: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        ClientBuilder {
            environment: Environment::Practice,
            token: None,
            datetime_format: AcceptDatetimeFormat::Rfc3339,
            http: None,
            user_agent: concat!("oanda-rs/", env!("CARGO_PKG_VERSION")).to_owned(),
            rest_rate_limit: DEFAULT_REST_RATE_LIMIT,
            rate_limiting: true,
        }
    }
}

impl ClientBuilder {
    /// Selects the OANDA environment. Defaults to [`Environment::Practice`].
    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Sets the personal access token used as the bearer token on every
    /// request. Required.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Selects the datetime wire format requested via the
    /// `Accept-Datetime-Format` header. Defaults to
    /// [`AcceptDatetimeFormat::Rfc3339`].
    pub fn datetime_format(mut self, format: AcceptDatetimeFormat) -> Self {
        self.datetime_format = format;
        self
    }

    /// Supplies a pre-configured [`reqwest::Client`] (proxies, timeouts,
    /// custom TLS). When set, [`ClientBuilder::user_agent`] is ignored.
    pub fn http_client(mut self, http: reqwest::Client) -> Self {
        self.http = Some(http);
        self
    }

    /// Overrides the `User-Agent` header of the default HTTP client.
    /// Defaults to `oanda-rs/<version>`.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Sets the client-side REST rate limit in requests per second.
    /// Defaults to 100 (OANDA rejects above 120/s per IP).
    ///
    /// Note that OANDA's limits apply per IP address: if several processes
    /// or [`Client`] instances share one IP, their combined rate matters.
    /// Within one process, share a single `Client` (it is cheap to clone).
    pub fn rest_rate_limit(mut self, requests_per_second: u32) -> Self {
        self.rest_rate_limit = requests_per_second;
        self
    }

    /// Enables or disables built-in rate limiting entirely (REST requests
    /// and stream connections). Enabled by default; disable only when you
    /// provide your own throttling.
    pub fn rate_limiting(mut self, enabled: bool) -> Self {
        self.rate_limiting = enabled;
        self
    }

    /// Builds the [`Client`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] when no token was provided, the token is
    /// empty, or the HTTP client cannot be constructed.
    pub fn build(self) -> Result<Client, Error> {
        let token = match self.token {
            Some(t) if !t.trim().is_empty() => t,
            _ => return Err(Error::Config("a non-empty API token is required".into())),
        };
        if self.rate_limiting && self.rest_rate_limit == 0 {
            return Err(Error::Config(
                "rest_rate_limit must be at least 1 request per second".into(),
            ));
        }
        let http = match self.http {
            Some(http) => http,
            None => reqwest::Client::builder()
                .user_agent(&self.user_agent)
                .build()
                .map_err(|e| Error::Config(format!("failed to build HTTP client: {e}")))?,
        };
        let (rest_limiter, conn_limiter) = if self.rate_limiting {
            (
                Some(RateLimiter::per_second(self.rest_rate_limit)),
                Some(RateLimiter::per_second(CONNECTIONS_PER_SECOND)),
            )
        } else {
            (None, None)
        };
        Ok(Client {
            inner: Arc::new(Inner {
                http,
                rest_base: self.environment.rest_base(),
                stream_base: self.environment.stream_base(),
                token,
                datetime_format: self.datetime_format,
                rest_limiter,
                conn_limiter,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_is_send_sync_and_clone() {
        fn assert_traits<T: Send + Sync + Clone + 'static>() {}
        assert_traits::<Client>();
    }

    #[test]
    fn debug_redacts_token() {
        let client = Client::new(Environment::Practice, "super-secret");
        let debug = format!("{client:?}");
        assert!(!debug.contains("super-secret"));
        assert!(debug.contains("<redacted>"));
    }

    #[test]
    fn builder_requires_token() {
        assert!(matches!(Client::builder().build(), Err(Error::Config(_))));
        assert!(matches!(
            Client::builder().token("  ").build(),
            Err(Error::Config(_))
        ));
    }

    #[test]
    fn environment_hosts() {
        assert_eq!(
            Environment::Practice.rest_base().as_str(),
            "https://api-fxpractice.oanda.com/v3"
        );
        assert_eq!(
            Environment::Live.stream_base().as_str(),
            "https://stream-fxtrade.oanda.com/v3"
        );
    }

    #[test]
    fn builder_full_configuration() {
        let client = Client::builder()
            .environment(Environment::Live)
            .token("t")
            .datetime_format(crate::models::AcceptDatetimeFormat::Unix)
            .user_agent("custom-agent/1.0")
            .rest_rate_limit(10)
            .build()
            .unwrap();
        assert_eq!(
            client.datetime_format(),
            crate::models::AcceptDatetimeFormat::Unix
        );
        assert_eq!(
            client.inner.rest_base.as_str(),
            "https://api-fxtrade.oanda.com/v3"
        );
        assert!(client.inner.rest_limiter.is_some());
    }

    #[test]
    fn builder_accepts_custom_http_client_and_disables_limits() {
        let http = reqwest::Client::builder().build().unwrap();
        let client = Client::builder()
            .token("t")
            .http_client(http)
            .rate_limiting(false)
            .build()
            .unwrap();
        assert!(client.inner.rest_limiter.is_none());
        assert!(client.inner.conn_limiter.is_none());
    }

    #[test]
    fn builder_rejects_zero_rate_limit() {
        assert!(matches!(
            Client::builder().token("t").rest_rate_limit(0).build(),
            Err(Error::Config(_))
        ));
        // ...unless rate limiting is disabled entirely.
        assert!(
            Client::builder()
                .token("t")
                .rest_rate_limit(0)
                .rate_limiting(false)
                .build()
                .is_ok()
        );
    }

    #[test]
    fn custom_environment_uses_given_urls() {
        let env = Environment::Custom {
            rest: "http://127.0.0.1:1/api".parse().unwrap(),
            stream: "http://127.0.0.1:2/stream".parse().unwrap(),
        };
        assert_eq!(env.rest_base().as_str(), "http://127.0.0.1:1/api");
        assert_eq!(env.stream_base().as_str(), "http://127.0.0.1:2/stream");
        assert!(format!("{env:?}").contains("Custom"));
    }
}
