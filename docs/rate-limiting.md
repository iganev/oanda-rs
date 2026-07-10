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
- The limiter is proactive, not reactive: the SDK does not auto-retry HTTP 429.
  If you see one (e.g. another process shares your IP), detect it with
  [`Error::is_rate_limited`] and back off yourself.
- Transaction-stream back-fills after reconnects consume REST quota; they pass
  through the same limiter.
