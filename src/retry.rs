//! Retry policy shared by streams and plain requests.
//!
//! OANDA is a 24/5 venue behind a CDN: outages, weekend maintenance and rate
//! limiting are routine rather than exceptional. Rather than have every caller
//! invent its own status matching and backoff loop, the SDK exposes one
//! policy — [`RetryPolicy`] — driven by one classification,
//! [`Error::is_transient`].
//!
//! ```no_run
//! # async fn run() -> Result<(), oanda_rs::Error> {
//! # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
//! use oanda_rs::{RetryPolicy, retry};
//!
//! let policy = RetryPolicy::default();
//! let summary = retry(&policy, || client.account_summary("101-004-1234567-001")).await?;
//! # let _ = summary;
//! # Ok(())
//! # }
//! ```

use std::future::Future;
use std::time::Duration;

use tokio::time::Instant;

use crate::error::Error;

/// What to do with errors [`Error::is_fatal`] says cannot succeed on a retry.
///
/// A status code alone cannot always separate "this credential is dead" from
/// "OANDA is doing maintenance and is briefly answering 4xx". Choosing between
/// them is an operational decision, so it is a policy knob rather than a fixed
/// rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum FatalRetry {
    /// Surface a fatal error immediately (the default).
    ///
    /// Right for interactive tools and batch jobs, where a bad token or an
    /// unknown instrument should stop the program rather than stall it.
    #[default]
    FailFast,
    /// Retry fatal errors too, but only for this long after the first one —
    /// then give up and surface it.
    ///
    /// Right for long-lived workers: it rides out a maintenance window that
    /// answers 4xx without hiding a genuinely revoked credential forever. The
    /// budget is measured from the first fatal error of a run and is cleared
    /// by any success, so an unrelated failure later gets a full budget again.
    ///
    /// Every retried fatal error is logged at `WARN` (with the `tracing`
    /// feature enabled) because such an error is otherwise invisible.
    Budget(Duration),
}

/// Capped exponential backoff with jitter, plus the fatal-error rule.
///
/// The defaults match the streaming defaults: start at 1s, double to a 5
/// minute cap, treat a connection healthy for 60s as a fresh start, retry
/// transient failures indefinitely, and fail fast on fatal ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Delay before the first retry.
    pub initial: Duration,
    /// Ceiling the doubling delay is clamped to.
    pub max: Duration,
    /// How long an attempt must succeed for before the delay resets to
    /// [`initial`](RetryPolicy::initial). Used by streams, whose connections
    /// are long-lived; irrelevant to one-shot requests.
    pub reset_after: Duration,
    /// Consecutive transient failures allowed before giving up. `None` rides
    /// out arbitrarily long outages.
    pub max_attempts: Option<u32>,
    /// How fatal errors are treated.
    pub fatal: FatalRetry,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            initial: Duration::from_secs(1),
            max: Duration::from_secs(300),
            reset_after: Duration::from_secs(60),
            max_attempts: None,
            fatal: FatalRetry::FailFast,
        }
    }
}

impl RetryPolicy {
    /// Sets the backoff bounds: the delay starts at `initial` and doubles
    /// (with jitter) up to `max`.
    pub fn backoff(mut self, initial: Duration, max: Duration) -> Self {
        self.initial = initial;
        self.max = max;
        self
    }

    /// Sets how long a successful attempt must last before the delay resets.
    pub fn reset_after(mut self, stable: Duration) -> Self {
        self.reset_after = stable;
        self
    }

    /// Limits consecutive transient failures. Pass `None` for unlimited.
    pub fn max_attempts(mut self, attempts: impl Into<Option<u32>>) -> Self {
        self.max_attempts = attempts.into();
        self
    }

    /// Sets how fatal errors are treated. See [`FatalRetry`].
    pub fn fatal(mut self, fatal: FatalRetry) -> Self {
        self.fatal = fatal;
        self
    }
}

/// Tracks one retry sequence: the escalating delay and the fatal-error budget.
///
/// Shared by [`retry`] and the streaming state machine so both obey the same
/// rules; the caller drives it, which keeps the sleeping (and therefore
/// cancellation) in the caller's hands.
#[derive(Debug)]
pub(crate) struct Backoff {
    policy: RetryPolicy,
    delay: Duration,
    failures: u32,
    fatal_retries: u64,
    first_fatal_at: Option<Instant>,
    rng: Jitter,
}

