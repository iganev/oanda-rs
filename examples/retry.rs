//! Retrying ordinary (non-streaming) requests with the SDK's shared policy,
//! instead of hand-rolling status matching and backoff at every call site.
//!
//! ```sh
//! cargo run --example retry
//! ```

use std::time::Duration;

use oanda_rs::prelude::*;
use oanda_rs::{FatalRetry, RetryPolicy, retry};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);

    // The default: retry transient failures (5xx, 429, 408, transport errors)
    // on capped exponential backoff with jitter, and return immediately on
    // anything a retry cannot fix — a bad token, an unknown account.
    let accounts = retry(&RetryPolicy::default(), || client.list_accounts()).await?;
    println!("{} account(s) visible", accounts.accounts.len());

    let account_id: AccountId = std::env::var("OANDA_ACCOUNT_ID")
        .ok()
        .map(Into::into)
        .or_else(|| accounts.accounts.first().and_then(|a| a.id.clone()))
        .ok_or("no account available")?;

    // A batch job wants to fail fast and loudly, but not on a blip: bound the
    // attempts so a broken run ends rather than grinding all night.
    let batch = RetryPolicy::default()
        .backoff(Duration::from_millis(500), Duration::from_secs(30))
        .max_attempts(5);
    let summary = retry(&batch, || client.account_summary(account_id.as_str())).await?;
    println!(
        "balance: {:?} {:?}",
        summary.account.balance, summary.account.currency
    );

    // A long-lived worker wants the opposite: stay up. OANDA has been observed
    // answering 4xx during maintenance, which the classifier calls fatal, so
    // allow those to be retried for a bounded period. The bound matters — an
    // unbounded retry would hide a genuinely revoked token forever. Every such
    // retry is logged at WARN (with the `tracing` feature).
    let worker = RetryPolicy::default().fatal(FatalRetry::Budget(Duration::from_secs(6 * 3600)));
    let instruments = retry(&worker, || {
        client.account_instruments(account_id.as_str()).send()
    })
    .await?;
    println!("{} tradeable instruments", instruments.instruments.len());

    Ok(())
}
