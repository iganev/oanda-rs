//! Internal serde helpers for OANDA's wire-format quirks.

use std::fmt;

use rust_decimal::Decimal;
use serde::de::{self, Deserializer, Visitor};

/// Deserializes a [`Decimal`] from either a JSON string or a JSON number.
///
/// OANDA encodes decimal values as strings in REST responses, but some
/// streaming payloads (e.g. `PriceBucket.liquidity`) arrive as plain JSON
/// numbers. This visitor accepts both.
pub(crate) fn decimal_from_str_or_number<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    struct DecimalVisitor;

    impl Visitor<'_> for DecimalVisitor {
        type Value = Decimal;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a decimal encoded as a string or number")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Decimal, E> {
            v.parse::<Decimal>()
                .or_else(|_| Decimal::from_scientific(v))
                .map_err(|_| E::invalid_value(de::Unexpected::Str(v), &self))
        }

        fn visit_f64<E: de::Error>(self, v: f64) -> Result<Decimal, E> {
            Decimal::try_from(v).map_err(|_| E::invalid_value(de::Unexpected::Float(v), &self))
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Decimal, E> {
            Ok(Decimal::from(v))
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Decimal, E> {
            Ok(Decimal::from(v))
        }
    }

    deserializer.deserialize_any(DecimalVisitor)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Holder {
        #[serde(deserialize_with = "super::decimal_from_str_or_number")]
        value: rust_decimal::Decimal,
    }

    #[test]
    fn accepts_string() {
        let h: Holder = serde_json::from_str(r#"{"value":"1.23456"}"#).unwrap();
        assert_eq!(h.value.to_string(), "1.23456");
    }

    #[test]
    fn accepts_integer_number() {
        let h: Holder = serde_json::from_str(r#"{"value":1000000}"#).unwrap();
        assert_eq!(h.value.to_string(), "1000000");
    }

    #[test]
    fn accepts_float_number() {
        let h: Holder = serde_json::from_str(r#"{"value":0.25}"#).unwrap();
        assert_eq!(h.value.to_string(), "0.25");
    }

    #[test]
    fn accepts_scientific_string() {
        let h: Holder = serde_json::from_str(r#"{"value":"1e-5"}"#).unwrap();
        assert_eq!(h.value.to_string(), "0.00001");
    }

    #[test]
    fn accepts_negative_string() {
        let h: Holder = serde_json::from_str(r#"{"value":"-0.0075"}"#).unwrap();
        assert_eq!(h.value.to_string(), "-0.0075");
    }

    #[test]
    fn rejects_garbage() {
        assert!(serde_json::from_str::<Holder>(r#"{"value":"abc"}"#).is_err());
        assert!(serde_json::from_str::<Holder>(r#"{"value":true}"#).is_err());
    }
}
