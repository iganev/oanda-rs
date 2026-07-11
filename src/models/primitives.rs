//! Shared primitive types: IDs, decimal values, and datetimes.
//!
//! OANDA encodes almost everything as JSON strings on the wire. This module
//! provides thin newtypes so that values keep their meaning in Rust code
//! while (de)serializing exactly as the API expects.

use std::fmt;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::serde_util::decimal_from_str_or_number;

/// Declares a transparent string newtype with the usual conversions.
macro_rules! string_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            /// Returns the underlying string slice.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                $name(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                $name(value.to_owned())
            }
        }

        impl FromStr for $name {
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(s.to_owned()))
            }
        }
    };
}

string_newtype! {
    /// The string representation of an account identifier
    /// (`{siteID}-{divisionID}-{userID}-{accountNumber}`, e.g. `101-004-1234567-001`).
    AccountId
}

string_newtype! {
    /// The unique identifier of a transaction within an account.
    TransactionId
}

string_newtype! {
    /// The unique identifier of an order within an account.
    OrderId
}

string_newtype! {
    /// The unique identifier of a trade within an account.
    TradeId
}

string_newtype! {
    /// A client-provided identifier, used to refer to orders and trades by
    /// the id set through
    /// [`ClientExtensions`](crate::models::ClientExtensions).
    ClientId
}

string_newtype! {
    /// A client-provided tag attached to orders and trades.
    ClientTag
}

string_newtype! {
    /// A client-provided comment attached to orders and trades.
    ClientComment
}

string_newtype! {
    /// The identifier OANDA assigned to an API request (the `RequestID`
    /// response header).
    RequestId
}

string_newtype! {
    /// An ISO 4217 currency code (e.g. `EUR`, `USD`).
    Currency
}

string_newtype! {
    /// Identifies an order for lookup/cancel/replace operations: either the
    /// order's id (e.g. `6789`) or `@` followed by its client id
    /// (e.g. `@my-order`).
    OrderSpecifier
}

string_newtype! {
    /// Identifies a trade for lookup/close operations: either the trade's id
    /// (e.g. `6789`) or `@` followed by its client id.
    TradeSpecifier
}

impl OrderSpecifier {
    /// A specifier addressing an order by the client id assigned through
    /// [`ClientExtensions`](crate::models::ClientExtensions).
    pub fn from_client_id(id: impl AsRef<str>) -> Self {
        OrderSpecifier(format!("@{}", id.as_ref()))
    }
}

impl From<&OrderId> for OrderSpecifier {
    fn from(id: &OrderId) -> Self {
        OrderSpecifier(id.0.clone())
    }
}

impl From<OrderId> for OrderSpecifier {
    fn from(id: OrderId) -> Self {
        OrderSpecifier(id.0)
    }
}

impl TradeSpecifier {
    /// A specifier addressing a trade by the client id assigned through
    /// [`ClientExtensions`](crate::models::ClientExtensions).
    pub fn from_client_id(id: impl AsRef<str>) -> Self {
        TradeSpecifier(format!("@{}", id.as_ref()))
    }
}

impl From<&TradeId> for TradeSpecifier {
    fn from(id: &TradeId) -> Self {
        TradeSpecifier(id.0.clone())
    }
}

impl From<TradeId> for TradeSpecifier {
    fn from(id: TradeId) -> Self {
        TradeSpecifier(id.0)
    }
}

/// Declares a decimal newtype that serializes as a JSON string (OANDA's wire
/// format) and tolerantly deserializes from either a string or a number.
macro_rules! decimal_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        pub struct $name(pub Decimal);

        impl $name {
            /// Returns the underlying [`Decimal`] value.
            pub fn value(&self) -> Decimal {
                self.0
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                decimal_from_str_or_number(deserializer).map($name)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl From<Decimal> for $name {
            fn from(value: Decimal) -> Self {
                $name(value)
            }
        }

        impl From<$name> for Decimal {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl From<i64> for $name {
            fn from(value: i64) -> Self {
                $name(Decimal::from(value))
            }
        }

        impl From<i32> for $name {
            fn from(value: i32) -> Self {
                $name(Decimal::from(value))
            }
        }

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                $name(Decimal::from(value))
            }
        }

        impl FromStr for $name {
            type Err = rust_decimal::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                s.parse::<Decimal>()
                    .or_else(|_| Decimal::from_scientific(s))
                    .map($name)
            }
        }

        impl TryFrom<f64> for $name {
            type Error = rust_decimal::Error;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Decimal::try_from(value).map($name)
            }
        }
    };
}

