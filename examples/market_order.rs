//! Places a small EUR/USD market order with take-profit and stop-loss,
//! prints the fill, then closes the trade again.
//!
//! **This trades on the account configured in `.env`.** Use a practice
//! account only.
//!
//! ```sh
//! cargo run --example market_order
//! ```

use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);
    let account_id: AccountId = std::env::var("OANDA_ACCOUNT_ID")?.into();

    let response = client
        .create_order(
            account_id.clone(),
            MarketOrderRequest::new(InstrumentName::EurUsd, 1)
                .stop_loss_on_fill(StopLossDetails::at_distance("0.0100".parse()?))
                .client_extensions(ClientExtensions::new().tag("oanda-rs-example")),
        )
        .await?;

    println!("created: {:?}", response.order_create_transaction.id());
    let Some(fill) = response.order_fill_transaction else {
        println!(
            "order was not filled: {:?}",
            response.order_cancel_transaction
        );
        return Ok(());
    };
    println!("filled at {:?}", fill.price);

    if let Some(opened) = fill.trade_opened {
        let trade_id = opened.trade_id.expect("fill without trade id");
        let closed = client.close_trade(account_id, trade_id).send().await?;
        println!(
            "closed, realized P/L: {:?}",
            closed.order_fill_transaction.and_then(|f| f.pl)
        );
    }
    Ok(())
}
