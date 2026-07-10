//! Lists the accounts available to the token and prints each summary.
//!
//! ```sh
//! cargo run --example list_accounts
//! ```

use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);

    for properties in client.list_accounts().await?.accounts {
        let id = properties.id.expect("account without id");
        let summary = client.account_summary(id.clone()).await?.account;
        println!(
            "{id}: alias={:?} currency={:?} balance={:?} NAV={:?} open trades={:?}",
            summary.alias, summary.currency, summary.balance, summary.nav, summary.open_trade_count
        );
    }
    Ok(())
}
