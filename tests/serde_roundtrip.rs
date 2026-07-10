//! Serde round-trip tests over the discriminated unions.
//!
//! Each fixture file holds one maximally-populated JSON object per union
//! variant (generated from the OpenAPI schema property lists). Every object
//! must deserialize into the matching variant and re-serialize to exactly
//! the same JSON — this catches field renames, wrong types, and dropped
//! fields across all ~60 structs involved.

use oanda_rs::models::Order;
use oanda_rs::models::transaction::Transaction;
use serde_json::Value;

fn roundtrip<T>(fixtures: &str) -> Vec<(String, T)>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let objects: Vec<Value> = serde_json::from_str(fixtures).unwrap();
    assert!(!objects.is_empty());
    let mut out = Vec::new();
    for object in objects {
        let tag = object["type"].as_str().unwrap().to_owned();
        let parsed: T = serde_json::from_value(object.clone())
            .unwrap_or_else(|e| panic!("failed to deserialize {tag}: {e}"));
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert_eq!(reserialized, object, "round-trip mismatch for {tag}");
        out.push((tag, parsed));
    }
    out
}

#[test]
fn all_transaction_variants_roundtrip() {
    let parsed = roundtrip::<Transaction>(include_str!("fixtures/transactions.json"));
    assert_eq!(parsed.len(), 36);
    for (tag, tx) in &parsed {
        assert!(
            !matches!(tx, Transaction::Unknown(_)),
            "{tag} fell through to Transaction::Unknown"
        );
        assert_eq!(tx.type_name(), Some(tag.as_str()));
        assert_eq!(tx.id().unwrap().as_str(), "6789");
        assert_eq!(tx.account_id().unwrap().as_str(), "101-004-1234567-001");
        assert!(tx.time().unwrap().to_utc().is_some());
    }
}

#[test]
fn all_order_variants_roundtrip() {
    let parsed = roundtrip::<Order>(include_str!("fixtures/orders.json"));
    assert_eq!(parsed.len(), 8);
    for (tag, order) in &parsed {
        assert!(
            !matches!(order, Order::Unknown(_)),
            "{tag} fell through to Order::Unknown"
        );
        assert_eq!(order.type_name(), Some(tag.as_str()));
        assert!(order.id().is_some());
        assert!(order.state().is_some());
    }
}

#[test]
fn unknown_transaction_type_is_preserved() {
    let raw = serde_json::json!({
        "type": "SOME_FUTURE_TYPE",
        "id": "1",
        "accountID": "101-004-1234567-001",
        "novelField": {"a": 1}
    });
    let tx: Transaction = serde_json::from_value(raw.clone()).unwrap();
    match &tx {
        Transaction::Unknown(value) => assert_eq!(*value, raw),
        other => panic!("expected Unknown, got {other:?}"),
    }
    assert_eq!(tx.type_name(), Some("SOME_FUTURE_TYPE"));
    assert_eq!(tx.id(), None);
    // Unknown transactions re-serialize to their original JSON.
    assert_eq!(serde_json::to_value(&tx).unwrap(), raw);
}

#[test]
fn unknown_order_type_is_preserved() {
    let raw = serde_json::json!({"type": "GUARANTEED_STOP_LOSS", "id": "77"});
    let order: Order = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(order, Order::Unknown(_)));
    assert_eq!(order.type_name(), Some("GUARANTEED_STOP_LOSS"));
    assert_eq!(serde_json::to_value(&order).unwrap(), raw);
}
