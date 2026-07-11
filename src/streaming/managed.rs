//! The self-managing stream engine: connection state machine with
//! heartbeat watchdog, capped exponential backoff, and back-fill support.

use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use futures_core::Stream;
use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;
use serde::de::DeserializeOwned;
use tokio::time::{Instant, Sleep};

use super::StreamConfig;
use super::json_lines::JsonLines;
use crate::error::Error;

pub(crate) type ByteStream = BoxStream<'static, reqwest::Result<Bytes>>;
type Lines<T> = JsonLines<ByteStream, T>;
type ConnectFuture = BoxFuture<'static, Result<ByteStream, Error>>;
type BackfillFuture<T> = BoxFuture<'static, Result<Vec<T>, Error>>;
/// `Some(event)` ends the current poll iteration; `None` continues the state machine.
type Terminal<T> = Option<Poll<Option<Result<T, Error>>>>;

/// Endpoint-specific behaviour plugged into [`ManagedStream`].
pub(crate) trait StreamKind: Send + Unpin + 'static {
    type Item: DeserializeOwned + Send + Unpin + 'static;

    /// Builds a future that opens the connection (waiting for a
    /// connection-limiter slot, sending the request, and checking the
    /// response status).
    fn connect(&mut self, reconnect: bool) -> ConnectFuture;

    /// Called for every item before it is yielded; returning `false` drops
    /// the item (used to deduplicate back-filled transactions).
    fn filter(&mut self, _item: &Self::Item) -> bool {
        true
    }

    /// Builds an optional back-fill future run after a successful
    /// reconnect, yielding items missed while disconnected.
    fn backfill(&mut self) -> Option<BackfillFuture<Self::Item>> {
        None
    }
}

/// A snapshot of a managed stream's connection statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct StreamStats {
    /// Number of successful reconnects performed since the stream was
    /// created (the initial connection is not counted).
    pub reconnects: u64,
    /// Number of failed connection attempts since the stream was created.
    pub failed_attempts: u64,
}

enum State<T> {
    Connecting(ConnectFuture),
    Backfilling {
        lines: Lines<T>,
        future: BackfillFuture<T>,
    },
    Draining {
        lines: Lines<T>,
        pending: VecDeque<T>,
    },
    Streaming {
        lines: Lines<T>,
        watchdog: Pin<Box<Sleep>>,
    },
    Sleeping(Pin<Box<Sleep>>),
    Done,
}

pub(crate) struct ManagedStream<K: StreamKind> {
    kind: K,
    config: StreamConfig,
    state: State<K::Item>,
    stats: StreamStats,
    attempts_since_success: u32,
    current_delay: Duration,
    connected_at: Option<Instant>,
}

impl<K: StreamKind> ManagedStream<K> {
    /// Wraps an already-established connection (the initial connect is
    /// performed by the endpoint builder so connection errors surface at
    /// `send()`).
    pub(crate) fn new(kind: K, config: StreamConfig, initial: ByteStream) -> Self {
        let heartbeat_timeout = config.heartbeat_timeout;
        ManagedStream {
            kind,
            current_delay: config.backoff_initial,
            config,
            state: State::Streaming {
                lines: JsonLines::new(initial),
                watchdog: Box::pin(tokio::time::sleep(heartbeat_timeout)),
            },
            stats: StreamStats::default(),
            attempts_since_success: 0,
            connected_at: Some(Instant::now()),
        }
    }

    pub(crate) fn stats(&self) -> StreamStats {
        self.stats
    }

    /// Handles a broken connection: either schedules a reconnect (returning
    /// `None`) or produces the caller-visible terminal event.
    fn connection_lost(&mut self, error: Option<Error>) -> Terminal<K::Item> {
        #[cfg(feature = "tracing")]
        tracing::debug!(error = ?error, "stream connection lost");

        if !self.config.auto_reconnect {
            self.state = State::Done;
            return Some(Poll::Ready(error.map(Err)));
        }
        // A connection that stayed healthy long enough resets the backoff;
        // one that died right after connecting keeps escalating it.
        if let Some(connected_at) = self.connected_at.take() {
            if connected_at.elapsed() >= self.config.backoff_reset_after {
                self.attempts_since_success = 0;
                self.current_delay = self.config.backoff_initial;
            }
        }
        self.schedule_reconnect(error)
    }

