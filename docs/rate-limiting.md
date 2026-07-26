# Rate limiting

OANDA enforces per-IP limits:

| Limit | Value |
|---|---|
| REST requests | 120 / second (HTTP 429 beyond that) |
| New connections | 2 / second (rejected beyond that) |
| Active streams | 20 |

## What the SDK does

The client ships with a built-in token-bucket limiter, **enabled by default**:

- REST requests are limited to **100/second** (headroom under the 120/s cap).
- Stream connections — including every automatic reconnect — are limited to
  **2/second**.

Both buckets are shared by *all clones* of a `Client`, so a multi-task
application that clones one client stays within limits no matter how many tasks
fire requests concurrently. When the bucket is empty, requests simply wait
(`.await`) for the next token; no error is surfaced.

```rust,no_run
use oanda_rs::{Client, Environment};

let client = Client::builder()
    .environment(Environment::Practice)
    .token("t")
    .rest_rate_limit(50)     // stricter than the default 100/s
    .build()
    .unwrap();

// Or, if you throttle elsewhere:
let unlimited = Client::builder()
    .environment(Environment::Practice)
    .token("t")
    .rate_limiting(false)
    .build()
    .unwrap();
```

## Caveats

- OANDA's limits are **per IP address**. Two `Client` instances (or two
  processes) behind one IP each limit themselves independently — their *combined*
  rate can still trip the server. Within one process, always share a single
  client; it is cheap to clone.
- The limiter is proactive: it keeps you *under* the limits rather than reacting
  to them. A 429 can still arrive — another process may share your IP — and it
  is classified as **transient**, so streams back off and reconnect rather than
  ending. Use `Error::is_rate_limited` when you want to treat it specially.
- Transaction-stream back-fills after reconnects consume REST quota; they pass
  through the same limiter.

## Retrying ordinary requests

The limiter makes requests *wait*, but it cannot prevent a failure that has
already happened. For that, the SDK exposes the same retry policy its streams
use, so callers need not hand-roll status matching and backoff:

```rust,no_run
# async fn run() -> Result<(), oanda_rs::Error> {
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
use oanda_rs::{RetryPolicy, retry};

// Retries 429s, 5xx and transport failures on capped, jittered backoff;
// returns immediately on a bad token or a malformed request.
let summary = retry(&RetryPolicy::default(), || {
    client.account_summary("101-004-1234567-001")
})
.await?;
# let _ = summary;
# Ok(())
# }
```

See [`RetryPolicy`] for the backoff bounds and attempt limit, and `FatalRetry`
for riding out maintenance windows that answer 4xx.
