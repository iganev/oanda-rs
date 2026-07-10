//! Live integration tests against the OANDA fxPractice environment.
//!
//! All tests are `#[ignore]`d: they need a practice-account token in `.env`
//! (`OANDA_TOKEN`, optionally `OANDA_ACCOUNT_ID`) and are run explicitly:
//!
//! ```sh
//! cargo test --test live -- --ignored --test-threads=1
//! ```
//!
//! Only read-only endpoints and short stream connections are exercised;
//! nothing is traded or modified. Never run in CI.

use futures_util::StreamExt;
use oanda_rs::models::{AccountId, CandlestickGranularity, InstrumentName};
use oanda_rs::{Client, Environment};

fn client() -> Client {
    dotenvy::dotenv().ok();
    let token = std::env::var("OANDA_TOKEN").expect("OANDA_TOKEN missing from .env");
    Client::new(Environment::Practice, token)
}

/// Discovers the account from the API (asserting it matches
/// `OANDA_ACCOUNT_ID` when that is set).
async fn account_id(client: &Client) -> AccountId {
    let accounts = client.list_accounts().await.expect("list_accounts failed");
    let first = accounts.accounts.first().expect("token has no accounts");
    let id = first.id.clone().expect("account without id");
    if let Ok(expected) = std::env::var("OANDA_ACCOUNT_ID") {
        assert!(
            accounts
                .accounts
                .iter()
                .any(|a| a.id.as_ref().is_some_and(|i| i.as_str() == expected)),
            "OANDA_ACCOUNT_ID={expected} not among the token's accounts"
        );
        return AccountId(expected);
    }
    id
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn account_summary_and_details() {
    let client = client();
    let id = account_id(&client).await;

    let summary = client.account_summary(id.clone()).await.unwrap();
    assert!(summary.account.currency.is_some());
    assert!(summary.account.balance.is_some());

    let full = client.account(id.clone()).await.unwrap();
    assert_eq!(
        full.account.id.as_ref().map(AccountId::as_str),
        Some(id.as_str())
    );

    // The changes-polling endpoint must accept the last transaction ID.
    let last = full.last_transaction_id.clone().unwrap();
    let changes = client
        .account_changes(id)
        .since_transaction_id(last)
        .send()
        .await
        .unwrap();
    assert!(changes.state.is_some());
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn instruments_are_all_known_symbols() {
    let client = client();
    let id = account_id(&client).await;
    let response = client.account_instruments(id).send().await.unwrap();
    assert!(!response.instruments.is_empty());
    let unknown: Vec<_> = response
        .instruments
        .iter()
        .filter_map(|i| i.name.as_ref())
        .filter(|name| !name.is_known())
        .collect();
    assert!(
        unknown.is_empty(),
        "symbol table needs refreshing; unknown instruments: {unknown:?}"
    );
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn candles_and_books() {
    let client = client();
    let candles = client
        .candles(InstrumentName::EurUsd)
        .granularity(CandlestickGranularity::H1)
        .count(10)
        .send()
        .await
        .unwrap();
    assert_eq!(candles.candles.len(), 10);
    assert!(candles.candles[0].mid.is_some());
    assert!(candles.candles[0].time.as_ref().unwrap().to_utc().is_some());

    let book = client
        .instrument_order_book(InstrumentName::EurUsd)
        .send()
        .await
        .unwrap();
    assert!(!book.order_book.buckets.is_empty());
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn prices_and_open_state() {
    let client = client();
    let id = account_id(&client).await;

    let prices = client
        .prices(id.clone(), [InstrumentName::EurUsd, InstrumentName::UsdJpy])
        .include_home_conversions(true)
        .send()
        .await
        .unwrap();
    assert_eq!(prices.prices.len(), 2);

    // Read-only listings must deserialize cleanly whatever the account holds.
    client.list_open_trades(id.clone()).await.unwrap();
    client.list_open_positions(id.clone()).await.unwrap();
    client.list_pending_orders(id.clone()).await.unwrap();
    let transactions = client
        .list_transactions(id)
        .page_size(100)
        .send()
        .await
        .unwrap();
    assert!(transactions.last_transaction_id.is_some());
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn pricing_stream_smoke() {
    let client = client();
    let id = account_id(&client).await;
    let mut stream = client
        .pricing_stream(id, [InstrumentName::EurUsd])
        .send()
        .await
        .unwrap();
    // Expect at least one item (snapshot price or heartbeat) within 10s
    // even on weekends.
    let item = tokio::time::timeout(std::time::Duration::from_secs(10), stream.next())
        .await
        .expect("no stream item within 10s")
        .expect("stream ended unexpectedly");
    item.expect("stream item failed to decode");
}

#[tokio::test]
#[ignore = "hits the live fxPractice API"]
async fn transaction_history_roundtrips() {
    let client = client();
    let id = account_id(&client).await;
    let last = client
        .account_summary(id.clone())
        .await
        .unwrap()
        .last_transaction_id
        .unwrap();
    let last_num: u64 = last.as_str().parse().unwrap();
    let from = last_num.saturating_sub(100).max(1);
    let range = client
        .transactions_id_range(id, from.to_string(), last.clone())
        .send()
        .await
        .unwrap();
    // Every historical transaction must deserialize into a known variant
    // (Unknown would signal schema drift).
    for tx in &range.transactions {
        assert!(
            !matches!(tx, oanda_rs::models::transaction::Transaction::Unknown(_)),
            "unknown transaction type in history: {:?}",
            tx.type_name()
        );
    }
}
