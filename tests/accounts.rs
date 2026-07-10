//! Endpoint tests for the accounts domain.

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::Error;
use oanda_rs::endpoints::accounts::ConfigureAccountRejectBody;
use oanda_rs::models::{GuaranteedStopLossOrderMode, InstrumentName};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_accounts() {
    let (server, client) = mock_client().await;
    standard_headers(Mock::given(method("GET")).and(path("/accounts")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accounts": [
                {"id": ACCOUNT_ID, "tags": ["primary"]},
                {"id": "101-004-1234567-002", "mt4AccountID": 555, "tags": []}
            ]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.list_accounts().await.unwrap();
    assert_eq!(response.accounts.len(), 2);
    assert_eq!(
        response.accounts[0].id.as_ref().unwrap().as_str(),
        ACCOUNT_ID
    );
    assert_eq!(response.accounts[0].tags, vec!["primary"]);
    assert_eq!(response.accounts[1].mt4_account_id, Some(555));
}

#[tokio::test]
async fn account_summary() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/summary"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "account": {
            "id": ACCOUNT_ID,
            "alias": "Primary",
            "currency": "EUR",
            "balance": "43650.78835",
            "createdByUserID": 1234567,
            "createdTime": "2024-01-25T21:15:47.268309048Z",
            "guaranteedStopLossOrderMode": "DISABLED",
            "pl": "-56034.41199",
            "resettablePL": "-56034.41199",
            "financing": "-570.91196",
            "commission": "0.0",
            "marginRate": "0.02",
            "openTradeCount": 2,
            "openPositionCount": 1,
            "pendingOrderCount": 0,
            "hedgingEnabled": false,
            "unrealizedPL": "8321.46",
            "NAV": "51972.24835",
            "marginUsed": "1073.839",
            "marginAvailable": "50898.40935",
            "positionValue": "21476.78",
            "marginCloseoutUnrealizedPL": "8467.9",
            "marginCloseoutNAV": "52118.68835",
            "marginCloseoutMarginUsed": "1073.839",
            "marginCloseoutPercent": "0.0103",
            "marginCloseoutPositionValue": "21476.78",
            "withdrawalLimit": "50898.40935",
            "marginCallMarginUsed": "1073.839",
            "marginCallPercent": "0.0206",
            "lastTransactionID": "6356"
        },
        "lastTransactionID": "6356"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client.account_summary(ACCOUNT_ID).await.unwrap();
    let account = response.account;
    assert_eq!(account.alias.as_deref(), Some("Primary"));
    assert_eq!(account.balance.unwrap().to_string(), "43650.78835");
    assert_eq!(
        account.guaranteed_stop_loss_order_mode,
        Some(GuaranteedStopLossOrderMode::Disabled)
    );
    assert_eq!(account.open_trade_count, Some(2));
    assert_eq!(response.last_transaction_id.unwrap().as_str(), "6356");
}

#[tokio::test]
async fn account_instruments_with_filter() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/instruments")))
            .and(query_param("instruments", "EUR_USD,XAU_USD")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "instruments": [{
            "name": "EUR_USD",
            "type": "CURRENCY",
            "displayName": "EUR/USD",
            "pipLocation": -4,
            "displayPrecision": 5,
            "tradeUnitsPrecision": 0,
            "minimumTradeSize": "1",
            "maximumTrailingStopDistance": "1.0",
            "minimumTrailingStopDistance": "0.0005",
            "maximumPositionSize": "0",
            "maximumOrderUnits": "100000000",
            "marginRate": "0.02",
            "tags": [{"type": "ASSET_CLASS", "name": "CURRENCY"}]
        }],
        "lastTransactionID": "6356"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .account_instruments(ACCOUNT_ID)
        .instruments([InstrumentName::EurUsd, InstrumentName::XauUsd])
        .send()
        .await
        .unwrap();
    let instrument = &response.instruments[0];
    assert_eq!(instrument.name, Some(InstrumentName::EurUsd));
    assert_eq!(instrument.pip_location, Some(-4));
    assert_eq!(instrument.tags[0].name.as_deref(), Some("CURRENCY"));
}

#[tokio::test]
async fn configure_account_sends_patch_body() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PATCH"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/configuration")))
            .and(body_json(
                json!({"alias": "new-alias", "marginRate": "0.05"}),
            )),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "clientConfigureTransaction": {
            "id": "6357",
            "time": "2024-06-14T12:01:32.000000000Z",
            "userID": 1234567,
            "accountID": ACCOUNT_ID,
            "batchID": "6357",
            "type": "CLIENT_CONFIGURE",
            "alias": "new-alias",
            "marginRate": "0.05"
        },
        "lastTransactionID": "6357"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .configure_account(ACCOUNT_ID)
        .alias("new-alias")
        .margin_rate("0.05".parse::<oanda_rs::models::DecimalNumber>().unwrap())
        .send()
        .await
        .unwrap();
    let tx = response.client_configure_transaction.unwrap();
    assert_eq!(tx.alias.as_deref(), Some("new-alias"));
    assert_eq!(tx.margin_rate.unwrap().to_string(), "0.05");
}

