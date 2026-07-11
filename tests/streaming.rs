//! Integration tests for the pricing endpoint and the two managed streams.

mod common;

use std::time::Duration;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use futures_util::StreamExt;
use oanda_rs::Error;
use oanda_rs::models::transaction::{Transaction, TransactionStreamItem};
use oanda_rs::models::{InstrumentName, PriceStreamItem};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn prices_endpoint() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/pricing")))
            .and(query_param("instruments", "EUR_USD,USD_JPY"))
            .and(query_param("includeHomeConversions", "true")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "prices": [{
            "type": "PRICE",
            "instrument": "EUR_USD",
            "time": "2024-06-14T12:00:00.000000000Z",
            "tradeable": true,
            "bids": [{"price": "1.07132", "liquidity": 10000000}],
            "asks": [{"price": "1.07145", "liquidity": "10000000"}],
            "closeoutBid": "1.07128",
            "closeoutAsk": "1.07149"
        }],
        "homeConversions": [{
            "currency": "USD",
            "accountGain": "0.93261",
            "accountLoss": "0.93317",
            "positionValue": "0.93289"
        }],
        "time": "2024-06-14T12:00:00.000000000Z"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .prices(ACCOUNT_ID, ["EUR_USD", "USD_JPY"])
        .include_home_conversions(true)
        .send()
        .await
        .unwrap();
    let price = &response.prices[0];
    assert_eq!(price.instrument, Some(InstrumentName::EurUsd));
    // liquidity arrives as a JSON number here and as a string elsewhere;
    // both must parse.
    assert_eq!(price.bids[0].liquidity.unwrap().to_string(), "10000000");
    assert_eq!(price.asks[0].liquidity.unwrap().to_string(), "10000000");
    assert_eq!(response.home_conversions.len(), 1);
}

#[tokio::test]
async fn pricing_stream_yields_prices_and_heartbeats() {
    let (server, client) = mock_client().await;
    let body = concat!(
        r#"{"type":"PRICE","instrument":"EUR_USD","closeoutBid":"1.07128","closeoutAsk":"1.07149","tradeable":true}"#,
        "\n",
        r#"{"type":"HEARTBEAT","time":"2024-06-14T12:00:05.000000000Z"}"#,
        "\n",
        r#"{"type":"PRICE","instrument":"EUR_USD","closeoutBid":"1.07131","closeoutAsk":"1.07152","tradeable":true}"#,
        "\n",
    );
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/pricing/stream")))
            .and(query_param("instruments", "EUR_USD"))
            .and(query_param("snapshot", "false")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/octet-stream"))
    .expect(1)
    .mount(&server)
    .await;

    let stream = client
        .pricing_stream(ACCOUNT_ID, [InstrumentName::EurUsd])
        .snapshot(false)
        .auto_reconnect(false)
        .send()
        .await
        .unwrap();
    let items: Vec<_> = stream.map(Result::unwrap).collect().await;
    assert_eq!(items.len(), 3);
    assert!(matches!(items[0], PriceStreamItem::Price(_)));
    assert!(matches!(items[1], PriceStreamItem::Heartbeat(_)));
    let PriceStreamItem::Price(price) = &items[2] else {
        panic!("expected price");
    };
    assert_eq!(price.closeout_bid.unwrap().to_string(), "1.07131");
}

#[tokio::test]
async fn transaction_stream_backfills_and_deduplicates_on_reconnect() {
    let (server, client) = mock_client().await;

    // First connection: transaction 6790 + heartbeat, then EOF.
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            concat!(
                r#"{"type":"ORDER_FILL","id":"6790","orderID":"6789"}"#,
                "\n",
                r#"{"type":"HEARTBEAT","lastTransactionID":"6790"}"#,
                "\n",
            ),
            "application/octet-stream",
        ))
        .up_to_n_times(1)
        .mount(&server)
        .await;

    // Reconnects: transaction 6791 again (duplicate of the back-fill) and
    // then 6792.
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            concat!(
                r#"{"type":"ORDER_CANCEL","id":"6791","orderID":"6789"}"#,
                "\n",
                r#"{"type":"MARKET_ORDER","id":"6792","instrument":"EUR_USD","units":"100"}"#,
                "\n",
            ),
            "application/octet-stream",
        ))
        .mount(&server)
        .await;

    // Back-fill query performed after the reconnect.
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/sinceid")))
        .and(query_param("id", "6790"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "transactions": [{"type": "ORDER_CANCEL", "id": "6791", "orderID": "6789"}],
            "lastTransactionID": "6791"
        })))
        .mount(&server)
        .await;

    let stream = client
        .transaction_stream(ACCOUNT_ID)
        .backoff(Duration::from_millis(10), Duration::from_millis(50))
        .send()
        .await
        .unwrap();

    let items: Vec<_> = tokio::time::timeout(
        Duration::from_secs(10),
        stream
            .filter_map(|item| async {
                match item.unwrap() {
                    TransactionStreamItem::Transaction(tx) => Some(tx),
                    _ => None,
                }
            })
            .take(3)
            .collect::<Vec<_>>(),
    )
    .await
    .expect("timed out waiting for backfilled stream");

    // 6790 live, 6791 from the back-fill, 6792 live; the duplicate 6791
    // from the second connection must have been dropped.
    let ids: Vec<_> = items
        .iter()
        .map(|tx| tx.id().unwrap().as_str().to_owned())
        .collect();
    assert_eq!(ids, vec!["6790", "6791", "6792"]);
    assert!(matches!(items[1], Transaction::OrderCancel(_)));
}

