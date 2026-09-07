# Streaming guide

OANDA provides two streaming endpoints — prices and transactions — served from a
dedicated host (`stream-fxpractice.oanda.com` / `stream-fxtrade.oanda.com`) as
chunked, newline-delimited JSON with a heartbeat line every 5 seconds. The SDK
routes stream requests to the correct host automatically.

## Basic usage

```rust,no_run
# async fn run() -> Result<(), oanda_rs::Error> {
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
use futures_util::StreamExt;
use oanda_rs::prelude::*;

let mut prices = client
    .pricing_stream("101-004-1234567-001", ["EUR_USD", "XAU_USD"])
    .send()
    .await?;

while let Some(item) = prices.next().await {
    match item? {
        PriceStreamItem::Price(price) => { /* ... */ }
        PriceStreamItem::Heartbeat(_) => { /* liveness signal */ }
        _ => {}
    }
}
# Ok(())
# }
```

`send()` fails fast if the initial connection is rejected for a reason retrying
cannot fix — a bad token, an unknown account. A *transient* rejection (a 5xx
from OANDA's edge while the venue is closed, a rate limit, a dropped
connection) is retried on the same backoff the running stream uses, so a worker
started **during** an outage waits for the venue to return instead of exiting.
`auto_reconnect` is the single switch governing both the initial connect and
later reconnects; disabling it restores unconditional fail-fast.

After that, the stream **manages its own connection** — the `while let` loop
above survives dropped connections, stale sockets, and OANDA's weekend
maintenance windows without any extra code.

## What the managed stream does for you

### Stale-connection detection

OANDA sends a heartbeat every 5 seconds. If *nothing* arrives for
`heartbeat_timeout` (default **10s**), the connection is presumed dead and
replaced. Heartbeats are also yielded to your code, so you can implement your own
liveness indicators.

### Reconnection with outage-safe backoff

Reconnect attempts use exponential backoff with ±25% jitter: starting at **1s**,
doubling to a cap of **5 minutes**. During a long outage (OANDA regularly takes
the API down for maintenance over the weekend) the stream settles at roughly one
gentle attempt every 5 minutes instead of hammering the endpoint.

The backoff resets to 1s only after a connection has stayed healthy for
`backoff_reset_after` (default **60s**) — a connection that dies right after
connecting keeps escalating, so connect/die/connect churn cannot bypass the
cooldown. Every attempt additionally passes through the client's shared
2-connections-per-second limiter (OANDA's per-IP cap).

### Which failures are retried

One classification — [`Error::is_transient`] — governs the initial connect,
reconnects and mid-stream failures alike:

| Failure | Retried? |
|---|---|
| Transport error (connection, TLS, DNS, timeout) | yes |
| HTTP 5xx (including the Cloudflare `520` OANDA's edge serves at weekends) | yes |
| HTTP 429 (rate limited) | yes — backing off is the correct response |
| HTTP 408 (request timeout) | yes |
| Any other 4xx (bad token, unknown account) | no — the stream ends with one final `Err` |

Transient failures are retried indefinitely unless you set
`max_reconnect_attempts`.

### Riding out maintenance that answers 4xx

OANDA has been observed serving *temporary* 4xx during maintenance, which the
table above would treat as terminal. A long-lived worker can opt into retrying
them anyway, for a bounded period:

```rust,no_run
# async fn run() -> Result<(), oanda_rs::Error> {
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
use std::time::Duration;
use oanda_rs::FatalRetry;

let stream = client
    .pricing_stream("101-004-1234567-001", ["EUR_USD"])
    // Ride out a maintenance window, but still surface a genuinely revoked
    // credential after six hours rather than retrying it forever.
    .fatal_retry(FatalRetry::Budget(Duration::from_secs(6 * 3600)))
    .send()
    .await?;
# Ok(())
# }
```

The budget is measured from the first fatal error and **cleared by any
successful connection**, so an unrelated failure later gets a full budget
again. The cost is real: a token revoked at the start of the window stays
hidden for its duration. Because of that, every such retry is logged at `WARN`
(with the `tracing` feature) and counted in `stats().fatal_retries` — alert on
that counter rather than discovering the problem when the budget expires.

### No data loss on the transaction stream

The transaction stream remembers the last transaction ID it delivered. After every
reconnect it first calls `GET .../transactions/sinceid` and yields the missed
transactions **in order** before resuming live data, deduplicating any overlap.
The back-fill request obeys the stream's retry policy — backoff, attempt limit
and [`FatalRetry`](#riding-out-maintenance-that-answers-4xx) budget alike — so a
rejection the reconnect itself rode out cannot fail the back-fill. Only when the
policy gives up does the stream yield that error (so you know a gap is possible)
and continue streaming live data.

The pricing stream instead reconnects with `snapshot=true`, so you immediately
receive current prices for all subscribed instruments after a gap.

### Observability

- `stream.stats()` returns the number of successful reconnects, failed
  connection attempts, and fatal errors retried under a
  [`FatalRetry::Budget`](#riding-out-maintenance-that-answers-4xx).
- With the `tracing` feature enabled, connection loss, scheduled retries and
  successful reconnects are logged at `DEBUG`; a retried *fatal* error is
  logged at `WARN`, since it means the stream is running on borrowed time.

## Tuning

All knobs live on the stream request builders:

```rust,no_run
# async fn run() -> Result<(), oanda_rs::Error> {
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
use std::time::Duration;

let stream = client
    .transaction_stream("101-004-1234567-001")
    .heartbeat_timeout(Duration::from_secs(15))
    .backoff(Duration::from_secs(2), Duration::from_secs(600))
    .backoff_reset_after(Duration::from_secs(120))
    .max_reconnect_attempts(50)   // default: unlimited
    .send()
    .await?;
# Ok(())
# }
```

To manage connections yourself, opt out with `.auto_reconnect(false)`: the stream
then ends (or yields a single `Err`) on the first connection problem.

## Error items

| Item | Meaning | Stream continues? |
|---|---|---|
| `Err(Error::Decode { .. })` | One malformed line (raw body preserved) | yes |
| `Err(Error::Api { .. })` (4xx other than 429/408) | Fatal rejection, on connect or mid-stream | no |
| `Err(...)` after back-fill | Back-fill failed after exhausting the retry policy; a gap is possible | yes |
| `Err(...)` with `auto_reconnect(false)` or exhausted attempts | Terminal | no |

A fatal rejection ends the stream wherever it occurs — the connect path and an
established connection obey the same rule.

## Limits to keep in mind

- Max **20 active streams** per IP; the SDK does not pool streams for you.
- Prices are throttled by OANDA to at most 4 updates/second per instrument.
- Each account is limited on the server side; share one client per process.