/// What a [`Backoff`] decided to do with a failure.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Decision {
    /// Wait this long, then try again.
    Retry(Duration),
    /// Stop: the error is fatal (and unretryable under the policy), or the
    /// budget or attempt limit is spent.
    GiveUp,
}

impl Backoff {
    pub(crate) fn new(policy: RetryPolicy) -> Self {
        Backoff {
            delay: policy.initial,
            policy,
            failures: 0,
            fatal_retries: 0,
            first_fatal_at: None,
            rng: Jitter::new(),
        }
    }

    /// Resets the escalation after a successful attempt, and clears any
    /// fatal-error budget so a later, unrelated failure starts fresh.
    pub(crate) fn succeeded(&mut self) {
        self.delay = self.policy.initial;
        self.failures = 0;
        self.first_fatal_at = None;
    }

    /// Fatal errors retried under [`FatalRetry::Budget`] so far. Surfaced by
    /// streams as `StreamStats::fatal_retries`, since such a retry is
    /// otherwise invisible to the caller.
    pub(crate) fn fatal_retries(&self) -> u64 {
        self.fatal_retries
    }

    /// Classifies a failure and, when retrying, returns how long to wait.
    pub(crate) fn on_error(&mut self, error: &Error) -> Decision {
        self.on_failure(Some(error))
    }

    /// As [`on_error`](Backoff::on_error), but `None` means "the connection
    /// ended without an error" (a clean EOF), which is always transient.
    pub(crate) fn on_failure(&mut self, error: Option<&Error>) -> Decision {
        if let Some(error) = error
            && error.is_fatal()
            && !self.fatal_within_budget(error)
        {
            return Decision::GiveUp;
        }
        if let Some(max) = self.policy.max_attempts
            && self.failures >= max
        {
            return Decision::GiveUp;
        }
        self.failures += 1;
        let delay = self.rng.apply(self.delay);
        self.delay = (self.delay * 2).min(self.policy.max);
        Decision::Retry(delay)
    }

    /// Whether a fatal error may still be retried under [`FatalRetry`].
    /// Starts the budget clock on the first such error.
    fn fatal_within_budget(&mut self, _error: &Error) -> bool {
        let FatalRetry::Budget(budget) = self.policy.fatal else {
            return false;
        };
        let started = *self.first_fatal_at.get_or_insert_with(Instant::now);
        let elapsed = started.elapsed();
        let within = elapsed < budget;
        if within {
            self.fatal_retries += 1;
        }

        #[cfg(feature = "tracing")]
        if within {
            // Loud on purpose: a retried 4xx is otherwise invisible, and an
            // operator needs to see a dying credential long before the budget
            // runs out.
            tracing::warn!(
                error = %_error,
                status = ?_error.status().map(|s| s.as_u16()),
                elapsed = ?elapsed,
                budget = ?budget,
                "retrying a fatal error under FatalRetry::Budget"
            );
        }
        within
    }

    /// Fatal errors retried so far, for reporting.
    #[cfg(test)]
    pub(crate) fn failures(&self) -> u32 {
        self.failures
    }
}

/// Runs `op` until it succeeds or `policy` gives up.
///
/// Transient failures ([`Error::is_transient`]) are retried on the policy's
/// capped, jittered backoff; fatal ones follow [`FatalRetry`]. The last error
/// is returned when the policy gives up.
///
/// The future holds no cancellation of its own — wrap it in `tokio::select!`
/// (or drop it) to abort a long wait.
///
/// ```no_run
/// # async fn run() -> Result<(), oanda_rs::Error> {
/// # let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
/// use std::time::Duration;
/// use oanda_rs::{FatalRetry, RetryPolicy, retry};
///
/// // A long-lived worker: ride out maintenance windows that answer 4xx,
/// // but still surface a genuinely dead credential after six hours.
/// let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(6 * 3600)));
/// let accounts = retry(&policy, || client.list_accounts()).await?;
/// # let _ = accounts;
/// # Ok(())
/// # }
/// ```
pub async fn retry<T, F, Fut>(policy: &RetryPolicy, mut op: F) -> Result<T, Error>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, Error>>,
{
    let mut backoff = Backoff::new(*policy);
    loop {
        let error = match op().await {
            Ok(value) => return Ok(value),
            Err(error) => error,
        };
        match backoff.on_error(&error) {
            Decision::GiveUp => return Err(error),
            Decision::Retry(delay) => tokio::time::sleep(delay).await,
        }
    }
}

