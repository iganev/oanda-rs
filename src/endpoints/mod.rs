//! Endpoint implementations, grouped by API domain.
//!
//! Every operation is a method on [`Client`](crate::Client): operations
//! without optional parameters are plain `async fn`s; operations with
//! optional parameters return a lightweight builder whose terminal
//! `send().await` performs the request. Per-operation response types live
//! next to their endpoint in these modules.

pub mod accounts;
pub mod instruments;
pub mod orders;
pub mod positions;
pub mod pricing;
pub mod trades;
pub mod transactions;
