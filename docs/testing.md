# Testing guide

## Test layers

| Layer | Where | What it proves |
|---|---|---|
| Unit tests | `src/**` (`cargo test --lib`) | rate limiter timing (paused tokio time), NDJSON framing edge cases, reconnect state machine (backoff escalation/reset, watchdog, fatal errors), primitives/serde helpers |
| Serde round-trips | `tests/serde_roundtrip.rs` + `tests/fixtures/` | every `Transaction` (36) and `Order` (8) variant deserializes into the right variant and re-serializes byte-identically; unknown types fall back losslessly |
| Endpoint tests | `tests/{accounts,instruments,orders,trades,positions,transactions,streaming}.rs` | one wiremock test per operation: URL, query, auth + `Accept-Datetime-Format` headers, request bodies (`body_json`), typed responses, error mapping, reject-body decoding |
| Stream integration | `tests/streaming.rs` | NDJSON parsing over HTTP, heartbeat surfacing, reconnect + `sinceid` back-fill + deduplication, fail-fast on 401 |
| Live tests | `tests/live.rs` (all `#[ignore]`) | schema fidelity against the real fxPractice API |

## Running

```sh
cargo test                                   # everything except live tests
cargo test --test streaming                  # one suite
cargo test --test live -- --ignored          # live fxPractice tests (needs .env)
```

Live tests are read-only (accounts, instruments, candles, prices, history, a
short pricing-stream connection) and require:

```env
OANDA_TOKEN=...          # practice token
OANDA_ACCOUNT_ID=...     # optional; the suite discovers accounts via GET /v3/accounts
```

They intentionally fail if the API returns an instrument or transaction type the
SDK doesn't know — that is the signal to refresh the symbol table / models.

## Testing your own code against a mock

`Environment::Custom` points a client at any base URL, which makes the SDK
trivially testable with [wiremock](https://docs.rs/wiremock):

```rust,ignore
let server = wiremock::MockServer::start().await;
let url: reqwest::Url = server.uri().parse().unwrap();
let client = oanda_rs::Client::builder()
    .environment(oanda_rs::Environment::Custom { rest: url.clone(), stream: url })
    .token("test-token")
    .build()
    .unwrap();
```

## Coverage

CI generates coverage with [`cargo-llvm-cov`](https://github.com/taiki-e/cargo-llvm-cov)
and uploads it to Codecov. Locally:

```sh
cargo llvm-cov --all-features --open
```