#[tokio::test]
async fn configure_account_reject_carries_typed_details() {
    let (server, client) = mock_client().await;
    Mock::given(method("PATCH"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/configuration")))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "clientConfigureRejectTransaction": {
                "id": "6358",
                "type": "CLIENT_CONFIGURE_REJECT",
                "marginRate": "99.0",
                "rejectReason": "MARGIN_RATE_INVALID"
            },
            "lastTransactionID": "6357",
            "errorCode": "MARGIN_RATE_INVALID",
            "errorMessage": "The margin rate provided is invalid"
        })))
        .mount(&server)
        .await;

    let error = client
        .configure_account(ACCOUNT_ID)
        .margin_rate("99.0".parse::<oanda_rs::models::DecimalNumber>().unwrap())
        .send()
        .await
        .unwrap_err();
    match &error {
        Error::Api { status, body, .. } => {
            assert_eq!(status.as_u16(), 400);
            assert_eq!(body.error_code.as_deref(), Some("MARGIN_RATE_INVALID"));
            let details: ConfigureAccountRejectBody = body.details().unwrap();
            let tx = details.client_configure_reject_transaction.unwrap();
            assert_eq!(
                tx.reject_reason.as_ref().unwrap().as_str(),
                "MARGIN_RATE_INVALID"
            );
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
    assert_eq!(error.status().map(|s| s.as_u16()), Some(400));
}

#[tokio::test]
async fn unauthorized_maps_to_api_error() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(
            ResponseTemplate::new(401)
                .insert_header("RequestID", "42359180358046734")
                .set_body_json(
                    json!({"errorMessage": "Insufficient authorization to perform request."}),
                ),
        )
        .mount(&server)
        .await;

    let error = client.list_accounts().await.unwrap_err();
    match error {
        Error::Api { status, body, .. } => {
            assert_eq!(status.as_u16(), 401);
            assert!(body.error_message.contains("authorization"));
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn non_json_error_body_is_preserved() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>Bad Gateway</html>"))
        .mount(&server)
        .await;

    let error = client.list_accounts().await.unwrap_err();
    match error {
        Error::Api { status, body, .. } => {
            assert_eq!(status.as_u16(), 502);
            assert_eq!(body.error_message, "<html>Bad Gateway</html>");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn decode_error_keeps_raw_body() {
    let (server, client) = mock_client().await;
    Mock::given(method("GET"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/summary")))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json at all"))
        .mount(&server)
        .await;

    let error = client.account_summary(ACCOUNT_ID).await.unwrap_err();
    match error {
        Error::Decode { body, .. } => assert_eq!(body, "not json at all"),
        other => panic!("expected Error::Decode, got {other:?}"),
    }
}

#[tokio::test]
async fn full_account_details() {
    let (server, client) = mock_client().await;
    standard_headers(Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}"))))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "account": {
                "id": ACCOUNT_ID,
                "currency": "EUR",
                "balance": "43650.78835",
                "openTradeCount": 1,
                "trades": [{
                    "id": "6543",
                    "instrument": "EUR_USD",
                    "price": "1.10423",
                    "state": "OPEN",
                    "currentUnits": "100"
                }],
                "positions": [],
                "orders": [{
                    "type": "LIMIT",
                    "id": "6789",
                    "instrument": "EUR_USD",
                    "units": "100",
                    "price": "1.0900",
                    "state": "PENDING"
                }],
                "lastTransactionID": "6790"
            },
            "lastTransactionID": "6790"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client.account(ACCOUNT_ID).await.unwrap();
    assert_eq!(response.account.trades.len(), 1);
    assert_eq!(response.account.orders.len(), 1);
    assert!(matches!(
        response.account.orders[0],
        oanda_rs::models::Order::Limit(_)
    ));
}

#[tokio::test]
async fn account_changes_polling() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/changes")))
            .and(query_param("sinceTransactionID", "6790")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "changes": {
            "ordersCreated": [],
            "ordersFilled": [{
                "type": "MARKET",
                "id": "6791",
                "instrument": "EUR_USD",
                "units": "100",
                "state": "FILLED"
            }],
            "transactions": [{
                "type": "ORDER_FILL",
                "id": "6792",
                "orderID": "6791"
            }]
        },
        "state": {
            "unrealizedPL": "12.3456",
            "NAV": "43663.13395",
            "marginUsed": "1073.839"
        },
        "lastTransactionID": "6792"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .account_changes(ACCOUNT_ID)
        .since_transaction_id("6790")
        .send()
        .await
        .unwrap();
    let changes = response.changes.unwrap();
    assert_eq!(changes.orders_filled.len(), 1);
    assert_eq!(changes.transactions.len(), 1);
    let state = response.state.unwrap();
    assert_eq!(state.unrealized_pl.unwrap().to_string(), "12.3456");
    assert_eq!(response.last_transaction_id.unwrap().as_str(), "6792");
}
