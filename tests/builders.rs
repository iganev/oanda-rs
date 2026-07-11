//! Exhaustive construction tests for the order-request builders: every
//! setter of every request type is exercised and the serialized JSON is
//! asserted field-by-field.

use oanda_rs::models::{
    ClientExtensions, DecimalNumber, LimitOrderRequest, LimitOrderTimeInForce,
    MarketIfTouchedOrderRequest, MarketIfTouchedOrderTimeInForce, MarketOrderRequest,
    MarketOrderTimeInForce, Order, OrderPositionFill, OrderRequest, OrderTriggerCondition,
    PriceValue, StopLossDetails, StopLossOrderRequest, StopLossOrderTimeInForce, StopOrderRequest,
    StopOrderTimeInForce, TakeProfitDetails, TakeProfitOrderRequest, TakeProfitOrderTimeInForce,
    TrailingStopLossDetails, TrailingStopLossOrderRequest, TrailingStopLossOrderTimeInForce,
};
use serde_json::json;

fn extensions() -> ClientExtensions {
    ClientExtensions::new()
        .id("my-id")
        .tag("my-tag")
        .comment("my-comment")
}

fn extensions_json() -> serde_json::Value {
    json!({"id": "my-id", "tag": "my-tag", "comment": "my-comment"})
}

#[test]
fn market_order_request_all_setters() {
    let request: OrderRequest = MarketOrderRequest::new("EUR_USD", 100)
        .time_in_force(MarketOrderTimeInForce::Ioc)
        .price_bound("1.1000".parse::<PriceValue>().unwrap())
        .position_fill(OrderPositionFill::ReduceFirst)
        .client_extensions(extensions())
        .take_profit_on_fill(TakeProfitDetails::at_price("1.1200".parse().unwrap()))
        .stop_loss_on_fill(StopLossDetails::at_price("1.0900".parse().unwrap()))
        .trailing_stop_loss_on_fill(TrailingStopLossDetails::at_distance(
            "0.0050".parse().unwrap(),
        ))
        .trade_client_extensions(extensions())
        .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "MARKET",
            "instrument": "EUR_USD",
            "units": "100",
            "timeInForce": "IOC",
            "priceBound": "1.1000",
            "positionFill": "REDUCE_FIRST",
            "clientExtensions": extensions_json(),
            "takeProfitOnFill": {"price": "1.1200"},
            "stopLossOnFill": {"price": "1.0900"},
            "trailingStopLossOnFill": {"distance": "0.0050"},
            "tradeClientExtensions": extensions_json(),
        })
    );
}

#[test]
fn limit_order_request_all_setters() {
    let request: OrderRequest =
        LimitOrderRequest::new("EUR_USD", -50, "1.0800".parse::<PriceValue>().unwrap())
            .time_in_force(LimitOrderTimeInForce::Gtd)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .position_fill(OrderPositionFill::OpenOnly)
            .trigger_condition(OrderTriggerCondition::Bid)
            .client_extensions(extensions())
            .take_profit_on_fill(TakeProfitDetails::at_price("1.0500".parse().unwrap()))
            .stop_loss_on_fill(StopLossDetails::at_distance("0.0100".parse().unwrap()))
            .trailing_stop_loss_on_fill(TrailingStopLossDetails::at_distance(
                "0.0075".parse().unwrap(),
            ))
            .trade_client_extensions(extensions())
            .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "LIMIT",
            "instrument": "EUR_USD",
            "units": "-50",
            "price": "1.0800",
            "timeInForce": "GTD",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "positionFill": "OPEN_ONLY",
            "triggerCondition": "BID",
            "clientExtensions": extensions_json(),
            "takeProfitOnFill": {"price": "1.0500"},
            "stopLossOnFill": {"distance": "0.0100"},
            "trailingStopLossOnFill": {"distance": "0.0075"},
            "tradeClientExtensions": extensions_json(),
        })
    );
}

#[test]
fn stop_order_request_all_setters() {
    let request: OrderRequest =
        StopOrderRequest::new("USD_JPY", 1000, "157.500".parse::<PriceValue>().unwrap())
            .price_bound("157.800".parse::<PriceValue>().unwrap())
            .time_in_force(StopOrderTimeInForce::Gfd)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .position_fill(OrderPositionFill::Default)
            .trigger_condition(OrderTriggerCondition::Mid)
            .client_extensions(extensions())
            .take_profit_on_fill(TakeProfitDetails::at_price("160.000".parse().unwrap()))
            .stop_loss_on_fill(StopLossDetails::at_price("155.000".parse().unwrap()))
            .trailing_stop_loss_on_fill(TrailingStopLossDetails::at_distance(
                "0.500".parse().unwrap(),
            ))
            .trade_client_extensions(extensions())
            .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "STOP",
            "instrument": "USD_JPY",
            "units": "1000",
            "price": "157.500",
            "priceBound": "157.800",
            "timeInForce": "GFD",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "positionFill": "DEFAULT",
            "triggerCondition": "MID",
            "clientExtensions": extensions_json(),
            "takeProfitOnFill": {"price": "160.000"},
            "stopLossOnFill": {"price": "155.000"},
            "trailingStopLossOnFill": {"distance": "0.500"},
            "tradeClientExtensions": extensions_json(),
        })
    );
}

