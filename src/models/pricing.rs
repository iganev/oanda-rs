//! Pricing models: client prices, conversion factors, and stream heartbeats.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::{Currency, DateTime, DecimalNumber, InstrumentName, PriceValue, UnitsAvailable};

/// The Price representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Price {
    /// The Price's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// Flag indicating if the Price is tradeable or not
    #[serde(rename = "tradeable", skip_serializing_if = "Option::is_none")]
    pub tradeable: Option<bool>,

    /// The date/time when the Price was created.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime>,

    /// The base bid price as calculated by pricing.
    #[serde(rename = "baseBid", skip_serializing_if = "Option::is_none")]
    pub base_bid: Option<PriceValue>,

    /// The base ask price as calculated by pricing.
    #[serde(rename = "baseAsk", skip_serializing_if = "Option::is_none")]
    pub base_ask: Option<PriceValue>,

    /// The list of prices and liquidity available on the Instrument's bid side.
    /// It is possible for this list to be empty if there is no bid liquidity
    /// currently available for the Instrument in the Account.
    #[serde(rename = "bids", default, skip_serializing_if = "Vec::is_empty")]
    pub bids: Vec<PriceBucket>,

    /// The list of prices and liquidity available on the Instrument's ask side.
    /// It is possible for this list to be empty if there is no ask liquidity
    /// currently available for the Instrument in the Account.
    #[serde(rename = "asks", default, skip_serializing_if = "Vec::is_empty")]
    pub asks: Vec<PriceBucket>,

    /// The closeout bid price. This price is used when a bid is required to
    /// closeout a Position (margin closeout or manual) yet there is no bid
    /// liquidity. The closeout bid is never used to open a new position.
    #[serde(rename = "closeoutBid", skip_serializing_if = "Option::is_none")]
    pub closeout_bid: Option<PriceValue>,

    /// The closeout ask price. This price is used when an ask is required to
    /// closeout a Position (margin closeout or manual) yet there is no ask
    /// liquidity. The closeout ask is never used to open a new position.
    #[serde(rename = "closeoutAsk", skip_serializing_if = "Option::is_none")]
    pub closeout_ask: Option<PriceValue>,
}

/// The specification of an Account-specific Price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClientPrice {
    /// The type discriminator, always `PRICE`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The Price's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The date/time when the Price was created
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The `status` field.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<PriceStatus>,

    /// Flag indicating if the Price is tradeable or not
    #[serde(rename = "tradeable", skip_serializing_if = "Option::is_none")]
    pub tradeable: Option<bool>,

    /// The list of prices and liquidity available on the Instrument's bid side.
    /// It is possible for this list to be empty if there is no bid liquidity
    /// currently available for the Instrument in the Account.
    #[serde(rename = "bids", default, skip_serializing_if = "Vec::is_empty")]
    pub bids: Vec<PriceBucket>,

    /// The list of prices and liquidity available on the Instrument's ask side.
    /// It is possible for this list to be empty if there is no ask liquidity
    /// currently available for the Instrument in the Account.
    #[serde(rename = "asks", default, skip_serializing_if = "Vec::is_empty")]
    pub asks: Vec<PriceBucket>,

    /// The closeout bid Price. This Price is used when a bid is required to
    /// closeout a Position (margin closeout or manual) yet there is no bid
    /// liquidity. The closeout bid is never used to open a new position.
    #[serde(rename = "closeoutBid", skip_serializing_if = "Option::is_none")]
    pub closeout_bid: Option<PriceValue>,

    /// The closeout ask Price. This Price is used when a ask is required to
    /// closeout a Position (margin closeout or manual) yet there is no ask
    /// liquidity. The closeout ask is never used to open a new position.
    #[serde(rename = "closeoutAsk", skip_serializing_if = "Option::is_none")]
    pub closeout_ask: Option<PriceValue>,

    /// The `quoteHomeConversionFactors` field.
    #[serde(
        rename = "quoteHomeConversionFactors",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_home_conversion_factors: Option<QuoteHomeConversionFactors>,

    /// The `unitsAvailable` field.
    #[serde(rename = "unitsAvailable", skip_serializing_if = "Option::is_none")]
    pub units_available: Option<UnitsAvailable>,

    /// The date/time when the Price was created. This field is returned by the
    /// live v20 API but is not present in OANDA's official documentation.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime>,
}

/// A Price Bucket represents a price available for an amount of liquidity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PriceBucket {
    /// The Price offered by the PriceBucket
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The amount of liquidity offered by the PriceBucket. The REST API encodes
    /// this as a string, the streaming API as a JSON number; OANDA documents it
    /// as "integer or decimal if available".
    #[serde(rename = "liquidity", skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<DecimalNumber>,
}

