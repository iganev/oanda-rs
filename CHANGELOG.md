# Changelog

All notable changes to this project are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/); the crate is pre-1.0,
so minor versions may contain breaking changes.

## [0.2.2] - 2026-09-07

### Added

- `Instrument::round_to_display_precision` / `conform_to_display_precision`
  and `Instrument::validate_trailing_distance` / `validate_trailing_distance`
  (with `TrailingDistanceError`): a stop-loss or trailing distance that does
  not conform to the instrument's `displayPrecision` and distance limits is
  rejected by OANDA with an HTTP 400, so callers can conform and check a value
  before sending the order.

### Fixed

- The transaction stream's reconnect back-fill (`GET .../transactions/sinceid`)
  now runs under the stream's own `RetryPolicy`. It used the client's plain
  request path, so a 4xx the stream had just ridden out under a
  `FatalRetry::Budget` failed the back-fill instantly and surfaced as an `Err`
  item — seen in production as a 401 flicker that a consumer treating `Err`
  items as terminal turned into a process exit.

## [0.2.0] - 2026-07-26

Resilience is now a single, shared policy rather than something each caller
reimplements. Found in production: a worker started during a weekend outage met
a Cloudflare `520` on the pricing stream and exited, because the SDK's
outage-safe backoff only applied *after* one successful connection.

### Added

- `Error::is_transient()` / `Error::is_fatal()` — the classification the SDK
  applies to its own retries, exposed so callers stop matching on statuses by
  hand. 5xx, 429, 408 and transport failures are transient; other 4xx, decode
  failures and configuration mistakes are not.
- `RetryPolicy` and `retry()` — capped exponential backoff with jitter for
  ordinary requests, using the same classification as the streams.
- `FatalRetry` — how errors that cannot succeed on a retry are handled.
  `FailFast` (the default) preserves existing behaviour; `Budget(Duration)`
  retries them anyway for a bounded period, for long-lived workers that must
  ride out maintenance windows in which OANDA answers 4xx. Every such retry is
  logged at `WARN` (with the `tracing` feature) and counted, because an
  unbounded, silent retry would mask a genuinely revoked credential.
- `StreamStats::fatal_retries`, and the `fatal_retry()` / `retry_policy()`
  setters on both stream builders.
- `StatusCode` re-export at the crate root, so matching on `Error::Api.status`
  no longer requires a version-matched `reqwest`.
- `examples/retry.rs`.

### Changed

- **Stream `send()` now retries a transient rejection** instead of failing
  immediately, governed by the existing `auto_reconnect` (enabled by default).
  A stream opened while the venue is down now waits for it. Fatal rejections —
  a bad token, an unknown account — still surface immediately, so the
  fail-fast property that mattered is preserved. Use `auto_reconnect(false)`
  for unconditional fail-fast.
- **HTTP 429 is now transient.** A rate limit during a reconnect used to end
  the stream permanently, which contradicted both `Error::is_rate_limited` and
  the client's own connection limiter.
- **A fatal error on an established connection now ends the stream.**
  Previously only the connect path consulted the classifier, so a revoked
  token mid-stream caused an endless reconnect loop.
- `Error::Decode` is classified fatal (a retry returns the same bytes). It was
  previously retried on the connect path, where it is unreachable in practice.
- `StreamConfig` is public, and holds a `RetryPolicy` in place of its separate
  `backoff_*` / `max_reconnect_attempts` fields. The builder setters are
  unchanged apart from the item below.
- `max_reconnect_attempts` accepts `impl Into<Option<u32>>`, so an explicit
  limit can be lifted again with `None`.

### Fixed

- **Reconnect jitter did not jitter.** The factor derived from
  `Instant::now().elapsed()`, which is ~0 (exactly 0 under a paused clock), so
  for any whole-second delay it evaluated to a constant `0.75`. Two
  consequences: independent clients never decorrelated — the opposite of the
  function's purpose, and a real thundering-herd risk for a fleet reconnecting
  after one outage — and every delay was 25% shorter than configured, making
  the documented 5-minute cap 225s in practice. Replaced with a per-stream
  xorshift PRNG seeded from `RandomState` (no new dependencies), spreading
  delays across the full ±25% band.
