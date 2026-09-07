//! Self-managing streaming connections for the pricing and transaction
//! streams.
//!
//! The streams returned by
//! [`Client::pricing_stream`](crate::Client::pricing_stream) and
//! [`Client::transaction_stream`](crate::Client::transaction_stream) manage
//! their own connection state:
//!
//! - OANDA sends a heartbeat line every 5 seconds; if nothing arrives
//!   within the configured heartbeat timeout, the connection is considered
//!   stale and replaced.
//! - Reconnects use capped exponential backoff with jitter (default 1s
//!   doubling up to 5 minutes), so an extended outage — e.g. OANDA's
//!   weekend maintenance — is retried gently instead of flooding. The
//!   backoff only resets after a connection has stayed healthy for a
//!   stability period, and every attempt is gated by the client's shared
//!   2-connections-per-second limiter.
//! - The transaction stream tracks the last transaction ID it delivered
//!   and back-fills the gap via `GET .../transactions/sinceid` after every
//!   reconnect (no silent data loss); the pricing stream reconnects with
//!   `snapshot=true` so fresh prices arrive immediately.
//!
//! Heartbeats are yielded to the caller (useful as a liveness signal);
//! reconnection is otherwise invisible. Fatal errors (HTTP 4xx on
//! reconnect) end the stream with a final `Err` item.

mod json_lines;
mod managed;

use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_core::Stream;
use futures_util::StreamExt;
use reqwest::{Method, Url};

pub use managed::StreamStats;

use crate::client::Client;
use crate::error::Error;
use crate::models::transaction::TransactionStreamItem;
use crate::models::{AccountId, PriceStreamItem};
use crate::retry::{Backoff, Decision, RetryPolicy};
pub(crate) use managed::StreamKind;
use managed::{ByteStream, ManagedStream};

/// Connection policy of a managed stream.
///
/// Built through the setters on
/// [`pricing_stream`](crate::Client::pricing_stream) and
/// [`transaction_stream`](crate::Client::transaction_stream); exposed so the
/// same policy can be stored and shared across streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamConfig {
    /// Whether a broken connection is re-established automatically. This one
    /// switch governs the *initial* connect as well: with it enabled a stream
    /// opened during an outage waits for the venue to come back instead of
    /// failing, which is what a long-lived worker wants.
    pub auto_reconnect: bool,
    /// How long without any line (heartbeats arrive every 5s) before the
    /// connection is considered stale and replaced.
    pub heartbeat_timeout: Duration,
    /// Backoff, attempt limit and fatal-error handling, shared with
    /// [`retry`](crate::retry()).
    pub retry: RetryPolicy,
}

impl Default for StreamConfig {
    fn default() -> Self {
        StreamConfig {
            auto_reconnect: true,
            heartbeat_timeout: Duration::from_secs(10),
            retry: RetryPolicy::default(),
        }
    }
}

/// Opens a streaming connection: waits for a connection-limiter slot,
/// sends the request, and verifies the response status.
pub(crate) async fn open_stream(client: Client, url: Url) -> Result<ByteStream, Error> {
    client.acquire_connection_slot().await;
    let response = client.request(Method::GET, url).send().await?;
    if !response.status().is_success() {
        return Err(crate::transport::error_from_response(response).await);
    }
    Ok(response.bytes_stream().boxed())
}

/// Opens the first connection of a stream, applying the same retry policy the
/// stream will use once running.
///
/// Without this the policy only took effect *after* one successful connect, so
/// a worker started during an outage — OANDA's edge answers 5xx at weekends —
/// failed immediately, while the identical failure a minute later would have
/// been ridden out. A fatal rejection (bad token, unknown account) still
/// surfaces straight away, so `send()` keeps its fail-fast behaviour for the
/// errors that deserve it.
pub(crate) async fn connect_initial<K: StreamKind>(
    kind: &mut K,
    config: &StreamConfig,
) -> Result<ByteStream, Error> {
    if !config.auto_reconnect {
        return kind.connect(false).await;
    }
    let mut backoff = Backoff::new(config.retry);
    loop {
        let error = match kind.connect(false).await {
            Ok(stream) => return Ok(stream),
            Err(error) => error,
        };
        match backoff.on_error(&error) {
            Decision::GiveUp => return Err(error),
            Decision::Retry(delay) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(error = %error, delay = ?delay, "stream connect failed; retrying");
                tokio::time::sleep(delay).await;
            }
        }
    }
}