decimal_newtype! {
    /// A decimal number encoded as a string on the wire (variable precision).
    DecimalNumber
}

decimal_newtype! {
    /// An amount in an account's home currency, encoded as a string on the
    /// wire.
    AccountUnits
}

decimal_newtype! {
    /// An instrument price, encoded as a string on the wire.
    PriceValue
}

/// A point in time as delivered by (and sent to) the OANDA API.
///
/// The wire representation depends on the `Accept-Datetime-Format` request
/// header ([`AcceptDatetimeFormat`]): RFC 3339 (`2024-06-14T12:01:32.000000Z`)
/// or a UNIX epoch value with fractional seconds (`1718366492.000000`).
/// The value is kept verbatim as a string; use [`DateTime::to_utc`] to parse
/// either representation into a [`chrono::DateTime`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTime(pub String);

impl DateTime {
    /// Returns the underlying string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parses the timestamp, accepting both the RFC 3339 and the UNIX
    /// (`seconds[.fraction]`) representations. Returns `None` when the value
    /// matches neither format.
    pub fn to_utc(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        let s = self.0.as_str();
        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
            return Some(dt.with_timezone(&chrono::Utc));
        }
        // UNIX format: "1718366492" or "1718366492.000000000".
        let (secs, frac) = match s.split_once('.') {
            Some((secs, frac)) => (secs, frac),
            None => (s, ""),
        };
        let secs: i64 = secs.parse().ok()?;
        let nanos: u32 = if frac.is_empty() {
            0
        } else if frac.len() <= 9 && frac.chars().all(|c| c.is_ascii_digit()) {
            frac.parse::<u32>().ok()? * 10u32.pow(9 - frac.len() as u32)
        } else {
            return None;
        };
        chrono::DateTime::from_timestamp(secs, nanos)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for DateTime {
    fn from(value: String) -> Self {
        DateTime(value)
    }
}

impl From<&str> for DateTime {
    fn from(value: &str) -> Self {
        DateTime(value.to_owned())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        DateTime(value.to_rfc3339_opts(chrono::SecondsFormat::Micros, true))
    }
}

/// Wire format for datetime fields, selected via the `Accept-Datetime-Format`
/// request header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AcceptDatetimeFormat {
    /// UNIX epoch seconds with fractional part, e.g. `1718366492.000000`.
    #[serde(rename = "UNIX")]
    Unix,
    /// RFC 3339 / ISO 8601, e.g. `2024-06-14T12:01:32.000000Z`. The default.
    #[default]
    #[serde(rename = "RFC3339")]
    Rfc3339,
}

