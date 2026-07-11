//! Endpoint tests for the instruments domain (candles and books).

mod common;

use common::{ACCOUNT_ID, mock_client, standard_headers};
use oanda_rs::models::{
    CandleSpecification, CandlestickGranularity, InstrumentName, PricingComponent,
};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

fn candles_body() -> serde_json::Value {
    json!({
        "instrument": "EUR_USD",
        "granularity": "H1",
        "candles": [
            {
                "time": "2024-06-14T12:00:00.000000000Z",
                "mid": {"o": "1.07132", "h": "1.07240", "l": "1.07106", "c": "1.07223"},
                "volume": 4713,
                "complete": true
            },
            {
                "time": "2024-06-14T13:00:00.000000000Z",
                "bid": {"o": "1.07215", "h": "1.07220", "l": "1.07158", "c": "1.07198"},
                "ask": {"o": "1.07230", "h": "1.07236", "l": "1.07172", "c": "1.07213"},
                "volume": 3892,
                "complete": false
            }
        ]
    })
}

#[tokio::test]
async fn instrument_candles_with_params() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path("/instruments/EUR_USD/candles"))
            .and(query_param("granularity", "H1"))
            .and(query_param("count", "2"))
            .and(query_param("price", "BMA"))
            .and(query_param("smooth", "false"))
            .and(query_param("weeklyAlignment", "Friday")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(candles_body()))
    .expect(1)
    .mount(&server)
    .await;

    let candles = client
        .candles("EUR_USD")
        .granularity(CandlestickGranularity::H1)
        .count(2)
        .price(PricingComponent::BID.with_mid().with_ask())
        .smooth(false)
        .weekly_alignment(oanda_rs::models::WeeklyAlignment::Friday)
        .send()
        .await
        .unwrap();

    assert_eq!(candles.instrument, Some(InstrumentName::EurUsd));
    assert_eq!(candles.granularity, Some(CandlestickGranularity::H1));
    assert_eq!(candles.candles.len(), 2);
    let first = &candles.candles[0];
    assert_eq!(first.complete, Some(true));
    assert_eq!(first.volume, Some(4713));
    assert_eq!(
        first.mid.as_ref().unwrap().o.unwrap().to_string(),
        "1.07132"
    );
    assert!(candles.candles[1].bid.is_some());
}

#[tokio::test]
async fn account_scoped_candles_with_units() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!(
                "/accounts/{ACCOUNT_ID}/instruments/EUR_USD/candles"
            )))
            .and(query_param("units", "1000")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(candles_body()))
    .expect(1)
    .mount(&server)
    .await;

    let candles = client
        .account_candles(ACCOUNT_ID, InstrumentName::EurUsd)
        .units(1000)
        .send()
        .await
        .unwrap();
    assert_eq!(candles.candles.len(), 2);
}

#[tokio::test]
async fn latest_candles_specifications() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/candles/latest")))
            .and(query_param(
                "candleSpecifications",
                "EUR_USD:S10:BM,XAU_USD:M1",
            )),
    )
    .respond_with(
        ResponseTemplate::new(200).set_body_json(json!({"latestCandles": [candles_body()]})),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .latest_candles(
            ACCOUNT_ID,
            [
                CandleSpecification::new("EUR_USD", CandlestickGranularity::S10)
                    .price(PricingComponent::BID.with_mid()),
                CandleSpecification::new(InstrumentName::XauUsd, CandlestickGranularity::M1),
            ],
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.latest_candles.len(), 1);
}

