# Getting started

## 1. Get a token

Create an [fxTrade Practice account](https://www.oanda.com/demo-account/) and generate a
personal access token under **Manage API Access**
(`https://www.oanda.com/demo-account/tpa/personal_token`).

Store it in an `.env` file (never commit it — see [`.env.example`](../.env.example)):

```env
OANDA_TOKEN=your-token-here
OANDA_ACCOUNT_ID=101-004-1234567-001   # optional; discoverable via list_accounts()
```

## 2. Create a client

```rust,no_run
use oanda_rs::{Client, Environment};

let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN").unwrap());
```

`Environment::Practice` talks to `api-fxpractice.oanda.com` (REST) and
`stream-fxpractice.oanda.com` (streams); `Environment::Live` talks to the fxTrade
production hosts. `Environment::Custom` accepts arbitrary base URLs, which is how the
test suite points the client at a mock server.

The client is **cheap to clone** and `Send + Sync`. In a multi-task application, create
it once and clone it into each task — all clones share one connection pool and one rate
limiter:

```rust,no_run
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
let for_worker = client.clone();
tokio::spawn(async move {
    let accounts = for_worker.list_accounts().await;
});
```

Fine-tuning happens through the builder:

```rust,no_run
use oanda_rs::{Client, Environment};
use oanda_rs::models::AcceptDatetimeFormat;

let client = Client::builder()
    .environment(Environment::Practice)
    .token("my-token")
    .datetime_format(AcceptDatetimeFormat::Rfc3339) // or Unix
    .rest_rate_limit(50)                            // default 100 req/s
    .user_agent("my-bot/1.0")
    .build()
    .unwrap();
```

## 3. Call endpoints

Operations without optional parameters are plain async methods; operations with
optional parameters return a builder finished by `.send().await`:

```rust,no_run
# async fn run() -> Result<(), oanda_rs::Error> {
# let client = oanda_rs::Client::new(oanda_rs::Environment::Practice, "t");
use oanda_rs::prelude::*;

// Plain call:
let summary = client.account_summary("101-004-1234567-001").await?;

// Builder call:
let candles = client
    .candles("EUR_USD")
    .granularity(CandlestickGranularity::H1)
    .count(100)
    .send()
    .await?;

// Trading:
let fill = client
    .create_order(
        "101-004-1234567-001",
        MarketOrderRequest::new(InstrumentName::EurUsd, 100)
            .take_profit_on_fill(TakeProfitDetails::at_price("1.1050".parse()?))
            .stop_loss_on_fill(StopLossDetails::at_distance("0.0050".parse()?)),
    )
    .await?
    .order_fill_transaction;
# Ok(())
# }
```

Instruments are typed (`InstrumentName::EurUsd`) but every parameter also accepts the
plain wire string (`"EUR_USD"`); unknown symbols round-trip through
`InstrumentName::Other`.

## 4. Handle errors

Every method returns `Result<_, oanda_rs::Error>`:

- `Error::Api` — OANDA rejected the request; carries the HTTP status, `RequestID` and
  the parsed error body. Order rejections expose their reject transactions through
  `ApiErrorBody::details::<T>()` (e.g. `CreateOrderRejectBody`).
- `Error::Transport` — connection/TLS/timeout problems from reqwest.
- `Error::Decode` — a 2xx body did not match the expected schema; the raw body is
  preserved for debugging.
- `Error::Stream` / `Error::Config` — stream protocol violations / bad client setup.

## 5. Numbers and timestamps

OANDA encodes decimals as JSON strings. The SDK maps them to
[`rust_decimal::Decimal`](https://docs.rs/rust_decimal) newtypes
(`PriceValue`, `DecimalNumber`, `AccountUnits`) — exact arithmetic, no floats.

Timestamps are kept verbatim in a `DateTime` string newtype because the wire format
depends on the `Accept-Datetime-Format` header; `DateTime::to_utc()` parses both the
RFC 3339 and UNIX representations into `chrono::DateTime<Utc>`.

## Next

- [Endpoint coverage](endpoints.md)
- [Streaming guide](streaming.md)
- [Rate limiting](rate-limiting.md)
- [Testing guide](testing.md)