impl AcceptDatetimeFormat {
    /// The value sent in the `Accept-Datetime-Format` header.
    pub fn as_header_value(self) -> &'static str {
        match self {
            AcceptDatetimeFormat::Unix => "UNIX",
            AcceptDatetimeFormat::Rfc3339 => "RFC3339",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_newtype_serializes_as_string() {
        let v = DecimalNumber::from_str("1.2345").unwrap();
        assert_eq!(serde_json::to_string(&v).unwrap(), r#""1.2345""#);
    }

    #[test]
    fn decimal_newtype_deserializes_from_number() {
        let v: PriceValue = serde_json::from_str("1.1050").unwrap();
        assert_eq!(v.to_string(), "1.105");
    }

    #[test]
    fn decimal_newtype_roundtrips_string() {
        let v: AccountUnits = serde_json::from_str(r#""-43.2100""#).unwrap();
        assert_eq!(serde_json::to_string(&v).unwrap(), r#""-43.2100""#);
    }

    #[test]
    fn datetime_parses_rfc3339() {
        let dt = DateTime::from("2024-06-14T12:01:32.123456Z");
        let parsed = dt.to_utc().unwrap();
        assert_eq!(parsed.timestamp(), 1718366492);
        assert_eq!(parsed.timestamp_subsec_micros(), 123456);
    }

    #[test]
    fn datetime_parses_unix_with_fraction() {
        let dt = DateTime::from("1718366492.123456000");
        let parsed = dt.to_utc().unwrap();
        assert_eq!(parsed.timestamp(), 1718366492);
        assert_eq!(parsed.timestamp_subsec_micros(), 123456);
    }

    #[test]
    fn datetime_parses_unix_without_fraction() {
        let dt = DateTime::from("1718366492");
        assert_eq!(dt.to_utc().unwrap().timestamp(), 1718366492);
    }

    #[test]
    fn datetime_rejects_garbage() {
        assert!(DateTime::from("not a date").to_utc().is_none());
    }

    #[test]
    fn order_specifier_from_client_id() {
        assert_eq!(OrderSpecifier::from_client_id("my-id").as_str(), "@my-id");
        assert_eq!(OrderSpecifier::from(OrderId::from("42")).as_str(), "42");
    }

    #[test]
    fn decimal_newtype_conversions() {
        assert_eq!(DecimalNumber::from(5i64).to_string(), "5");
        assert_eq!(DecimalNumber::from(-3i32).to_string(), "-3");
        assert_eq!(DecimalNumber::from(7u32).to_string(), "7");
        let d = rust_decimal::Decimal::new(12345, 4);
        assert_eq!(PriceValue::from(d).to_string(), "1.2345");
        assert_eq!(rust_decimal::Decimal::from(PriceValue::from(d)), d);
        assert_eq!(AccountUnits::try_from(0.5f64).unwrap().to_string(), "0.5");
        assert!(AccountUnits::try_from(f64::NAN).is_err());
        assert_eq!(PriceValue::default().value(), rust_decimal::Decimal::ZERO);
        assert!("garbage".parse::<DecimalNumber>().is_err());
        assert_eq!("2e3".parse::<DecimalNumber>().unwrap().to_string(), "2000");
    }

    #[test]
    fn string_newtype_conversions() {
        let id: AccountId = "101-004-1234567-001".parse().unwrap();
        assert_eq!(id.as_str(), "101-004-1234567-001");
        assert_eq!(id.to_string(), "101-004-1234567-001");
        assert_eq!(AccountId::from(String::from("x")), AccountId::from("x"));
        assert_eq!(TradeSpecifier::from_client_id("t").as_str(), "@t");
        assert_eq!(TradeSpecifier::from(TradeId::from("9")).as_str(), "9");
        assert_eq!(TradeSpecifier::from(&TradeId::from("9")).as_str(), "9");
        assert_eq!(OrderSpecifier::from(&OrderId::from("7")).as_str(), "7");
    }

    #[test]
    fn datetime_conversions() {
        let utc = chrono::DateTime::from_timestamp(1718366492, 123_456_000).unwrap();
        let dt = DateTime::from(utc);
        assert_eq!(dt.as_str(), "2024-06-14T12:01:32.123456Z");
        assert_eq!(dt.to_utc(), Some(utc));
        assert_eq!(dt.to_string(), dt.as_str());
        assert_eq!(DateTime::from(String::from("x")).as_str(), "x");
        // over-long fraction is rejected rather than mis-parsed
        assert!(DateTime::from("123.1234567891").to_utc().is_none());
    }

    #[test]
    fn accept_datetime_format_serde_and_header() {
        assert_eq!(
            AcceptDatetimeFormat::default(),
            AcceptDatetimeFormat::Rfc3339
        );
        assert_eq!(AcceptDatetimeFormat::Unix.as_header_value(), "UNIX");
        assert_eq!(
            serde_json::to_string(&AcceptDatetimeFormat::Rfc3339).unwrap(),
            r#""RFC3339""#
        );
        assert_eq!(
            serde_json::from_str::<AcceptDatetimeFormat>(r#""UNIX""#).unwrap(),
            AcceptDatetimeFormat::Unix
        );
    }
}
