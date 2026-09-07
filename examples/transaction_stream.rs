//! Streams the account's transactions. If the connection drops, the SDK
//! reconnects with backoff and back-fills anything missed via
//! `GET .../transactions/sinceid`, so no transaction is silently lost. The
//! same backoff covers the first connection, so starting while OANDA is
//! down waits for it to return instead of exiting.
//!
//! ```sh
//! cargo run --example transaction_stream
//! ```

use futures_util::StreamExt;
use oanda_rs::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let client = Client::new(Environment::Practice, std::env::var("OANDA_TOKEN")?);
    let account_id: AccountId = std::env::var("OANDA_ACCOUNT_ID")?.into();

    let mut stream = client.transaction_stream(account_id).send().await?;
    println!("waiting for transactions (place a trade to see events) ...");

    while let Some(item) = stream.next().await {
        match item {
            Ok(TransactionStreamItem::Transaction(tx)) => {
                println!(
                    "{:?}: id={:?} time={:?}",
                    tx.type_name(),
                    tx.id(),
                    tx.time()
                );
            }
            Ok(TransactionStreamItem::Heartbeat(hb)) => {
                println!("♥ last transaction id: {:?}", hb.last_transaction_id);
            }
            Ok(_) => {}
            // Fatal means this is the last item; transient means the SDK is
            // already reconnecting behind the scenes.
            Err(e) if e.is_fatal() => eprintln!("stream ended: {e}"),
            Err(e) => eprintln!("transient stream error (reconnecting): {e}"),
        }
    }
    Ok(())
}
