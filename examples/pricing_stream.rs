//! Streams live EUR/USD and XAU/USD prices. Reconnection (including
//! OANDA's weekend maintenance windows) is handled automatically by the
//! SDK; just keep consuming items.
//!
//! ```sh
//! cargo run --example pricing_stream
//! ```

use futures_util::StreamExt;
use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);
    let account_id: AccountId = std::env::var("OANDA_ACCOUNT_ID")?.into();

    let mut stream = client
        .pricing_stream(account_id, [InstrumentName::EurUsd, InstrumentName::XauUsd])
        .send()
        .await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(PriceStreamItem::Price(price)) => println!(
                "{:?}  bid={:?} ask={:?}",
                price.instrument, price.closeout_bid, price.closeout_ask
            ),
            Ok(PriceStreamItem::Heartbeat(_)) => {
                println!("♥ (reconnects so far: {})", stream.stats().reconnects);
            }
            Ok(_) => {}
            Err(e) => eprintln!("stream error: {e}"),
        }
    }
    Ok(())
}