/// Decorrelating jitter for retry delays.
///
/// Spreads each delay uniformly across ±25% so that independent clients — a
/// fleet of workers reconnecting after the same outage — do not synchronise
/// into a thundering herd. Seeded per instance from [`RandomState`], whose
/// keys are randomised per process, so separate processes diverge too.
///
/// [`RandomState`]: std::collections::hash_map::RandomState
#[derive(Debug)]
struct Jitter {
    state: u64,
}

impl Jitter {
    fn new() -> Self {
        use std::hash::{BuildHasher, Hasher, RandomState};
        // A fresh RandomState carries process-random keys; hashing nothing
        // still yields a distinct value per instance.
        let seed = RandomState::new().build_hasher().finish();
        Jitter::with_seed(seed)
    }

    /// Deterministic construction, for tests that assert the distribution.
    fn with_seed(seed: u64) -> Self {
        // Any non-zero state; xorshift64 is stuck at zero.
        Jitter { state: seed | 1 }
    }

    /// xorshift64 — small, fast and adequate for spreading retries.
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Scales `delay` by a factor in `0.75..=1.25`.
    fn apply(&mut self, delay: Duration) -> Duration {
        // 0..=10_000 -> 0.75..=1.25
        let steps = self.next_u64() % 10_001;
        let factor = 0.75 + (steps as f64) / 20_000.0;
        delay.mul_f64(factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ApiErrorBody;
    use reqwest::StatusCode;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn api_error(status: u16) -> Error {
        Error::Api {
            status: StatusCode::from_u16(status).unwrap(),
            request_id: None,
            body: ApiErrorBody::from_text("boom".into()),
        }
    }

    /// Counts calls so tests can assert how many attempts actually ran.
    #[derive(Default)]
    struct Calls(AtomicU32);

    impl Calls {
        fn bump(&self) -> u32 {
            self.0.fetch_add(1, Ordering::SeqCst) + 1
        }
        fn count(&self) -> u32 {
            self.0.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn builder_sets_every_field() {
        let policy = RetryPolicy::default()
            .backoff(Duration::from_millis(50), Duration::from_secs(9))
            .reset_after(Duration::from_secs(120))
            .max_attempts(7)
            .fatal(FatalRetry::Budget(Duration::from_secs(600)));
        assert_eq!(policy.initial, Duration::from_millis(50));
        assert_eq!(policy.max, Duration::from_secs(9));
        assert_eq!(policy.reset_after, Duration::from_secs(120));
        assert_eq!(policy.max_attempts, Some(7));
        assert_eq!(policy.fatal, FatalRetry::Budget(Duration::from_secs(600)));

        // An attempt limit can be lifted again, which `Option<u32>` alone
        // would not allow through the builder.
        assert_eq!(policy.max_attempts(None).max_attempts, None);

        // The default is the conservative one.
        assert_eq!(RetryPolicy::default().fatal, FatalRetry::FailFast);
    }

    #[tokio::test(start_paused = true)]
    async fn the_configured_initial_delay_is_used() {
        let policy =
            RetryPolicy::default().backoff(Duration::from_secs(30), Duration::from_secs(60));
        let mut backoff = Backoff::new(policy);
        let Decision::Retry(delay) = backoff.on_error(&api_error(500)) else {
            panic!("expected a retry");
        };
        assert!(
            delay >= Duration::from_millis(22_500) && delay <= Duration::from_millis(37_500),
            "{delay:?} is not ~30s ±25%"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn succeeds_without_retrying() {
        let calls = Calls::default();
        let value = retry(&RetryPolicy::default(), || async {
            calls.bump();
            Ok::<_, Error>(7)
        })
        .await
        .unwrap();
        assert_eq!(value, 7);
        assert_eq!(calls.count(), 1, "a success must not be retried");
    }

    #[tokio::test(start_paused = true)]
    async fn retries_transient_until_success() {
        let calls = Calls::default();
        let value = retry(&RetryPolicy::default(), || async {
            // Two outages (a Cloudflare 520, then a rate limit), then through.
            match calls.bump() {
                1 => Err(api_error(520)),
                2 => Err(api_error(429)),
                _ => Ok(3),
            }
        })
        .await
        .unwrap();
        assert_eq!(value, 3);
        assert_eq!(calls.count(), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn fatal_fails_fast_without_sleeping() {
        let calls = Calls::default();
        let start = Instant::now();
        let error = retry(&RetryPolicy::default(), || async {
            calls.bump();
            Err::<(), _>(api_error(401))
        })
        .await
        .unwrap_err();

        assert_eq!(error.status().map(|s| s.as_u16()), Some(401));
        assert_eq!(calls.count(), 1, "a fatal error must not be retried");
        assert_eq!(start.elapsed(), Duration::ZERO, "and must not sleep first");
    }

    #[tokio::test(start_paused = true)]
    async fn gives_up_after_max_attempts_returning_the_last_error() {
        let calls = Calls::default();
        let policy = RetryPolicy::default().max_attempts(3);
        let error = retry(&policy, || async {
            // The final error is the one the caller should see.
            if calls.bump() < 4 {
                Err::<(), _>(api_error(503))
            } else {
                Err(api_error(502))
            }
        })
        .await
        .unwrap_err();

        // 3 retries after the first failure = 4 attempts.
        assert_eq!(calls.count(), 4);
        assert_eq!(error.status().map(|s| s.as_u16()), Some(502));
    }

    #[tokio::test(start_paused = true)]
    async fn backoff_escalates_and_holds_at_the_cap() {
        let policy = RetryPolicy::default().max_attempts(9);
        let mut backoff = Backoff::new(policy);
        let mut delays = Vec::new();
        for _ in 0..9 {
            match backoff.on_error(&api_error(500)) {
                Decision::Retry(delay) => delays.push(delay),
                Decision::GiveUp => break,
            }
        }
        assert_eq!(delays.len(), 9);
        for (i, delay) in delays.iter().enumerate() {
            let expected = Duration::from_secs(1 << i).min(Duration::from_secs(300));
            assert!(
                *delay >= expected.mul_f64(0.74) && *delay <= expected.mul_f64(1.26),
                "attempt {i}: {delay:?} outside ±25% of {expected:?}"
            );
        }
        // Exhausted: the tenth failure gives up.
        assert_eq!(backoff.on_error(&api_error(500)), Decision::GiveUp);
    }

    #[test]
    fn success_resets_the_escalation() {
        let mut backoff = Backoff::new(RetryPolicy::default());
        for _ in 0..4 {
            backoff.on_error(&api_error(500));
        }
        assert_eq!(backoff.failures(), 4);
        backoff.succeeded();
        assert_eq!(backoff.failures(), 0);
        let Decision::Retry(delay) = backoff.on_error(&api_error(500)) else {
            panic!("expected a retry");
        };
        // Back to ~1s rather than continuing at ~16s.
        assert!(delay <= Duration::from_millis(1_260), "{delay:?}");
    }

    // ---- FatalRetry::Budget -------------------------------------------

    #[tokio::test(start_paused = true)]
    async fn budget_retries_a_fatal_error_then_surfaces_it() {
        let calls = Calls::default();
        // Six hours of maintenance answering 403 — with a paused clock this
        // runs instantly.
        let policy =
            RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(6 * 3600)));
        let error = retry(&policy, || async {
            calls.bump();
            Err::<(), _>(api_error(403))
        })
        .await
        .unwrap_err();

        assert!(calls.count() > 1, "the fatal error must have been retried");
        assert_eq!(
            error.status().map(|s| s.as_u16()),
            Some(403),
            "and the original error must survive to the caller"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn budget_lets_a_recovering_endpoint_through() {
        let calls = Calls::default();
        let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(3600)));
        let value = retry(&policy, || async {
            // Maintenance answers 404 twice, then service resumes.
            if calls.bump() < 3 {
                Err(api_error(404))
            } else {
                Ok("back")
            }
        })
        .await
        .unwrap();
        assert_eq!(value, "back");
        assert_eq!(calls.count(), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn budget_is_measured_from_the_first_fatal_error() {
        let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(60)));
        let mut backoff = Backoff::new(policy);
        assert!(matches!(
            backoff.on_error(&api_error(401)),
            Decision::Retry(_)
        ));
        // Still inside the window.
        tokio::time::sleep(Duration::from_secs(30)).await;
        assert!(matches!(
            backoff.on_error(&api_error(401)),
            Decision::Retry(_)
        ));
        // Past it: the credential really is dead.
        tokio::time::sleep(Duration::from_secs(31)).await;
        assert_eq!(backoff.on_error(&api_error(401)), Decision::GiveUp);
    }

    #[tokio::test(start_paused = true)]
    async fn a_success_clears_the_fatal_budget() {
        let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(60)));
        let mut backoff = Backoff::new(policy);
        backoff.on_error(&api_error(401));
        tokio::time::sleep(Duration::from_secs(59)).await;
        // Recovery must hand back a full budget, not the 1s remnant.
        backoff.succeeded();
        assert!(matches!(
            backoff.on_error(&api_error(401)),
            Decision::Retry(_)
        ));
        tokio::time::sleep(Duration::from_secs(30)).await;
        assert!(
            matches!(backoff.on_error(&api_error(401)), Decision::Retry(_)),
            "the budget must have restarted at the later failure"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_zero_budget_behaves_like_fail_fast() {
        let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::ZERO));
        let mut backoff = Backoff::new(policy);
        assert_eq!(backoff.on_error(&api_error(401)), Decision::GiveUp);
    }

    #[tokio::test(start_paused = true)]
    async fn max_attempts_also_bounds_budgeted_fatal_retries() {
        // Whichever bound trips first wins: here the attempt cap, well inside
        // a generous budget.
        let policy = RetryPolicy::default()
            .max_attempts(2)
            .fatal(FatalRetry::Budget(Duration::from_secs(86_400)));
        let mut backoff = Backoff::new(policy);
        assert!(matches!(
            backoff.on_error(&api_error(403)),
            Decision::Retry(_)
        ));
        assert!(matches!(
            backoff.on_error(&api_error(403)),
            Decision::Retry(_)
        ));
        assert_eq!(backoff.on_error(&api_error(403)), Decision::GiveUp);
    }

    #[test]
    fn transient_errors_ignore_the_fatal_budget() {
        let policy = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::ZERO));
        let mut backoff = Backoff::new(policy);
        // A 503 is transient, so the spent fatal budget is irrelevant.
        assert!(matches!(
            backoff.on_error(&api_error(503)),
            Decision::Retry(_)
        ));
    }

    // ---- jitter --------------------------------------------------------

    /// The regression test for a jitter function that returned a constant
    /// 0.75× — which both shortened every delay by 25% and left a fleet of
    /// clients perfectly synchronised, the opposite of the intent.
    #[test]
    fn jitter_spreads_across_the_whole_band() {
        let mut jitter = Jitter::with_seed(0x5eed);
        let base = Duration::from_secs(4);
        let mut low = false;
        let mut high = false;
        let mut seen = Vec::new();
        for _ in 0..200 {
            let delay = jitter.apply(base);
            assert!(
                delay >= base.mul_f64(0.75) && delay <= base.mul_f64(1.25),
                "{delay:?} outside ±25% of {base:?}"
            );
            if delay < base.mul_f64(0.85) {
                low = true;
            }
            if delay > base.mul_f64(1.15) {
                high = true;
            }
            seen.push(delay);
        }
        assert!(low && high, "jitter must reach both ends of the band");
        seen.dedup();
        assert!(seen.len() > 100, "successive delays must differ");
    }

    #[test]
    fn jitter_instances_diverge() {
        // Two workers reconnecting after the same outage must not agree.
        let mut a = Jitter::new();
        let mut b = Jitter::new();
        let base = Duration::from_secs(60);
        let differs = (0..8).any(|_| a.apply(base) != b.apply(base));
        assert!(differs, "independent instances must decorrelate");
    }

    #[test]
    fn jitter_survives_a_zero_seed() {
        // xorshift64 is stuck at zero; the seed must be forced non-zero.
        let mut jitter = Jitter::with_seed(0);
        let base = Duration::from_secs(1);
        assert_ne!(jitter.apply(base), jitter.apply(base));
    }
}