pub(crate) struct PricingKind {
    pub(crate) client: Client,
    pub(crate) account_id: AccountId,
    pub(crate) instruments: String,
    pub(crate) snapshot: Option<bool>,
}

impl PricingKind {
    fn url(&self, reconnect: bool) -> Url {
        let mut url =
            self.client
                .stream_url(&["accounts", self.account_id.as_str(), "pricing", "stream"]);
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("instruments", &self.instruments);
            // Reconnects always request a snapshot so consumers get fresh
            // prices immediately after a gap.
            let snapshot = if reconnect { Some(true) } else { self.snapshot };
            if let Some(snapshot) = snapshot {
                query.append_pair("snapshot", if snapshot { "true" } else { "false" });
            }
        }
        url
    }
}

impl StreamKind for PricingKind {
    type Item = PriceStreamItem;

    fn connect(
        &mut self,
        reconnect: bool,
    ) -> futures_core::future::BoxFuture<'static, Result<ByteStream, Error>> {
        let client = self.client.clone();
        let url = self.url(reconnect);
        Box::pin(open_stream(client, url))
    }
}

/// A self-managing pricing stream; see the [module docs](self) for the
/// reconnection behaviour. Yields [`PriceStreamItem`]s.
pub struct PricingStream {
    inner: ManagedStream<PricingKind>,
}

impl PricingStream {
    pub(crate) fn new(kind: PricingKind, config: StreamConfig, initial: ByteStream) -> Self {
        PricingStream {
            inner: ManagedStream::new(kind, config, initial),
        }
    }

    /// A snapshot of the stream's connection statistics.
    pub fn stats(&self) -> StreamStats {
        self.inner.stats()
    }
}

impl Stream for PricingStream {
    type Item = Result<PriceStreamItem, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl std::fmt::Debug for PricingStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PricingStream")
            .field("stats", &self.stats())
            .finish_non_exhaustive()
    }
}

pub(crate) struct TransactionKind {
    pub(crate) client: Client,
    pub(crate) account_id: AccountId,
    /// The stream's retry policy, applied to the reconnect back-fill as
    /// well. Without it the back-fill ran on the client's plain request
    /// path, so a 401 the stream itself was riding out under a
    /// [`FatalRetry::Budget`](crate::FatalRetry::Budget) failed the
    /// back-fill instantly and surfaced as a gap.
    pub(crate) retry: RetryPolicy,
    /// The numeric ID of the last transaction yielded, used for reconnect
    /// back-fill and deduplication.
    pub(crate) last_seen: Option<u64>,
}

impl StreamKind for TransactionKind {
    type Item = TransactionStreamItem;

    fn connect(
        &mut self,
        _reconnect: bool,
    ) -> futures_core::future::BoxFuture<'static, Result<ByteStream, Error>> {
        let client = self.client.clone();
        let url = self.client.stream_url(&[
            "accounts",
            self.account_id.as_str(),
            "transactions",
            "stream",
        ]);
        Box::pin(open_stream(client, url))
    }

    fn filter(&mut self, item: &TransactionStreamItem) -> bool {
        if let TransactionStreamItem::Transaction(tx) = item {
            if let Some(id) = tx.id().and_then(|id| id.as_str().parse::<u64>().ok()) {
                if self.last_seen.is_some_and(|seen| id <= seen) {
                    return false; // already delivered (backfill overlap)
                }
                self.last_seen = Some(id);
            }
        }
        true
    }

    fn backfill(
        &mut self,
    ) -> Option<futures_core::future::BoxFuture<'static, Result<Vec<TransactionStreamItem>, Error>>>
    {
        let last_seen = self.last_seen?;
        let client = self.client.clone();
        let account_id = self.account_id.clone();
        let policy = self.retry;
        Some(Box::pin(async move {
            let response = crate::retry::retry(&policy, || {
                client.transactions_since_id(account_id.clone(), last_seen.to_string())
            })
            .await?;
            Ok(response
                .transactions
                .into_iter()
                .map(TransactionStreamItem::Transaction)
                .collect())
        }))
    }
}

