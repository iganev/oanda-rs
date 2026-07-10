//! Fetches recent hourly candles for EUR/USD and prints them.
//!
//! ```sh
//! cargo run --example candles
//! ```

use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);

    let candles = client
        .candles(InstrumentName::EurUsd)
        .granularity(CandlestickGranularity::H1)
        .count(24)
        .price(PricingComponent::MID)
        .send()
        .await?;

    println!("{} {:?}", candles.candles.len(), candles.instrument);
    for candle in candles.candles {
        let mid = candle.mid.unwrap();
        println!(
            "{}  O:{} H:{} L:{} C:{}  volume={} complete={}",
            candle.time.unwrap(),
            mid.o.unwrap(),
            mid.h.unwrap(),
            mid.l.unwrap(),
            mid.c.unwrap(),
            candle.volume.unwrap_or(0),
            candle.complete.unwrap_or(false),
        );
    }
    Ok(())
}
