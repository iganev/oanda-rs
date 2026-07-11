//! Endpoint tests for the trades domain.

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::models::{
    ClientExtensions, StopLossDetails, TakeProfitDetails, TradeId, TradeStateFilter,
};
use serde_json::json;
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn trade_body() -> serde_json::Value {
    json!({
        "id": "6543",
        "instrument": "EUR_USD",
        "price": "1.10423",
        "openTime": "2024-06-14T12:00:00.000000000Z",
        "state": "OPEN",
        "initialUnits": "100",
        "currentUnits": "100",
        "realizedPL": "0.0000",
        "unrealizedPL": "0.0521",
        "financing": "0.0000"
    })
}

#[tokio::test]
async fn list_trades_with_filters() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/trades")))
            .and(query_param("state", "CLOSED"))
            .and(query_param("ids", "6543,6544"))
            .and(query_param("count", "5")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "trades": [trade_body()],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .list_trades(ACCOUNT_ID)
        .state(TradeStateFilter::Closed)
        .ids([TradeId::from("6543"), TradeId::from("6544")])
        .count(5)
        .send()
        .await
        .unwrap();
    assert_eq!(response.trades.len(), 1);
    assert_eq!(response.trades[0].current_units.unwrap().to_string(), "100");
}

#[tokio::test]
async fn open_trades_and_get_trade() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/openTrades"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "trades": [trade_body()],
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;
    standard_headers(
        Mock::given(method("GET")).and(path(format!("/accounts/{ACCOUNT_ID}/trades/6543"))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "trade": trade_body(),
        "lastTransactionID": "6790"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let open = client.list_open_trades(ACCOUNT_ID).await.unwrap();
    assert_eq!(open.trades.len(), 1);

    let single = client
        .trade(ACCOUNT_ID, TradeId::from("6543"))
        .await
        .unwrap();
    assert_eq!(single.trade.id.unwrap().as_str(), "6543");
}

#[tokio::test]
async fn close_trade_partial_units() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/trades/6543/close")))
            .and(body_json(json!({"units": "50"}))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "orderCreateTransaction": {
            "type": "MARKET_ORDER",
            "id": "6900",
            "instrument": "EUR_USD",
            "units": "-50",
            "tradeClose": {"tradeID": "6543", "units": "50"}
        },
        "orderFillTransaction": {
            "type": "ORDER_FILL",
            "id": "6901",
            "orderID": "6900",
            "pl": "0.0260"
        },
        "relatedTransactionIDs": ["6900", "6901"],
        "lastTransactionID": "6901"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .close_trade(ACCOUNT_ID, TradeId::from("6543"))
        .units("50")
        .send()
        .await
        .unwrap();
    assert_eq!(
        response
            .order_fill_transaction
            .unwrap()
            .pl
            .unwrap()
            .to_string(),
        "0.0260"
    );
}

#[tokio::test]
async fn set_trade_client_extensions() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!(
                "/accounts/{ACCOUNT_ID}/trades/6543/clientExtensions"
            )))
            .and(body_json(json!({"clientExtensions": {"id": "my-trade"}}))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "tradeClientExtensionsModifyTransaction": {
            "type": "TRADE_CLIENT_EXTENSIONS_MODIFY",
            "id": "6902",
            "tradeID": "6543"
        },
        "relatedTransactionIDs": ["6902"],
        "lastTransactionID": "6902"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .set_trade_client_extensions(
            ACCOUNT_ID,
            TradeId::from("6543"),
            ClientExtensions::new().id("my-trade"),
        )
        .await
        .unwrap();
    assert!(
        response
            .trade_client_extensions_modify_transaction
            .is_some()
    );
}

#[tokio::test]
async fn set_trade_dependent_orders_with_cancel() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/trades/6543/orders")))
            .and(body_json(json!({
                "takeProfit": {"price": "1.1200"},
                "stopLoss": {"distance": "0.0050"},
                "trailingStopLoss": null
            }))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "takeProfitOrderTransaction": {
            "type": "TAKE_PROFIT_ORDER",
            "id": "6903",
            "tradeID": "6543",
            "price": "1.1200"
        },
        "stopLossOrderTransaction": {
            "type": "STOP_LOSS_ORDER",
            "id": "6904",
            "tradeID": "6543",
            "distance": "0.0050"
        },
        "relatedTransactionIDs": ["6903", "6904"],
        "lastTransactionID": "6904"
    })))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .set_trade_dependent_orders(ACCOUNT_ID, TradeId::from("6543"))
        .take_profit(TakeProfitDetails::at_price("1.1200".parse().unwrap()))
        .stop_loss(StopLossDetails::at_distance("0.0050".parse().unwrap()))
        .cancel_trailing_stop_loss()
        .send()
        .await
        .unwrap();
    assert!(response.take_profit_order_transaction.is_some());
    assert!(response.stop_loss_order_transaction.is_some());
}

#[tokio::test]
async fn list_trades_instrument_and_before_id() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/trades")))
            .and(query_param("instrument", "EUR_USD"))
            .and(query_param("beforeID", "7000")),
    )
    .respond_with(
        ResponseTemplate::new(200)
            .set_body_json(json!({"trades": [], "lastTransactionID": "7000"})),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .list_trades(ACCOUNT_ID)
        .instrument("EUR_USD")
        .before_id(TradeId::from("7000"))
        .send()
        .await
        .unwrap();
    assert!(response.trades.is_empty());
}

#[tokio::test]
async fn close_trade_reject_carries_typed_details() {
    let (server, client) = mock_client().await;
    Mock::given(method("PUT"))
        .and(path(format!("/accounts/{ACCOUNT_ID}/trades/6543/close")))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "orderRejectTransaction": {
                "type": "MARKET_ORDER_REJECT",
                "id": "7200",
                "instrument": "EUR_USD",
                "rejectReason": "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE"
            },
            "lastTransactionID": "7199",
            "errorCode": "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE",
            "errorMessage": "units exceed trade size"
        })))
        .mount(&server)
        .await;

    let error = client
        .close_trade(ACCOUNT_ID, TradeId::from("6543"))
        .units("99999")
        .send()
        .await
        .unwrap_err();
    let oanda_rs::Error::Api { body, .. } = &error else {
        panic!("expected Error::Api");
    };
    let details: oanda_rs::endpoints::trades::CloseTradeRejectBody = body.details().unwrap();
    assert!(details.order_reject_transaction.is_some());
}

#[tokio::test]
async fn set_trade_dependent_orders_cancel_and_trailing() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("PUT"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/trades/6543/orders")))
            .and(body_json(json!({
                "takeProfit": null,
                "stopLoss": null,
                "trailingStopLoss": {"distance": "0.0080"}
            }))),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({
        "trailingStopLossOrderTransaction": {
            "type": "TRAILING_STOP_LOSS_ORDER",
            "id": "6905",
            "tradeID": "6543",
            "distance": "0.0080"
        },
        "lastTransactionID": "6905"
    })))
    .expect(1)
    .mount(&server)
    .await;

    use oanda_rs::models::TrailingStopLossDetails;
    let response = client
        .set_trade_dependent_orders(ACCOUNT_ID, TradeId::from("6543"))
        .cancel_take_profit()
        .cancel_stop_loss()
        .trailing_stop_loss(TrailingStopLossDetails::at_distance(
            "0.0080".parse().unwrap(),
        ))
        .send()
        .await
        .unwrap();
    assert!(response.trailing_stop_loss_order_transaction.is_some());
}
