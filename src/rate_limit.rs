//! Internal token-bucket rate limiter shared by all clones of a client.

use std::sync::Mutex;
use std::time::Duration;

use tokio::time::Instant;

/// A small async token bucket.
///
/// Tokens refill continuously at `refill_per_sec`; the bucket never holds
/// more than `capacity` tokens. [`RateLimiter::acquire`] takes one token,
/// sleeping until one becomes available. Uses [`tokio::time::Instant`] so
/// tests with paused time behave deterministically.
#[derive(Debug)]
pub(crate) struct RateLimiter {
    capacity: f64,
    refill_per_sec: f64,
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    tokens: f64,
    last_refill: Instant,
}

impl RateLimiter {
    /// A limiter allowing `per_second` acquisitions per second with a burst
    /// capacity of the same size.
    pub(crate) fn per_second(per_second: u32) -> Self {
        let cap = f64::from(per_second.max(1));
        RateLimiter {
            capacity: cap,
            refill_per_sec: cap,
            state: Mutex::new(State {
                tokens: cap,
                last_refill: Instant::now(),
            }),
        }
    }

    /// Takes one token, waiting for the bucket to refill when empty.
    pub(crate) async fn acquire(&self) {
        loop {
            let wait = {
                let mut state = self.state.lock().expect("rate limiter poisoned");
                let now = Instant::now();
                let elapsed = now.duration_since(state.last_refill).as_secs_f64();
                state.tokens = (state.tokens + elapsed * self.refill_per_sec).min(self.capacity);
                state.last_refill = now;
                if state.tokens >= 1.0 {
                    state.tokens -= 1.0;
                    return;
                }
                // Time until one full token is available.
                Duration::from_secs_f64((1.0 - state.tokens) / self.refill_per_sec)
            };
            tokio::time::sleep(wait).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test(start_paused = true)]
    async fn burst_then_blocked() {
        let limiter = RateLimiter::per_second(2);
        let start = Instant::now();
        // Burst capacity: two immediate acquisitions.
        limiter.acquire().await;
        limiter.acquire().await;
        assert_eq!(start.elapsed(), Duration::ZERO);
        // Third must wait ~500ms for a refill at 2 tokens/sec.
        limiter.acquire().await;
        let waited = start.elapsed();
        assert!(
            waited >= Duration::from_millis(490) && waited <= Duration::from_millis(600),
            "waited {waited:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn refill_caps_at_capacity() {
        let limiter = RateLimiter::per_second(2);
        limiter.acquire().await;
        limiter.acquire().await;
        // A long idle period must not accumulate more than `capacity` tokens.
        tokio::time::sleep(Duration::from_secs(60)).await;
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        assert_eq!(start.elapsed(), Duration::ZERO);
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_millis(490));
    }

    #[tokio::test(start_paused = true)]
    async fn concurrent_tasks_share_bucket() {
        let limiter = Arc::new(RateLimiter::per_second(10));
        let done = Arc::new(AtomicU32::new(0));
        let mut handles = Vec::new();
        for _ in 0..30 {
            let l = Arc::clone(&limiter);
            let d = Arc::clone(&done);
            handles.push(tokio::spawn(async move {
                l.acquire().await;
                d.fetch_add(1, Ordering::SeqCst);
            }));
        }
        // 10 immediately, then 10/sec: all 30 need ~2s.
        tokio::time::sleep(Duration::from_millis(2100)).await;
        for h in handles {
            h.await.unwrap();
        }
        assert_eq!(done.load(Ordering::SeqCst), 30);
    }
}
