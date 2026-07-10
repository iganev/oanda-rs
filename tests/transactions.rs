//! Endpoint tests for the transactions domain (REST).

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::models::TransactionId;
use oanda_rs::models::transaction::{Transaction, TransactionFilter};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_transactions_pages() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/transactions")))
            .and(query_param("pageSize", "100"))
            .and(query_param("type", "ORDER,FUNDING")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "from": "2024-06-01T00:00:00.000000000Z",
        "to": "2024-06-14T00:00:00.000000000Z",
        "pageSize": 100,
        "type": ["ORDER", "FUNDING"],
        "count": 245,
        "pages": [
            "https://api-fxpractice.oanda.com/v3/accounts/x/transactions/idrange?from=1&to=100"
        ],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .list_transactions(ACCOUNT_ID)
        .page_size(100)
        .types([TransactionFilter::Order, TransactionFilter::Funding])
        .send()
        .await
        .unwrap();
    assert_eq!(response.count, Some(245));
    assert_eq!(response.pages.len(), 1);
    assert_eq!(response.r#type.len(), 2);
}

#[tokio::test]
async fn get_single_transaction() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/transactions/6789"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "transaction": {
            "type": "CLIENT_CONFIGURE",
            "id": "6789",
            "accountID": ACCOUNT_ID,
            "alias": "renamed",
            "time": "2024-06-14T12:00:00.000000000Z"
        },
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .transaction(ACCOUNT_ID, TransactionId::from("6789"))
        .await
        .unwrap();
    assert!(matches!(
        response.transaction,
        Transaction::ClientConfigure(_)
    ));
    assert_eq!(response.transaction.id().unwrap().as_str(), "6789");
}

#[tokio::test]
async fn id_range_and_since_id() {
    let (server, client) = mock_client().await;
    let body = json!({
        "transactions": [
            {"type": "ORDER_FILL", "id": "6790", "orderID": "6789", "pl": "0.5"},
            {"type": "SOME_NEW_TYPE", "id": "6791", "novel": true}
        ],
        "lastTransactionID": "6791"
    });
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/idrange")))
            .and(query_param("from", "6790"))
            .and(query_param("to", "6791")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
    .expect(1)
    .mount(&server)
    .await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/transactions/sinceid")))
            .and(query_param("id", "6789")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(body))
    .expect(1)
    .mount(&server)
    .await;

    let range = client
        .transactions_id_range(
            ACCOUNT_ID,
            TransactionId::from("6790"),
            TransactionId::from("6791"),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(range.transactions.len(), 2);
    // Unknown transaction types are preserved, not dropped.
    assert!(matches!(range.transactions[1], Transaction::Unknown(_)));

    let since = client
        .transactions_since_id(ACCOUNT_ID, TransactionId::from("6789"))
        .await
        .unwrap();
    assert_eq!(since.last_transaction_id.unwrap().as_str(), "6791");
}