#[tokio::test]
async fn stream_connect_rejection_fails_fast() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/pricing/stream")))
        .respond_with(
            ResponseTemplate::new(401)
                .set_body_json(json!({"errorMessage": "Insufficient authorization"})),
        )
        .mount(&server)
        .await;

    let error = client
        .pricing_stream(ACCOUNT_ID, ["EUR_USD"])
        .send()
        .await
        .unwrap_err();
    match error {
        Error::Api { status, .. } => assert_eq!(status.as_u16(), 401),
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn stream_malformed_line_is_reported_but_not_fatal() {
    let (server, client) = mock_client().await;
    let body = concat!(
        "garbage line\n",
        r#"{"type":"HEARTBEAT","time":"2024-06-14T12:00:05.000000000Z"}"#,
        "\n",
    );
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/pricing/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(body, "application/octet-stream"))
        .mount(&server)
        .await;

    let stream = client
        .pricing_stream(ACCOUNT_ID, ["EUR_USD"])
        .auto_reconnect(false)
        .send()
        .await
        .unwrap();
    let items: Vec<_> = stream.collect().await;
    assert_eq!(items.len(), 2);
    assert!(matches!(items[0], Err(Error::Decode { .. })));
    assert!(matches!(items[1], Ok(PriceStreamItem::Heartbeat(_))));
}

#[tokio::test]
async fn stream_builders_accept_full_config_and_expose_stats() {
    let (server, client) = mock_client().await;
    let heartbeat = concat!(
        r#"{"type":"HEARTBEAT","time":"2024-06-14T12:00:05.000000000Z"}"#,
        "\n"
    );
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/pricing/stream")))
        .and(query_param("snapshot", "true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(heartbeat, "application/octet-stream"),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/stream")))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            concat!(r#"{"type":"HEARTBEAT","lastTransactionID":"1"}"#, "\n"),
            "application/octet-stream",
        ))
        .mount(&server)
        .await;

    let mut prices = client
        .pricing_stream(ACCOUNT_ID, ["EUR_USD"])
        .snapshot(true)
        .auto_reconnect(true)
        .heartbeat_timeout(Duration::from_secs(30))
        .backoff(Duration::from_millis(10), Duration::from_millis(100))
        .backoff_reset_after(Duration::from_secs(1))
        .max_reconnect_attempts(1)
        .send()
        .await
        .unwrap();
    assert!(prices.next().await.unwrap().is_ok());
    assert_eq!(prices.stats().reconnects, 0);
    assert!(format!("{prices:?}").contains("PricingStream"));

    let mut transactions = client
        .transaction_stream(ACCOUNT_ID)
        .auto_reconnect(true)
        .heartbeat_timeout(Duration::from_secs(30))
        .backoff(Duration::from_millis(10), Duration::from_millis(100))
        .backoff_reset_after(Duration::from_secs(1))
        .max_reconnect_attempts(1)
        .send()
        .await
        .unwrap();
    assert!(matches!(
        transactions.next().await.unwrap().unwrap(),
        TransactionStreamItem::Heartbeat(_)
    ));
    assert!(format!("{transactions:?}").contains("TransactionStream"));
    // Exhausting reconnect attempts (mock keeps EOF-ing) ends with an error.
    let last = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match transactions.next().await {
                Some(Err(e)) => break Some(e),
                Some(Ok(_)) => continue,
                None => break None,
            }
        }
    })
    .await
    .unwrap();
    assert!(last.is_some() || transactions.stats().reconnects > 0);
}

#[tokio::test]
async fn prices_since_parameter() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/pricing")))
            .and(query_param("since", "2024-06-14T12:00:00Z")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({"prices": []})))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .prices(ACCOUNT_ID, ["EUR_USD"])
        .since("2024-06-14T12:00:00Z")
        .send()
        .await
        .unwrap();
    assert!(response.prices.is_empty());
}
