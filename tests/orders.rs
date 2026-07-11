//! Endpoint tests for the orders domain.

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::Error;
use oanda_rs::endpoints::orders::CreateOrderRejectBody;
use oanda_rs::models::transaction::Transaction;
use oanda_rs::models::{
    ClientExtensions, LimitOrderRequest, MarketOrderRequest, Order, OrderId, OrderSpecifier,
    OrderStateFilter, StopLossDetails, TakeProfitDetails,
};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn create_market_order_pins_body_and_reads_location() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("POST"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/orders")))
            .and(body_json(json!({
                "order": {
                    "type": "MARKET",
                    "instrument": "EUR_USD",
                    "units": "100",
                    "takeProfitOnFill": {"price": "1.1050"},
                    "stopLossOnFill": {"distance": "0.0050"}
                }
            }))),
    )
    .respond_with(
        ResponseTemplate::new(201)
            .insert_header("Location", "/v3/accounts/x/orders/6790")
            .set_body_json(json!({
                "orderCreateTransaction": {
                    "type": "MARKET_ORDER",
                    "id": "6789",
                    "accountID": ACCOUNT_ID,
                    "instrument": "EUR_USD",
                    "units": "100",
                    "timeInForce": "FOK",
                    "reason": "CLIENT_ORDER"
                },
                "orderFillTransaction": {
                    "type": "ORDER_FILL",
                    "id": "6790",
                    "orderID": "6789",
                    "instrument": "EUR_USD",
                    "units": "100",
                    "price": "1.10423"
                },
                "relatedTransactionIDs": ["6789", "6790"],
                "lastTransactionID": "6790"
            })),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .create_order(
            ACCOUNT_ID,
            MarketOrderRequest::new("EUR_USD", 100)
                .take_profit_on_fill(TakeProfitDetails::at_price("1.1050".parse().unwrap()))
                .stop_loss_on_fill(StopLossDetails::at_distance("0.0050".parse().unwrap())),
        )
        .await
        .unwrap();

    assert!(matches!(
        response.order_create_transaction,
        Transaction::MarketOrder(_)
    ));
    let fill = response.order_fill_transaction.unwrap();
    assert_eq!(fill.price.unwrap().to_string(), "1.10423");
    assert_eq!(response.related_transaction_ids.len(), 2);
    assert_eq!(
        response.location.as_deref(),
        Some("/v3/accounts/x/orders/6790")
    );
}

#[tokio::test]
async fn create_order_reject_carries_typed_details() {
    let (server, client) = mock_client().await;
    Mock::given(method("POST"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/orders")))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "orderRejectTransaction": {
                "type": "MARKET_ORDER_REJECT",
                "id": "6791",
                "instrument": "EUR_USD",
                "units": "100",
                "rejectReason": "INSTRUMENT_NOT_TRADEABLE"
            },
            "relatedTransactionIDs": ["6791"],
            "lastTransactionID": "6791",
            "errorCode": "INSTRUMENT_NOT_TRADEABLE",
            "errorMessage": "The instrument specified is not tradeable"
        })))
        .mount(&server)
        .await;

    let error = client
        .create_order(ACCOUNT_ID, MarketOrderRequest::new("EUR_USD", 100))
        .await
        .unwrap_err();
    let Error::Api { status, body, .. } = &error else {
        panic!("expected Error::Api, got {error:?}");
    };
    assert_eq!(status.as_u16(), 400);
    let details: CreateOrderRejectBody = body.details().unwrap();
    let Some(Transaction::MarketOrderReject(reject)) = details.order_reject_transaction else {
        panic!("expected MarketOrderReject");
    };
    assert_eq!(
        reject.reject_reason.unwrap().as_str(),
        "INSTRUMENT_NOT_TRADEABLE"
    );
}

#[tokio::test]
async fn list_orders_with_filters() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/orders")))
            .and(query_param("state", "PENDING"))
            .and(query_param("instrument", "EUR_USD"))
            .and(query_param("count", "10")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orders": [{
            "type": "LIMIT",
            "id": "6789",
            "createTime": "2024-06-14T12:00:00.000000000Z",
            "state": "PENDING",
            "instrument": "EUR_USD",
            "units": "100",
            "price": "1.0900",
            "timeInForce": "GTC"
        }],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .list_orders(ACCOUNT_ID)
        .state(OrderStateFilter::Pending)
        .instrument("EUR_USD")
        .count(10)
        .send()
        .await
        .unwrap();
    assert_eq!(response.orders.len(), 1);
    let Order::Limit(limit) = &response.orders[0] else {
        panic!("expected limit order");
    };
    assert_eq!(limit.price.unwrap().to_string(), "1.0900");
}

#[tokio::test]
async fn pending_orders_and_get_order() {
    let (server, client) = mock_client().await;
    let order_body = json!({
        "order": {
            "type": "TAKE_PROFIT",
            "id": "6789",
            "state": "PENDING",
            "tradeID": "6543",
            "price": "1.1200"
        },
        "lastTransactionID": "6790"
    });
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/pendingOrders"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orders": [],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/orders/@my-order"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(order_body))
    .expect(1)
    .mount(&server)
    .await;

    let pending = client.list_pending_orders(ACCOUNT_ID).await.unwrap();
    assert!(pending.orders.is_empty());

    let got = client
        .order(ACCOUNT_ID, OrderSpecifier::from_client_id("my-order"))
        .await
        .unwrap();
    assert!(matches!(got.order, Order::TakeProfit(_)));
}

