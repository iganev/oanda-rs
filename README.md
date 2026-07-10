# oanda-rs

[![CI](https://github.com/iganev/oanda-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/iganev/oanda-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/iganev/oanda-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/iganev/oanda-rs)
[![crates.io](https://img.shields.io/crates/v/oanda-rs.svg)](https://crates.io/crates/oanda-rs)
[![docs.rs](https://img.shields.io/docsrs/oanda-rs)](https://docs.rs/oanda-rs)
[![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-orange)](https://releases.rs/docs/1.85.0/)

A complete, asynchronous Rust SDK for the [OANDA v20 REST and streaming API](https://developer.oanda.com/rest-live-v20/introduction/), built for multi-threaded tokio environments.

## Highlights

- **Complete** — all 35 v20 operations (accounts, instruments, orders, trades, positions, pricing, transactions) plus both streaming endpoints. See the [coverage table](docs/endpoints.md).
- **One shared client** — `Client` is cheap to clone and `Send + Sync`; all clones share a connection pool and a built-in rate limiter that keeps you under OANDA's per-IP limits (100 of 120 REST req/s, 2 connections/s). No accidental 429s.
- **Self-managing streams** — pricing and transaction streams detect stale connections via heartbeats, reconnect with capped exponential backoff (outage-safe: at most one attempt per 5 minutes during e.g. weekend maintenance), and **back-fill missed transactions** via `sinceid` with deduplication. Consuming them is just a `while let` loop.
- **Typed, faithful models** — every request/response type is `Debug + Clone + Serialize + Deserialize`. Decimals are `rust_decimal::Decimal` newtypes (never floats). The 36-variant `Transaction` and 8-variant `Order` unions are internally tagged enums with lossless `Unknown` fallbacks, so schema drift never breaks deserialization.
- **Lightweight builders** — optional parameters via fluent per-endpoint builders; typed order requests (`MarketOrderRequest::new("EUR_USD", 100).stop_loss_on_fill(...)`).
- **Typed instruments** — `InstrumentName` enumerates all known OANDA symbols (with an `Other` escape hatch), while every API still accepts plain `"EUR_USD"` strings.

## Installation

```toml
[dependencies]
oanda-rs = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
futures-util = "0.3"
```

## Quickstart

```rust,no_run
use futures_util::StreamExt;
use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);

    // REST: place a market order with a stop loss.
    let account = "101-004-1234567-001";
    let response = client
        .create_order(
            account,
            MarketOrderRequest::new(InstrumentName::EurUsd, 100)
                .stop_loss_on_fill(StopLossDetails::at_distance("0.0050".parse()?)),
        )
        .await?;
    println!("filled: {:?}", response.order_fill_transaction.is_some());

    // Streaming: live prices with automatic reconnection.
    let mut prices = client.pricing_stream(account, ["EUR_USD"]).send().await?;
    while let Some(item) = prices.next().await {
        if let PriceStreamItem::Price(price) = item? {
            println!("{:?} / {:?}", price.closeout_bid, price.closeout_ask);
        }
    }
    Ok(())
}
```

More runnable examples in [`examples/`](examples/): [`list_accounts`](examples/list_accounts.rs), [`candles`](examples/candles.rs), [`market_order`](examples/market_order.rs), [`pricing_stream`](examples/pricing_stream.rs), [`transaction_stream`](examples/transaction_stream.rs).

## Feature flags

| Feature   | Default | Description                                                        |
|-----------|---------|--------------------------------------------------------------------|
| `tracing` | off     | `DEBUG`-level instrumentation of requests and stream reconnection. |

## Documentation

- [Getting started](docs/getting-started.md) — tokens, client configuration, error handling
- [Endpoint coverage](docs/endpoints.md) — all 35 operations → SDK methods
- [Streaming guide](docs/streaming.md) — heartbeats, reconnection, back-fill, tuning
- [Rate limiting](docs/rate-limiting.md) — OANDA's limits and the built-in limiter
- [Testing guide](docs/testing.md) — test layers, wiremock patterns, live tests, coverage
- [API reference](https://docs.rs/oanda-rs)

## Testing

~90 tests cover every endpoint (wiremock), every `Transaction`/`Order` variant (byte-exact serde round-trips), NDJSON framing edge cases, and the reconnect state machine under paused tokio time. An opt-in live suite validates against fxPractice:

```sh
cargo test                            # full mock-based suite
cargo test --test live -- --ignored   # read-only live tests (needs .env)
```

## Disclaimer

This is an **unofficial** SDK, not affiliated with or endorsed by OANDA. Trading foreign exchange carries a high level of risk. Develop against the **fxTrade Practice** environment until you are confident in your integration.

## License

BSD 3-Clause — see [LICENSE](LICENSE).