string_enum! {
    /// The status of the Price.
    pub enum PriceStatus {
        Tradeable => "tradeable",
        NonTradeable => "non-tradeable",
        Invalid => "invalid",
    }
}

/// A PricingHeartbeat object is injected into the Pricing stream to ensure that
/// the HTTP connection remains active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PricingHeartbeat {
    /// The type discriminator, always `HEARTBEAT`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The date/time when the Heartbeat was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
}

/// HomeConversions represents the factors to use to convert quantities of a
/// given currency into the Account's home currency. The conversion factor
/// depends on the scenario the conversion is required for.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HomeConversions {
    /// The currency to be converted into the home currency.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The factor used to convert any gains for an Account in the specified
    /// currency into the Account's home currency. This would include positive
    /// realized P/L and positive financing amounts. Conversion is performed by
    /// multiplying the positive P/L by the conversion factor.
    #[serde(rename = "accountGain", skip_serializing_if = "Option::is_none")]
    pub account_gain: Option<DecimalNumber>,

    /// The string representation of a decimal number.
    #[serde(rename = "accountLoss", skip_serializing_if = "Option::is_none")]
    pub account_loss: Option<DecimalNumber>,

    /// The factor used to convert a Position or Trade Value in the specified
    /// currency into the Account's home currency. Conversion is performed by
    /// multiplying the Position or Trade Value by the conversion factor.
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none")]
    pub position_value: Option<DecimalNumber>,
}

/// A HomeConversionFactors message contains information used to convert
/// amounts, from an Instrument's base or quote currency, to the home currency
/// of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct HomeConversionFactors {
    /// The `gainQuoteHome` field.
    #[serde(rename = "gainQuoteHome", skip_serializing_if = "Option::is_none")]
    pub gain_quote_home: Option<ConversionFactor>,

    /// The `lossQuoteHome` field.
    #[serde(rename = "lossQuoteHome", skip_serializing_if = "Option::is_none")]
    pub loss_quote_home: Option<ConversionFactor>,

    /// The `gainBaseHome` field.
    #[serde(rename = "gainBaseHome", skip_serializing_if = "Option::is_none")]
    pub gain_base_home: Option<ConversionFactor>,

    /// The `lossBaseHome` field.
    #[serde(rename = "lossBaseHome", skip_serializing_if = "Option::is_none")]
    pub loss_base_home: Option<ConversionFactor>,
}

/// QuoteHomeConversionFactors represents the factors that can be used used to
/// convert quantities of a Price's Instrument's quote currency into the
/// Account's home currency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct QuoteHomeConversionFactors {
    /// The factor used to convert a positive amount of the Price's Instrument's
    /// quote currency into a positive amount of the Account's home currency.
    /// Conversion is performed by multiplying the quote units by the conversion
    /// factor.
    #[serde(rename = "positiveUnits", skip_serializing_if = "Option::is_none")]
    pub positive_units: Option<DecimalNumber>,

    /// The factor used to convert a negative amount of the Price's Instrument's
    /// quote currency into a negative amount of the Account's home currency.
    /// Conversion is performed by multiplying the quote units by the conversion
    /// factor.
    #[serde(rename = "negativeUnits", skip_serializing_if = "Option::is_none")]
    pub negative_units: Option<DecimalNumber>,
}

/// A ConversionFactor contains information used to convert an amount, from an
/// Instrument's base or quote currency, to the home currency of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ConversionFactor {
    /// The factor by which to multiply the amount in the given currency to
    /// obtain the amount in the home currency of the Account.
    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<DecimalNumber>,
}

/// A single line of the pricing stream: either a price update or a
/// heartbeat (sent every 5 seconds to keep the connection alive).
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)] // variants mirror the wire format; boxing would hurt ergonomics
pub enum PriceStreamItem {
    /// A price update (`"type": "PRICE"`).
    Price(ClientPrice),
    /// A keep-alive heartbeat (`"type": "HEARTBEAT"`).
    Heartbeat(PricingHeartbeat),
}

impl serde::Serialize for PriceStreamItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PriceStreamItem::Price(price) => price.serialize(serializer),
            PriceStreamItem::Heartbeat(heartbeat) => heartbeat.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for PriceStreamItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let is_heartbeat =
            value.get("type").and_then(serde_json::Value::as_str) == Some("HEARTBEAT");
        if is_heartbeat {
            serde_json::from_value(value)
                .map(PriceStreamItem::Heartbeat)
                .map_err(serde::de::Error::custom)
        } else {
            serde_json::from_value(value)
                .map(PriceStreamItem::Price)
                .map_err(serde::de::Error::custom)
        }
    }
}