/// A self-managing transaction stream; see the [module docs](self) for the
/// reconnection and back-fill behaviour. Yields
/// [`TransactionStreamItem`]s.
pub struct TransactionStream {
    inner: ManagedStream<TransactionKind>,
}

impl TransactionStream {
    pub(crate) fn new(kind: TransactionKind, config: StreamConfig, initial: ByteStream) -> Self {
        TransactionStream {
            inner: ManagedStream::new(kind, config, initial),
        }
    }

    /// A snapshot of the stream's connection statistics.
    pub fn stats(&self) -> StreamStats {
        self.inner.stats()
    }
}

impl Stream for TransactionStream {
    type Item = Result<TransactionStreamItem, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl std::fmt::Debug for TransactionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransactionStream")
            .field("stats", &self.stats())
            .finish_non_exhaustive()
    }
}

/// Generates the shared reconnection-policy setters on stream request
/// builders.
macro_rules! stream_config_setters {
    () => {
        /// Enables or disables automatic reconnection (enabled by
        /// default). When disabled, the stream ends on the first
        /// connection problem *and* `send()` fails immediately rather than
        /// waiting out a transient rejection.
        pub fn auto_reconnect(mut self, enabled: bool) -> Self {
            self.config.auto_reconnect = enabled;
            self
        }

        /// Replaces the whole retry policy — backoff, attempt limit and
        /// fatal-error handling — in one go. The individual setters below
        /// are shorthands for parts of it.
        pub fn retry_policy(mut self, policy: $crate::RetryPolicy) -> Self {
            self.config.retry = policy;
            self
        }

        /// How errors that cannot succeed on a retry (a bad token, an
        /// unknown account) are treated. Defaults to
        /// [`FatalRetry::FailFast`](crate::FatalRetry::FailFast).
        ///
        /// Long-lived workers may prefer
        /// [`FatalRetry::Budget`](crate::FatalRetry::Budget), since OANDA
        /// has been observed answering 4xx during maintenance; see its docs
        /// for the trade-off.
        pub fn fatal_retry(mut self, fatal: $crate::FatalRetry) -> Self {
            self.config.retry.fatal = fatal;
            self
        }

        /// How long without any line (heartbeats arrive every 5s) before
        /// the connection is considered stale and replaced. Default 10s.
        pub fn heartbeat_timeout(mut self, timeout: std::time::Duration) -> Self {
            self.config.heartbeat_timeout = timeout;
            self
        }

        /// Sets the reconnect backoff: the delay starts at `initial` and
        /// doubles (with jitter) up to `max`. Defaults: 1s → 5 minutes.
        /// The 5-minute default cap keeps retry traffic negligible across
        /// extended outages such as OANDA's weekend maintenance windows.
        pub fn backoff(mut self, initial: std::time::Duration, max: std::time::Duration) -> Self {
            self.config.retry.initial = initial;
            self.config.retry.max = max;
            self
        }

        /// How long a connection must stay healthy before the backoff
        /// resets to its initial delay (default 60s). Prevents
        /// connect/die/connect churn from bypassing the cooldown.
        pub fn backoff_reset_after(mut self, stable: std::time::Duration) -> Self {
            self.config.retry.reset_after = stable;
            self
        }

        /// Limits the number of consecutive failed reconnect attempts
        /// before the stream gives up with an error. Default: unlimited
        /// (rides out arbitrarily long outages); pass `None` to restore it.
        pub fn max_reconnect_attempts(mut self, attempts: impl Into<Option<u32>>) -> Self {
            self.config.retry.max_attempts = attempts.into();
            self
        }
    };
}

pub(crate) use stream_config_setters;