#[test]
fn market_if_touched_order_request_all_setters() {
    let request: OrderRequest =
        MarketIfTouchedOrderRequest::new("XAU_USD", 5, "2300.00".parse::<PriceValue>().unwrap())
            .price_bound("2310.00".parse::<PriceValue>().unwrap())
            .time_in_force(MarketIfTouchedOrderTimeInForce::Gtc)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .position_fill(OrderPositionFill::ReduceOnly)
            .trigger_condition(OrderTriggerCondition::Ask)
            .client_extensions(extensions())
            .take_profit_on_fill(TakeProfitDetails::at_price("2400.00".parse().unwrap()))
            .stop_loss_on_fill(StopLossDetails::at_distance("25.00".parse().unwrap()))
            .trailing_stop_loss_on_fill(TrailingStopLossDetails::at_distance(
                "30.00".parse().unwrap(),
            ))
            .trade_client_extensions(extensions())
            .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "MARKET_IF_TOUCHED",
            "instrument": "XAU_USD",
            "units": "5",
            "price": "2300.00",
            "priceBound": "2310.00",
            "timeInForce": "GTC",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "positionFill": "REDUCE_ONLY",
            "triggerCondition": "ASK",
            "clientExtensions": extensions_json(),
            "takeProfitOnFill": {"price": "2400.00"},
            "stopLossOnFill": {"distance": "25.00"},
            "trailingStopLossOnFill": {"distance": "30.00"},
            "tradeClientExtensions": extensions_json(),
        })
    );
}

#[test]
fn take_profit_order_request_all_setters() {
    let request: OrderRequest =
        TakeProfitOrderRequest::new("6543", "1.1200".parse::<PriceValue>().unwrap())
            .client_trade_id("my-trade")
            .time_in_force(TakeProfitOrderTimeInForce::Gtd)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .trigger_condition(OrderTriggerCondition::Default)
            .client_extensions(extensions())
            .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "TAKE_PROFIT",
            "tradeID": "6543",
            "clientTradeID": "my-trade",
            "price": "1.1200",
            "timeInForce": "GTD",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "triggerCondition": "DEFAULT",
            "clientExtensions": extensions_json(),
        })
    );
}

#[test]
fn stop_loss_order_request_price_and_distance_forms() {
    let at_price: OrderRequest =
        StopLossOrderRequest::at_price("6543", "1.0900".parse::<PriceValue>().unwrap())
            .client_trade_id("my-trade")
            .time_in_force(StopLossOrderTimeInForce::Gtc)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .trigger_condition(OrderTriggerCondition::Inverse)
            .client_extensions(extensions())
            .into();
    assert_eq!(
        serde_json::to_value(&at_price).unwrap(),
        json!({
            "type": "STOP_LOSS",
            "tradeID": "6543",
            "clientTradeID": "my-trade",
            "price": "1.0900",
            "timeInForce": "GTC",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "triggerCondition": "INVERSE",
            "clientExtensions": extensions_json(),
        })
    );

    let at_distance: OrderRequest =
        StopLossOrderRequest::at_distance("6543", "0.0050".parse::<DecimalNumber>().unwrap())
            .into();
    assert_eq!(
        serde_json::to_value(&at_distance).unwrap(),
        json!({"type": "STOP_LOSS", "tradeID": "6543", "distance": "0.0050"})
    );
}

#[test]
fn trailing_stop_loss_order_request_all_setters() {
    let request: OrderRequest =
        TrailingStopLossOrderRequest::new("6543", "0.0075".parse::<DecimalNumber>().unwrap())
            .client_trade_id("my-trade")
            .time_in_force(TrailingStopLossOrderTimeInForce::Gfd)
            .gtd_time("2026-12-31T23:59:59.000000000Z")
            .trigger_condition(OrderTriggerCondition::Bid)
            .client_extensions(extensions())
            .into();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "type": "TRAILING_STOP_LOSS",
            "tradeID": "6543",
            "clientTradeID": "my-trade",
            "distance": "0.0075",
            "timeInForce": "GFD",
            "gtdTime": "2026-12-31T23:59:59.000000000Z",
            "triggerCondition": "BID",
            "clientExtensions": extensions_json(),
        })
    );
}

#[test]
fn order_request_roundtrips_through_deserialize() {
    // OrderRequest also derives Deserialize; a serialized request must come
    // back as the same variant.
    let original: OrderRequest = MarketOrderRequest::new("EUR_USD", 100).into();
    let json = serde_json::to_string(&original).unwrap();
    let back: OrderRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(back, original);
}

#[test]
fn client_extensions_default_is_empty() {
    let extensions = ClientExtensions::default();
    assert_eq!(serde_json::to_value(&extensions).unwrap(), json!({}));
}

#[test]
fn order_enum_accessors_cover_all_fields() {
    let order: Order = serde_json::from_value(json!({
        "type": "LIMIT",
        "id": "42",
        "createTime": "2024-06-14T12:00:00.000000000Z",
        "state": "PENDING",
        "instrument": "EUR_USD",
        "units": "100",
        "price": "1.0800",
        "clientExtensions": {"id": "my-id"}
    }))
    .unwrap();
    assert_eq!(order.id().unwrap().as_str(), "42");
    assert_eq!(
        order.create_time().unwrap().as_str(),
        "2024-06-14T12:00:00.000000000Z"
    );
    assert_eq!(order.state(), Some(&oanda_rs::models::OrderState::Pending));
    assert_eq!(
        order.client_extensions().unwrap().id.as_deref(),
        Some("my-id")
    );
    assert_eq!(order.type_name(), Some("LIMIT"));

    let unknown: Order = serde_json::from_value(json!({"type": "MYSTERY"})).unwrap();
    assert_eq!(unknown.id(), None);
    assert_eq!(unknown.create_time(), None);
    assert_eq!(unknown.state(), None);
    assert_eq!(unknown.client_extensions(), None);
}