#[tokio::test]
async fn order_book_surfaces_link_header() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path("/instruments/EUR_USD/orderBook"))
            .and(query_param("time", "2024-06-14T12:00:00Z")),
    )
    .respond_with(
        ResponseTemplate::new(200)
            .insert_header(
                "Link",
                "<https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/orderBook?time=123>; rel=\"next\"",
            )
            .set_body_json(json!({
                "orderBook": {
                    "instrument": "EUR_USD",
                    "time": "2024-06-14T12:00:00Z",
                    "unixTime": "1718366400",
                    "price": "1.07132",
                    "bucketWidth": "0.0005",
                    "buckets": [
                        {"price": "1.07100", "longCountPercent": "0.2422", "shortCountPercent": "0.1902"}
                    ]
                }
            })),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .instrument_order_book("EUR_USD")
        .time("2024-06-14T12:00:00Z")
        .send()
        .await
        .unwrap();
    assert_eq!(response.order_book.buckets.len(), 1);
    assert_eq!(
        response.order_book.buckets[0]
            .long_count_percent
            .unwrap()
            .to_string(),
        "0.2422"
    );
    assert!(response.link.as_deref().unwrap().contains("rel=\"next\""));
}

#[tokio::test]
async fn position_book() {
    let (server, client) = mock_client().await;
    standard_headers(Mock::given(method("GET")).and(path("/instruments/EUR_USD/positionBook")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "positionBook": {
                "instrument": "EUR_USD",
                "time": "2024-06-14T12:00:00Z",
                "price": "1.07132",
                "bucketWidth": "0.0005",
                "buckets": [
                    {"price": "1.07100", "longCountPercent": "55.1", "shortCountPercent": "44.9"}
                ]
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let response = client
        .instrument_position_book(InstrumentName::EurUsd)
        .send()
        .await
        .unwrap();
    assert_eq!(response.position_book.buckets.len(), 1);
    assert!(response.link.is_none());
}

#[tokio::test]
async fn candles_remaining_query_params() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path("/instruments/EUR_USD/candles"))
            .and(query_param("from", "2024-06-01T00:00:00Z"))
            .and(query_param("to", "2024-06-14T00:00:00Z"))
            .and(query_param("includeFirst", "true"))
            .and(query_param("dailyAlignment", "17"))
            .and(query_param("alignmentTimezone", "America/New_York")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(candles_body()))
    .expect(1)
    .mount(&server)
    .await;

    client
        .candles("EUR_USD")
        .from("2024-06-01T00:00:00Z")
        .to("2024-06-14T00:00:00Z")
        .include_first(true)
        .daily_alignment(17)
        .alignment_timezone("America/New_York")
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn latest_candles_remaining_query_params() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path(format!("/accounts/{ACCOUNT_ID}/candles/latest")))
            .and(query_param("units", "500"))
            .and(query_param("smooth", "true"))
            .and(query_param("dailyAlignment", "6"))
            .and(query_param("alignmentTimezone", "Europe/Sofia"))
            .and(query_param("weeklyAlignment", "Monday")),
    )
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({"latestCandles": []})))
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .latest_candles(
            ACCOUNT_ID,
            [CandleSpecification::new(
                "EUR_USD",
                CandlestickGranularity::M1,
            )],
        )
        .units(500)
        .smooth(true)
        .daily_alignment(6)
        .alignment_timezone("Europe/Sofia")
        .weekly_alignment(oanda_rs::models::WeeklyAlignment::Monday)
        .send()
        .await
        .unwrap();
    assert!(response.latest_candles.is_empty());
}

#[tokio::test]
async fn position_book_with_time_param() {
    let (server, client) = mock_client().await;
    standard_headers(
        Mock::given(method("GET"))
            .and(path("/instruments/EUR_USD/positionBook"))
            .and(query_param("time", "2024-06-14T12:00:00Z")),
    )
    .respond_with(
        ResponseTemplate::new(200)
            .insert_header("Link", "<next>; rel=\"next\"")
            .set_body_json(json!({
                "positionBook": {
                    "instrument": "EUR_USD",
                    "price": "1.07132",
                    "bucketWidth": "0.0005",
                    "buckets": []
                }
            })),
    )
    .expect(1)
    .mount(&server)
    .await;

    let response = client
        .instrument_position_book("EUR_USD")
        .time("2024-06-14T12:00:00Z")
        .send()
        .await
        .unwrap();
    assert_eq!(response.link.as_deref(), Some("<next>; rel=\"next\""));
}
