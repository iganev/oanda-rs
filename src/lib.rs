//! # oanda-rs
//!
//! An asynchronous Rust SDK for the [OANDA v20 REST and streaming
//! API](https://developer.oanda.com/rest-live-v20/introduction/), designed
//! for multi-threaded tokio environments.
//!
//! - **One shared client** — [`Client`] is cheap to clone and `Send + Sync`;
//!   all clones share a connection pool and a built-in rate limiter that
//!   keeps you under OANDA's per-IP limits (120 REST requests/s, 2 new
//!   connections/s).
//! - **Typed models** — every request/response type derives `Debug`,
//!   `Serialize` and `Deserialize`, with decimals as
//!   [`rust_decimal::Decimal`] newtypes (never floats).
//! - **Streaming** — pricing and transaction streams are self-managing:
//!   they detect stale connections via heartbeats, reconnect with capped
//!   exponential backoff, and back-fill missed transactions.
//!
//! ## Quickstart
//!
//! ```no_run
//! use oanda_rs::{Client, Environment};
//!
//! # async fn run() -> Result<(), oanda_rs::Error> {
//! let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN").unwrap());
//! # Ok(())
//! # }
//! ```
//!
//! This is an unofficial SDK; use the Practice environment until you are
//! confident in your integration. See the repository's `docs/` directory
//! for guides on streaming, rate limiting and testing.

#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

mod client;
mod error;
mod rate_limit;
mod transport;

pub mod endpoints;
pub mod models;
pub mod prelude;
pub mod streaming;

pub use client::{Client, ClientBuilder, Environment};
pub use error::{ApiErrorBody, Error};