    /// Handles a failed reconnect attempt.
    fn connect_failed(&mut self, error: Error) -> Terminal<K::Item> {
        self.stats.failed_attempts += 1;

        #[cfg(feature = "tracing")]
        tracing::debug!(error = %error, "stream reconnect attempt failed");

        if is_fatal(&error) {
            self.state = State::Done;
            return Some(Poll::Ready(Some(Err(error))));
        }
        self.schedule_reconnect(Some(error))
    }

    fn schedule_reconnect(&mut self, error: Option<Error>) -> Terminal<K::Item> {
        if let Some(max) = self.config.max_reconnect_attempts {
            if self.attempts_since_success >= max {
                self.state = State::Done;
                return Some(Poll::Ready(Some(Err(error.unwrap_or_else(|| {
                    Error::Stream("reconnect attempts exhausted".into())
                })))));
            }
        }
        self.attempts_since_success += 1;
        let delay = jitter(self.current_delay);
        self.current_delay = (self.current_delay * 2).min(self.config.backoff_max);

        #[cfg(feature = "tracing")]
        tracing::debug!(delay = ?delay, attempt = self.attempts_since_success, "stream reconnect scheduled");

        self.state = State::Sleeping(Box::pin(tokio::time::sleep(delay)));
        None
    }
}

/// Only client-side errors are fatal; transport failures and server errors
/// are worth retrying.
fn is_fatal(error: &Error) -> bool {
    match error {
        Error::Api { status, .. } => status.is_client_error(),
        Error::Config(_) => true,
        _ => false,
    }
}

/// Applies ±25% pseudo-random jitter so reconnecting clients don't
/// synchronize.
fn jitter(delay: Duration) -> Duration {
    let nanos = Instant::now().elapsed().subsec_nanos() as u64 ^ delay.as_nanos() as u64;
    let factor = 0.75 + (nanos % 1000) as f64 / 2000.0; // 0.75..=1.25
    delay.mul_f64(factor)
}

