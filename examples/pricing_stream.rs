//! Streams live EUR/USD and XAU/USD prices. Reconnection (including
//! OANDA's weekend maintenance windows) is handled automatically by the
//! SDK; just keep consuming items. That covers startup too: if the venue is
//! down when this starts, `send()` waits for it rather than failing.
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
            // A transient error is informational — the SDK is already
            // reconnecting. A fatal one means the stream is ending, so it is
            // the last item you will see.
            Err(e) if e.is_fatal() => eprintln!("stream ended: {e}"),
            Err(e) => eprintln!("transient stream error (reconnecting): {e}"),
        }
    }
    Ok(())
}