#[tokio::test]
async fn replace_and_cancel_order() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/orders/6789")))
            .and(body_json(json!({
                "order": {"type": "LIMIT", "instrument": "EUR_USD", "units": "200", "price": "1.0800"}
            }))),
    )
    .respond_with(ResponseTemplate::new(201).set_body_json(json!({
        "orderCancelTransaction": {"type": "ORDER_CANCEL", "id": "6800", "orderID": "6789", "reason": "CLIENT_REQUEST_REPLACED"},
        "orderCreateTransaction": {"type": "LIMIT_ORDER", "id": "6801", "instrument": "EUR_USD", "units": "200", "price": "1.0800"},
        "relatedTransactionIDs": ["6800", "6801"],
        "lastTransactionID": "6801"
    })))
    .expect(1)
    .mount(&server)
    .await;
    standard_headers(Mock::given(method("PUT")).and(path(format!(
        "/accounts/{ACCOUNT_ID}/orders/6801/cancel"
    ))))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orderCancelTransaction": {"type": "ORDER_CANCEL", "id": "6802", "orderID": "6801", "reason": "CLIENT_REQUEST"},
        "relatedTransactionIDs": ["6802"],
        "lastTransactionID": "6802"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let replaced = client
        .replace_order(
            ACCOUNT_ID,
            OrderId::from("6789"),
            LimitOrderRequest::new(
                "EUR_USD",
                200,
                "1.0800".parse::<oanda_rs::models::PriceValue>().unwrap(),
            ),
        )
        .await
        .unwrap();
    assert!(replaced.order_cancel_transaction.is_some());

    let cancelled = client
        .cancel_order(ACCOUNT_ID, OrderId::from("6801"))
        .await
        .unwrap();
    assert_eq!(
        cancelled
            .order_cancel_transaction
            .unwrap()
            .order_id
            .unwrap()
            .as_str(),
        "6801"
    );
}

#[tokio::test]
async fn set_order_client_extensions() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!(
                "/accounts/{ACCOUNT_ID}/orders/6789/clientExtensions"
            )))
            .and(body_json(json!({
                "clientExtensions": {"id": "my-id", "tag": "strategy-9"}
            }))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orderClientExtensionsModifyTransaction": {
            "type": "ORDER_CLIENT_EXTENSIONS_MODIFY",
            "id": "6803",
            "orderID": "6789"
        },
        "lastTransactionID": "6803"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .set_order_client_extensions(ACCOUNT_ID, OrderId::from("6789"))
        .client_extensions(ClientExtensions::new().id("my-id").tag("strategy-9"))
        .send()
        .await
        .unwrap();
    assert!(
        response
            .order_client_extensions_modify_transaction
            .is_some()
    );
}

#[tokio::test]
async fn list_orders_ids_and_before_id() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/orders")))
            .and(query_param("ids", "1,2"))
            .and(query_param("beforeID", "100")),
    )
    .respond_with(
        ResponseTemplate::new(200).set_body_json(json!({"orders": [], "lastTransactionID": "100"})),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .list_orders(ACCOUNT_ID)
        .ids([OrderId::from("1"), OrderId::from("2")])
        .before_id(OrderId::from("100"))
        .send()
        .await
        .unwrap();
    assert!(response.orders.is_empty());
}

#[tokio::test]
async fn cancel_order_reject_carries_typed_details() {
    let (server, client) = mock_client().await;
    Mock::given(method("PUT"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/orders/9999/cancel")))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "orderCancelRejectTransaction": {
                "type": "ORDER_CANCEL_REJECT",
                "id": "7100",
                "orderID": "9999",
                "rejectReason": "ORDER_DOESNT_EXIST"
            },
            "relatedTransactionIDs": ["7100"],
            "lastTransactionID": "7100",
            "errorCode": "ORDER_DOESNT_EXIST",
            "errorMessage": "The order does not exist"
        })))
        .mount(&server)
        .await;

    let error = client
        .cancel_order(ACCOUNT_ID, OrderId::from("9999"))
        .await
        .unwrap_err();
    let Error::Api { body, .. } = &error else {
        panic!("expected Error::Api");
    };
    let details: oanda_rs::endpoints::orders::CancelOrderRejectBody = body.details().unwrap();
    let reject = details.order_cancel_reject_transaction.unwrap();
    assert_eq!(reject.order_id.unwrap().as_str(), "9999");
    assert_eq!(details.last_transaction_id.unwrap().as_str(), "7100");
}

#[tokio::test]
async fn set_order_trade_client_extensions() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!(
                "/accounts/{ACCOUNT_ID}/orders/6789/clientExtensions"
            )))
            .and(body_json(json!({
                "tradeClientExtensions": {"comment": "on-fill"}
            }))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orderClientExtensionsModifyTransaction": {
            "type": "ORDER_CLIENT_EXTENSIONS_MODIFY",
            "id": "6804",
            "orderID": "6789"
        },
        "lastTransactionID": "6804"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .set_order_client_extensions(ACCOUNT_ID, OrderId::from("6789"))
        .trade_client_extensions(ClientExtensions::new().comment("on-fill"))
        .send()
        .await
        .unwrap();
    assert!(
        response
            .order_client_extensions_modify_transaction
            .is_some()
    );
}
