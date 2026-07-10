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

`send()` fails fast if the initial connection is rejected (bad token, unknown
account). After that, the stream **manages its own connection** — the `while let`
loop above survives dropped connections, stale sockets, and OANDA's weekend
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

HTTP 4xx rejections during reconnect are treated as fatal (retrying a revoked
token is pointless): the stream yields one final `Err` and ends. Transport errors
and 5xx responses are retried indefinitely by default.

### No data loss on the transaction stream

The transaction stream remembers the last transaction ID it delivered. After every
reconnect it first calls `GET .../transactions/sinceid` and yields the missed
transactions **in order** before resuming live data, deduplicating any overlap.
If the back-fill request itself fails, the stream yields that error (so you know
a gap is possible) and continues streaming live data.

The pricing stream instead reconnects with `snapshot=true`, so you immediately
receive current prices for all subscribed instruments after a gap.

### Observability

- `stream.stats()` returns the number of successful reconnects and failed
  connection attempts.
- With the `tracing` feature enabled, connection loss, scheduled retries and
  successful reconnects are logged at `DEBUG`.

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
| `Err(Error::Api { .. })` (4xx) | Fatal rejection during reconnect | no |
| `Err(...)` after back-fill | Back-fill failed; a gap is possible | yes |
| `Err(...)` with `auto_reconnect(false)` or exhausted attempts | Terminal | no |

## Limits to keep in mind

- Max **20 active streams** per IP; the SDK does not pool streams for you.
- Prices are throttled by OANDA to at most 4 updates/second per instrument.
- Each account is limited on the server side; share one client per process.