impl<K: StreamKind> Stream for ManagedStream<K> {
    type Item = Result<K::Item, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match &mut this.state {
                State::Connecting(future) => match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(bytes)) => {
                        this.stats.reconnects += 1;
                        this.connected_at = Some(Instant::now());
                        let lines = JsonLines::new(bytes);

                        #[cfg(feature = "tracing")]
                        tracing::debug!(reconnects = this.stats.reconnects, "stream reconnected");

                        this.state = match this.kind.backfill() {
                            Some(future) => State::Backfilling { lines, future },
                            None => State::Streaming {
                                lines,
                                watchdog: Box::pin(tokio::time::sleep(
                                    this.config.heartbeat_timeout,
                                )),
                            },
                        };
                    }
                    Poll::Ready(Err(e)) => {
                        if let Some(result) = this.connect_failed(e) {
                            return result;
                        }
                    }
                    Poll::Pending => return Poll::Pending,
                },
                State::Backfilling { future, .. } => match future.as_mut().poll(cx) {
                    Poll::Ready(result) => {
                        let State::Backfilling { lines, .. } =
                            std::mem::replace(&mut this.state, State::Done)
                        else {
                            unreachable!()
                        };
                        match result {
                            Ok(items) => {
                                this.state = State::Draining {
                                    lines,
                                    pending: items.into(),
                                };
                            }
                            Err(e) => {
                                // Surface the failed back-fill (there may be
                                // a gap), but keep the live stream running.
                                this.state = State::Streaming {
                                    lines,
                                    watchdog: Box::pin(tokio::time::sleep(
                                        this.config.heartbeat_timeout,
                                    )),
                                };
                                return Poll::Ready(Some(Err(e)));
                            }
                        }
                    }
                    Poll::Pending => return Poll::Pending,
                },
                State::Draining { pending, .. } => match pending.pop_front() {
                    Some(item) => {
                        if this.kind.filter(&item) {
                            return Poll::Ready(Some(Ok(item)));
                        }
                    }
                    None => {
                        let State::Draining { lines, .. } =
                            std::mem::replace(&mut this.state, State::Done)
                        else {
                            unreachable!()
                        };
                        this.state = State::Streaming {
                            lines,
                            watchdog: Box::pin(tokio::time::sleep(this.config.heartbeat_timeout)),
                        };
                    }
                },
                State::Streaming { lines, watchdog } => {
                    match Pin::new(lines).poll_next(cx) {
                        Poll::Ready(Some(Ok(item))) => {
                            watchdog
                                .as_mut()
                                .reset(Instant::now() + this.config.heartbeat_timeout);
                            if this.kind.filter(&item) {
                                return Poll::Ready(Some(Ok(item)));
                            }
                        }
                        Poll::Ready(Some(Err(e @ Error::Decode { .. }))) => {
                            // A malformed line doesn't invalidate the
                            // connection; report it and keep streaming.
                            return Poll::Ready(Some(Err(e)));
                        }
                        Poll::Ready(Some(Err(e))) => {
                            if let Some(result) = this.connection_lost(Some(e)) {
                                return result;
                            }
                        }
                        Poll::Ready(None) => {
                            if let Some(result) = this.connection_lost(None) {
                                return result;
                            }
                        }
                        Poll::Pending => match watchdog.as_mut().poll(cx) {
                            Poll::Ready(()) => {
                                let stale = Error::Stream(format!(
                                    "no data within {:?} (heartbeats expected every 5s); connection considered stale",
                                    this.config.heartbeat_timeout
                                ));
                                if let Some(result) = this.connection_lost(Some(stale)) {
                                    return result;
                                }
                            }
                            Poll::Pending => return Poll::Pending,
                        },
                    }
                }
                State::Sleeping(sleep) => match sleep.as_mut().poll(cx) {
                    Poll::Ready(()) => {
                        this.state = State::Connecting(this.kind.connect(true));
                    }
                    Poll::Pending => return Poll::Pending,
                },
                State::Done => return Poll::Ready(None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::StreamExt;
    use std::sync::{Arc, Mutex};

    /// Scripted connection outcomes for driving the state machine.
    enum Outcome {
        /// Connection refused (non-fatal).
        Fail,
        /// Connection refused with a fatal (4xx) error.
        FailFatal,
        /// Connects; yields the chunks, then EOF.
        Chunks(Vec<&'static [u8]>),
        /// Connects; yields the chunks, then hangs forever.
        ChunksThenHang(Vec<&'static [u8]>),
        /// Connects; yields one heartbeat line every `gap` for `count`
        /// lines, then EOF.
        Spaced { count: u32, gap: Duration },
    }

    struct MockKind {
        script: std::collections::VecDeque<Outcome>,
        connects: Arc<Mutex<Vec<Instant>>>,
        backfills: std::collections::VecDeque<Result<Vec<serde_json::Value>, ()>>,
        drop_below: Option<i64>,
    }

    impl MockKind {
        fn new(script: Vec<Outcome>) -> (Self, Arc<Mutex<Vec<Instant>>>) {
            let connects = Arc::new(Mutex::new(Vec::new()));
            (
                MockKind {
                    script: script.into(),
                    connects: Arc::clone(&connects),
                    backfills: Default::default(),
                    drop_below: None,
                },
                connects,
            )
        }
    }

    fn bytes_from(outcome: Outcome) -> Result<ByteStream, Error> {
        use futures_util::stream;
        match outcome {
            Outcome::Fail => Err(Error::Stream("connection refused".into())),
            Outcome::FailFatal => Err(Error::Api {
                status: reqwest::StatusCode::UNAUTHORIZED,
                request_id: None,
                body: crate::error::ApiErrorBody::from_text("nope".into()),
            }),
            Outcome::Chunks(chunks) => {
                Ok(stream::iter(chunks.into_iter().map(|c| Ok(Bytes::from_static(c)))).boxed())
            }
            Outcome::ChunksThenHang(chunks) => Ok(stream::iter(
                chunks.into_iter().map(|c| Ok(Bytes::from_static(c))),
            )
            .chain(stream::pending())
            .boxed()),
            Outcome::Spaced { count, gap } => Ok(stream::unfold(0u32, move |i| async move {
                if i >= count {
                    return None;
                }
                tokio::time::sleep(gap).await;
                Some((Ok(Bytes::from_static(b"{\"type\":\"HEARTBEAT\"}\n")), i + 1))
            })
            .boxed()),
        }
    }

    impl StreamKind for MockKind {
        type Item = serde_json::Value;

        fn connect(&mut self, _reconnect: bool) -> ConnectFuture {
            self.connects.lock().unwrap().push(Instant::now());
            let outcome = self.script.pop_front().expect("script exhausted");
            Box::pin(async move { bytes_from(outcome) })
        }

        fn filter(&mut self, item: &serde_json::Value) -> bool {
            match (self.drop_below, item.get("n").and_then(|n| n.as_i64())) {
                (Some(min), Some(n)) => n >= min,
                _ => true,
            }
        }

        fn backfill(&mut self) -> Option<BackfillFuture<serde_json::Value>> {
            let outcome = self.backfills.pop_front()?;
            Some(Box::pin(async move {
                outcome.map_err(|()| Error::Stream("backfill failed".into()))
            }))
        }
    }

    fn config() -> StreamConfig {
        StreamConfig::default()
    }

    fn managed(
        script: Vec<Outcome>,
        initial: Outcome,
        config: StreamConfig,
    ) -> (ManagedStream<MockKind>, Arc<Mutex<Vec<Instant>>>) {
        let (kind, connects) = MockKind::new(script);
        let initial = bytes_from(initial).unwrap();
        (ManagedStream::new(kind, config, initial), connects)
    }

    #[tokio::test(start_paused = true)]
    async fn backoff_escalates_with_cap_during_outage() {
        // Initial connection dies immediately; every reconnect fails. With
        // max 10 attempts the stream must end with an error, and the gaps
        // between attempts must escalate 1s→2s→…→300s cap (±25% jitter).
        let mut cfg = config();
        cfg.max_reconnect_attempts = Some(10);
        let script = (0..10).map(|_| Outcome::Fail).collect();
        let (stream, connects) = managed(script, Outcome::Chunks(vec![]), cfg);
        let start = Instant::now();
        let items: Vec<_> = stream.collect().await;
        assert_eq!(items.len(), 1);
        assert!(items[0].is_err(), "expected terminal error");

        let connects = connects.lock().unwrap();
        assert_eq!(connects.len(), 10);
        let mut previous = start;
        for (i, at) in connects.iter().enumerate() {
            let expected = Duration::from_secs(1 << i).min(Duration::from_secs(300));
            let delta = at.duration_since(previous);
            assert!(
                delta >= expected.mul_f64(0.74) && delta <= expected.mul_f64(1.26),
                "attempt {i}: delta {delta:?}, expected ~{expected:?}"
            );
            previous = *at;
        }
        // A multi-hour outage keeps the cadence at the 5-minute cap: the
        // last gap must be ~300s, not still growing.
        let last = connects[9].duration_since(connects[8]);
        assert!(last >= Duration::from_secs(225) && last <= Duration::from_secs(375));
    }

    #[tokio::test(start_paused = true)]
    async fn stable_connection_resets_backoff() {
        // fail, fail, then a connection healthy for >60s, then fail once
        // more: the delay after the healthy connection must be back at
        // ~1s, not continuing to escalate.
        let script = vec![
            Outcome::Fail,
            Outcome::Fail,
            Outcome::Spaced {
                count: 13,
                gap: Duration::from_secs(5),
            }, // healthy ~65s
            Outcome::Fail,
            Outcome::Chunks(vec![b"{\"ok\":1}\n"]),
        ];
        let mut cfg = config();
        cfg.max_reconnect_attempts = Some(100);
        let (stream, connects) = managed(script, Outcome::Chunks(vec![]), cfg);
        // 13 heartbeats + 1 final item; stream keeps reconnecting after
        // the last EOF, so just take what we expect.
        let items: Vec<_> = stream.take(14).collect().await;
        assert_eq!(items.iter().filter(|r| r.is_ok()).count(), 14);

        let connects = connects.lock().unwrap();
        // connect #2 (index 2, healthy) ends ~65s after it starts; connect
        // #3 (index 3) happens j(1s) later because the backoff reset.
        let healthy_end = connects[2] + Duration::from_secs(65);
        let delta = connects[3].duration_since(healthy_end);
        assert!(
            delta <= Duration::from_millis(1300),
            "backoff was not reset after stable connection: {delta:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn short_lived_connection_keeps_escalating() {
        // fail, then a connection that dies instantly, then fail: the
        // delay after the instant death must continue the escalation
        // (~4s), not reset to 1s.
        let script = vec![
            Outcome::Fail,
            Outcome::Chunks(vec![]), // connects, dies immediately
            Outcome::Fail,
            Outcome::Chunks(vec![b"{\"ok\":1}\n"]),
        ];
        let (stream, connects) = managed(script, Outcome::Chunks(vec![]), config());
        let items: Vec<_> = stream.take(1).collect().await;
        assert!(items[0].is_ok());

        let connects = connects.lock().unwrap();
        // delays: j(1) before #0, j(2) before #1, j(4) before #2? No —
        // successful connect #1 dies instantly (unstable), so the delay
        // before #2 continues at j(4).
        let delta = connects[2].duration_since(connects[1]);
        assert!(
            delta >= Duration::from_millis(2900),
            "backoff reset after an unstable connection: {delta:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn watchdog_detects_stale_connection() {
        // The initial connection sends one line then hangs. The watchdog
        // (10s) must declare it stale and reconnect.
        let script = vec![Outcome::Chunks(vec![b"{\"n\":2}\n"])];
        let (stream, connects) = managed(
            script,
            Outcome::ChunksThenHang(vec![b"{\"n\":1}\n"]),
            config(),
        );
        let start = Instant::now();
        let items: Vec<_> = stream.take(2).collect().await;
        assert_eq!(items.iter().filter(|r| r.is_ok()).count(), 2);

        let connects = connects.lock().unwrap();
        let delta = connects[0].duration_since(start);
        // ~10s watchdog + ~1s backoff (with jitter).
        assert!(
            delta >= Duration::from_millis(10_700) && delta <= Duration::from_millis(11_300),
            "unexpected stale detection timing: {delta:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn fatal_error_ends_stream() {
        let script = vec![Outcome::FailFatal];
        let (stream, _) = managed(script, Outcome::Chunks(vec![]), config());
        let items: Vec<_> = stream.collect().await;
        assert_eq!(items.len(), 1);
        match &items[0] {
            Err(Error::Api { status, .. }) => assert_eq!(status.as_u16(), 401),
            other => panic!("expected fatal Api error, got {other:?}"),
        }
    }

    #[tokio::test(start_paused = true)]
    async fn auto_reconnect_disabled_ends_on_eof() {
        let mut cfg = config();
        cfg.auto_reconnect = false;
        let (stream, connects) = managed(vec![], Outcome::Chunks(vec![b"{\"n\":1}\n"]), cfg);
        let items: Vec<_> = stream.collect().await;
        assert_eq!(items.len(), 1);
        assert!(items[0].is_ok());
        assert!(connects.lock().unwrap().is_empty(), "must not reconnect");
    }

    #[tokio::test(start_paused = true)]
    async fn stats_count_reconnects() {
        let script = vec![Outcome::Fail, Outcome::Chunks(vec![b"{\"n\":1}\n"])];
        let mut cfg = config();
        cfg.auto_reconnect = true;
        let (mut stream, _) = managed(script, Outcome::Chunks(vec![]), cfg);
        let first = stream.next().await.unwrap();
        assert!(first.is_ok());
        let stats = stream.stats();
        assert_eq!(stats.reconnects, 1);
        assert_eq!(stats.failed_attempts, 1);
    }

    #[tokio::test(start_paused = true)]
    async fn backfill_items_are_drained_and_filtered_before_live_data() {
        let script = vec![Outcome::Chunks(vec![b"{\"n\":4}\n"])];
        let (mut kind, _) = MockKind::new(script);
        kind.backfills.push_back(Ok(vec![
            serde_json::json!({"n": 1}), // dropped by filter
            serde_json::json!({"n": 3}),
        ]));
        kind.drop_below = Some(2);
        let initial = bytes_from(Outcome::Chunks(vec![])).unwrap();
        let stream = ManagedStream::new(kind, StreamConfig::default(), initial);
        let items: Vec<i64> = stream
            .take(2)
            .map(|r| r.unwrap()["n"].as_i64().unwrap())
            .collect()
            .await;
        assert_eq!(items, vec![3, 4]);
    }

    #[tokio::test(start_paused = true)]
    async fn failed_backfill_is_reported_but_stream_continues() {
        let script = vec![Outcome::Chunks(vec![b"{\"n\":7}\n"])];
        let (mut kind, _) = MockKind::new(script);
        kind.backfills.push_back(Err(()));
        let initial = bytes_from(Outcome::Chunks(vec![])).unwrap();
        let stream = ManagedStream::new(kind, StreamConfig::default(), initial);
        let items: Vec<_> = stream.take(2).collect().await;
        assert!(
            matches!(&items[0], Err(Error::Stream(msg)) if msg.contains("backfill failed")),
            "first item should be the backfill error: {items:?}"
        );
        assert_eq!(items[1].as_ref().unwrap()["n"], 7);
    }
}
