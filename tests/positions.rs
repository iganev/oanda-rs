//! Endpoint tests for the positions domain.

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::models::InstrumentName;
use serde_json::json;
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, ResponseTemplate};

fn position_body() -> serde_json::Value {
    json!({
        "instrument": "EUR_USD",
        "pl": "-54.2591",
        "unrealizedPL": "0.0000",
        "resettablePL": "-54.2591",
        "long": {
            "units": "100",
            "averagePrice": "1.10423",
            "pl": "-4.2591",
            "unrealizedPL": "0.0000",
            "resettablePL": "-4.2591",
            "tradeIDs": ["6543"]
        },
        "short": {
            "units": "0",
            "pl": "-50.0000",
            "unrealizedPL": "0.0000",
            "resettablePL": "-50.0000"
        }
    })
}

#[tokio::test]
async fn list_and_open_positions() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/positions"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "positions": [position_body()],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/openPositions"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "positions": [],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let all = client.list_positions(ACCOUNT_ID).await.unwrap();
    assert_eq!(all.positions.len(), 1);
    let long = all.positions[0].long.as_ref().unwrap();
    assert_eq!(long.average_price.unwrap().to_string(), "1.10423");
    assert_eq!(long.trade_ids[0].as_str(), "6543");

    let open = client.list_open_positions(ACCOUNT_ID).await.unwrap();
    assert!(open.positions.is_empty());
}

#[tokio::test]
async fn get_single_position() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/positions/EUR_USD"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "position": position_body(),
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .position(ACCOUNT_ID, InstrumentName::EurUsd)
        .await
        .unwrap();
    assert_eq!(response.position.instrument, Some(InstrumentName::EurUsd));
}

#[tokio::test]
async fn close_position_long_side() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!(
                "/accounts/{ACCOUNT_ID}/positions/EUR_USD/close"
            )))
            .and(body_json(json!({"longUnits": "ALL"}))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "longOrderCreateTransaction": {
            "type": "MARKET_ORDER",
            "id": "7000",
            "instrument": "EUR_USD",
            "units": "-100",
            "longPositionCloseout": {"instrument": "EUR_USD", "units": "ALL"}
        },
        "longOrderFillTransaction": {
            "type": "ORDER_FILL",
            "id": "7001",
            "orderID": "7000",
            "pl": "1.2000"
        },
        "relatedTransactionIDs": ["7000", "7001"],
        "lastTransactionID": "7001"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .close_position(ACCOUNT_ID, "EUR_USD")
        .long_units("ALL")
        .send()
        .await
        .unwrap();
    assert!(response.long_order_create_transaction.is_some());
    assert_eq!(
        response
            .long_order_fill_transaction
            .unwrap()
            .pl
            .unwrap()
            .to_string(),
        "1.2000"
    );
}
