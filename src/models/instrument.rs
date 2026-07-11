//! Instrument metadata, candlestick, and order/position book models.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::{DateTime, DecimalNumber, GuaranteedStopLossOrderMode, InstrumentName, PriceValue};

/// Full specification of an Instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Instrument {
    /// The name of the Instrument
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<InstrumentName>,

    /// The `type` field.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<InstrumentType>,

    /// The display name of the Instrument
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// The location of the "pip" for this instrument. The decimal position of
    /// the pip in this Instrument's price can be found at 10 ^ pipLocation
    /// (e.g. -4 pipLocation results in a decimal pip position of 10 ^ -4 =
    /// 0.0001).
    #[serde(rename = "pipLocation", skip_serializing_if = "Option::is_none")]
    pub pip_location: Option<i64>,

    /// The number of decimal places that should be used to display prices for
    /// this instrument. (e.g. a displayPrecision of 5 would result in a price
    /// of "1" being displayed as "1.00000")
    #[serde(rename = "displayPrecision", skip_serializing_if = "Option::is_none")]
    pub display_precision: Option<i64>,

    /// The amount of decimal places that may be provided when specifying the
    /// number of units traded for this instrument.
    #[serde(
        rename = "tradeUnitsPrecision",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_units_precision: Option<i64>,

    /// The smallest number of units allowed to be traded for this instrument.
    #[serde(rename = "minimumTradeSize", skip_serializing_if = "Option::is_none")]
    pub minimum_trade_size: Option<DecimalNumber>,

    /// The maximum trailing stop distance allowed for a trailing stop loss
    /// created for this instrument. Specified in price units.
    #[serde(
        rename = "maximumTrailingStopDistance",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_trailing_stop_distance: Option<DecimalNumber>,

    /// The minimum trailing stop distance allowed for a trailing stop loss
    /// created for this instrument. Specified in price units.
    #[serde(
        rename = "minimumTrailingStopDistance",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_trailing_stop_distance: Option<DecimalNumber>,

    /// The maximum position size allowed for this instrument. Specified in
    /// units.
    #[serde(
        rename = "maximumPositionSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub maximum_position_size: Option<DecimalNumber>,

    /// The maximum units allowed for an Order placed for this instrument.
    /// Specified in units.
    #[serde(rename = "maximumOrderUnits", skip_serializing_if = "Option::is_none")]
    pub maximum_order_units: Option<DecimalNumber>,

    /// The margin rate for this instrument.
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    pub margin_rate: Option<DecimalNumber>,

    /// The `commission` field.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<InstrumentCommission>,

    /// The `guaranteedStopLossOrderMode` field.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,

    /// The minimum distance allowed between the Trade's fill price and the
    /// configured price for guaranteed Stop Loss Orders created for this
    /// instrument. Specified in price units.
    #[serde(
        rename = "minimumGuaranteedStopLossDistance",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_guaranteed_stop_loss_distance: Option<DecimalNumber>,

    /// The amount that is charged to the account if a guaranteed Stop Loss
    /// Order is triggered and filled. The value is in price units and is
    /// charged for each unit of the Trade.
    #[serde(
        rename = "guaranteedStopLossOrderExecutionPremium",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_execution_premium: Option<DecimalNumber>,

    /// The `guaranteedStopLossOrderLevelRestriction` field.
    #[serde(
        rename = "guaranteedStopLossOrderLevelRestriction",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_level_restriction:
        Option<GuaranteedStopLossOrderLevelRestriction>,

    /// The `financing` field.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<InstrumentFinancing>,

    /// The tags associated with this instrument.
    #[serde(rename = "tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
}

/// An InstrumentCommission represents an instrument-specific commission
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstrumentCommission {
    /// The commission amount (in the Account's home currency) charged per
    /// unitsTraded of the instrument
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<DecimalNumber>,

    /// The number of units traded that the commission amount is based on.
    #[serde(rename = "unitsTraded", skip_serializing_if = "Option::is_none")]
    pub units_traded: Option<DecimalNumber>,

    /// The minimum commission amount (in the Account's home currency) that is
    /// charged when an Order is filled for this instrument.
    #[serde(rename = "minimumCommission", skip_serializing_if = "Option::is_none")]
    pub minimum_commission: Option<DecimalNumber>,
}

/// Financing data for the instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstrumentFinancing {
    /// The financing rate to be used for a long position for the instrument.
    #[serde(rename = "longRate", skip_serializing_if = "Option::is_none")]
    pub long_rate: Option<DecimalNumber>,

    /// The financing rate to be used for a short position for the instrument.
    #[serde(rename = "shortRate", skip_serializing_if = "Option::is_none")]
    pub short_rate: Option<DecimalNumber>,

    /// The days of the week to debit or credit financing charges; the exact
    /// time of day at which to charge the financing is set in the
    /// DivisionTradingGroup for the client's account.
    #[serde(
        rename = "financingDaysOfWeek",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub financing_days_of_week: Vec<FinancingDayOfWeek>,
}

/// A FinancingDayOfWeek message defines a day of the week when financing
/// charges are debited or credited.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FinancingDayOfWeek {
    /// The `dayOfWeek` field.
    #[serde(rename = "dayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,

    /// The number of days worth of financing to be charged on dayOfWeek.
    #[serde(rename = "daysCharged", skip_serializing_if = "Option::is_none")]
    pub days_charged: Option<i64>,
}

string_enum! {
    /// The DayOfWeek provides a representation of the day of the week.
    pub enum DayOfWeek {
        Sunday => "SUNDAY",
        Monday => "MONDAY",
        Tuesday => "TUESDAY",
        Wednesday => "WEDNESDAY",
        Thursday => "THURSDAY",
        Friday => "FRIDAY",
        Saturday => "SATURDAY",
    }
}

string_enum! {
    /// The type of an Instrument.
    pub enum InstrumentType {
        Currency => "CURRENCY",
        Cfd => "CFD",
        Metal => "METAL",
    }
}

/// A tag associated with an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tag {
    /// The type of the tag.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The name of the tag.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A GuaranteedStopLossOrderLevelRestriction represents the total position size
/// that can exist within a given price window for Trades with guaranteed Stop
/// Loss Orders attached for a specific Instrument.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuaranteedStopLossOrderLevelRestriction {
    /// Applies to Trades with a guaranteed Stop Loss Order attached for the
    /// specified Instrument. This is the total allowed Trade volume that can
    /// exist within the priceRange based on the trigger prices of the
    /// guaranteed Stop Loss Orders.
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<DecimalNumber>,

    /// The price range the volume applies to. This value is in price units.
    #[serde(rename = "priceRange", skip_serializing_if = "Option::is_none")]
    pub price_range: Option<DecimalNumber>,
}

string_enum! {
    /// The granularity of a candlestick
    pub enum CandlestickGranularity {
        S5 => "S5",
        S10 => "S10",
        S15 => "S15",
        S30 => "S30",
        M1 => "M1",
        M2 => "M2",
        M4 => "M4",
        M5 => "M5",
        M10 => "M10",
        M15 => "M15",
        M30 => "M30",
        H1 => "H1",
        H2 => "H2",
        H3 => "H3",
        H4 => "H4",
        H6 => "H6",
        H8 => "H8",
        H12 => "H12",
        D => "D",
        W => "W",
        M => "M",
    }
}

string_enum! {
    /// The day of the week to use for candlestick granularities with weekly
    /// alignment.
    pub enum WeeklyAlignment {
        Monday => "Monday",
        Tuesday => "Tuesday",
        Wednesday => "Wednesday",
        Thursday => "Thursday",
        Friday => "Friday",
        Saturday => "Saturday",
        Sunday => "Sunday",
    }
}

/// The price data (open, high, low, close) for the Candlestick representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CandlestickData {
    /// The first (open) price in the time-range represented by the candlestick.
    #[serde(rename = "o", skip_serializing_if = "Option::is_none")]
    pub o: Option<PriceValue>,

    /// The highest price in the time-range represented by the candlestick.
    #[serde(rename = "h", skip_serializing_if = "Option::is_none")]
    pub h: Option<PriceValue>,

    /// The lowest price in the time-range represented by the candlestick.
    #[serde(rename = "l", skip_serializing_if = "Option::is_none")]
    pub l: Option<PriceValue>,

    /// The last (closing) price in the time-range represented by the
    /// candlestick.
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub c: Option<PriceValue>,
}

/// The Candlestick representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Candlestick {
    /// The start time of the candlestick
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The `bid` field.
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<CandlestickData>,

    /// The `ask` field.
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<CandlestickData>,

    /// The `mid` field.
    #[serde(rename = "mid", skip_serializing_if = "Option::is_none")]
    pub mid: Option<CandlestickData>,

    /// The number of prices created during the time-range represented by the
    /// candlestick.
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,

    /// A flag indicating if the candlestick is complete. A complete candlestick
    /// is one whose ending time is not in the future.
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

/// The LatestCandles representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstrumentCandles {
    /// The instrument whose Prices are represented by the candlesticks.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The `granularity` field.
    #[serde(rename = "granularity", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<CandlestickGranularity>,

    /// The list of candlesticks that satisfy the request.
    #[serde(rename = "candles", default, skip_serializing_if = "Vec::is_empty")]
    pub candles: Vec<Candlestick>,
}

/// The representation of an instrument's order book at a point in time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderBook {
    /// The order book's instrument
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The time when the order book snapshot was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The price (midpoint) for the order book's instrument at the time of the
    /// order book snapshot
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The price width for each bucket. Each bucket covers the price range from
    /// the bucket's price to the bucket's price + bucketWidth.
    #[serde(rename = "bucketWidth", skip_serializing_if = "Option::is_none")]
    pub bucket_width: Option<PriceValue>,

    /// The partitioned order book, divided into buckets using a default bucket
    /// width. These buckets are only provided for price ranges which actually
    /// contain order or position data.
    #[serde(rename = "buckets", default, skip_serializing_if = "Vec::is_empty")]
    pub buckets: Vec<OrderBookBucket>,

    /// The book's creation time as a Unix timestamp (seconds), string-encoded.
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "unixTime", skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<DateTime>,
}

/// The order book data for a partition of the instrument's prices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderBookBucket {
    /// The lowest price (inclusive) covered by the bucket. The bucket covers
    /// the price range from the price to price + the order book's bucketWidth.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The percentage of the total number of orders represented by the long
    /// orders found in this bucket.
    #[serde(rename = "longCountPercent", skip_serializing_if = "Option::is_none")]
    pub long_count_percent: Option<DecimalNumber>,

    /// The percentage of the total number of orders represented by the short
    /// orders found in this bucket.
    #[serde(rename = "shortCountPercent", skip_serializing_if = "Option::is_none")]
    pub short_count_percent: Option<DecimalNumber>,
}

/// The representation of an instrument's position book at a point in time
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionBook {
    /// The position book's instrument
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The time when the position book snapshot was created
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The price (midpoint) for the position book's instrument at the time of
    /// the position book snapshot
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The price width for each bucket. Each bucket covers the price range from
    /// the bucket's price to the bucket's price + bucketWidth.
    #[serde(rename = "bucketWidth", skip_serializing_if = "Option::is_none")]
    pub bucket_width: Option<PriceValue>,

    /// The partitioned position book, divided into buckets using a default
    /// bucket width. These buckets are only provided for price ranges which
    /// actually contain order or position data.
    #[serde(rename = "buckets", default, skip_serializing_if = "Vec::is_empty")]
    pub buckets: Vec<PositionBookBucket>,

    /// The book's creation time as a Unix timestamp (seconds), string-encoded.
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "unixTime", skip_serializing_if = "Option::is_none")]
    pub unix_time: Option<DateTime>,
}

/// The position book data for a partition of the instrument's prices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionBookBucket {
    /// The lowest price (inclusive) covered by the bucket. The bucket covers
    /// the price range from the price to price + the position book's
    /// bucketWidth.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The percentage of the total number of positions represented by the long
    /// positions found in this bucket.
    #[serde(rename = "longCountPercent", skip_serializing_if = "Option::is_none")]
    pub long_count_percent: Option<DecimalNumber>,

    /// The percentage of the total number of positions represented by the short
    /// positions found in this bucket.
    #[serde(rename = "shortCountPercent", skip_serializing_if = "Option::is_none")]
    pub short_count_percent: Option<DecimalNumber>,
}

/// The price component(s) a candlestick request applies to: any combination
/// of bid (`B`), mid (`M`) and ask (`A`).
///
/// ```
/// use oanda_rs::models::PricingComponent;
///
/// assert_eq!(PricingComponent::MID.to_string(), "M");
/// assert_eq!(PricingComponent::MID.with_bid().with_ask().to_string(), "BMA");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PricingComponent {
    /// Include bid candles.
    pub bid: bool,
    /// Include mid candles.
    pub mid: bool,
    /// Include ask candles.
    pub ask: bool,
}

impl PricingComponent {
    /// Bid candles only (`B`).
    pub const BID: PricingComponent = PricingComponent {
        bid: true,
        mid: false,
        ask: false,
    };
    /// Mid candles only (`M`) — OANDA's default.
    pub const MID: PricingComponent = PricingComponent {
        bid: false,
        mid: true,
        ask: false,
    };
    /// Ask candles only (`A`).
    pub const ASK: PricingComponent = PricingComponent {
        bid: false,
        mid: false,
        ask: true,
    };

    /// Adds the bid component.
    pub fn with_bid(mut self) -> Self {
        self.bid = true;
        self
    }

    /// Adds the mid component.
    pub fn with_mid(mut self) -> Self {
        self.mid = true;
        self
    }

    /// Adds the ask component.
    pub fn with_ask(mut self) -> Self {
        self.ask = true;
        self
    }
}

impl std::fmt::Display for PricingComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bid {
            f.write_str("B")?;
        }
        if self.mid {
            f.write_str("M")?;
        }
        if self.ask {
            f.write_str("A")?;
        }
        Ok(())
    }
}

impl std::str::FromStr for PricingComponent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut component = PricingComponent::default();
        for c in s.chars() {
            match c.to_ascii_uppercase() {
                'B' => component.bid = true,
                'M' => component.mid = true,
                'A' => component.ask = true,
                other => return Err(format!("invalid pricing component character: {other:?}")),
            }
        }
        Ok(component)
    }
}

/// An instrument name, a granularity, and a price component to get
/// candlestick data for, e.g. `EUR_USD:S10:BM`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandleSpecification {
    /// The instrument to get candles for.
    pub instrument: InstrumentName,
    /// The candlestick granularity.
    pub granularity: CandlestickGranularity,
    /// The price component(s); OANDA defaults to mid when omitted.
    pub price: Option<PricingComponent>,
}

impl CandleSpecification {
    /// A specification for the given instrument and granularity (mid price).
    pub fn new(instrument: impl Into<InstrumentName>, granularity: CandlestickGranularity) -> Self {
        CandleSpecification {
            instrument: instrument.into(),
            granularity,
            price: None,
        }
    }

    /// Selects the price component(s) for this specification.
    pub fn price(mut self, price: PricingComponent) -> Self {
        self.price = Some(price);
        self
    }
}

impl std::fmt::Display for CandleSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.instrument, self.granularity)?;
        if let Some(price) = &self.price {
            write!(f, ":{price}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pricing_component_display_and_parse() {
        assert_eq!(PricingComponent::BID.to_string(), "B");
        assert_eq!(PricingComponent::ASK.to_string(), "A");
        assert_eq!(
            PricingComponent::MID.with_bid().with_ask().to_string(),
            "BMA"
        );
        assert_eq!(PricingComponent::default().with_mid().to_string(), "M");
        assert_eq!(
            "bma".parse::<PricingComponent>().unwrap().to_string(),
            "BMA"
        );
        assert!("BX".parse::<PricingComponent>().is_err());
    }

    #[test]
    fn candle_specification_display() {
        let spec = CandleSpecification::new("EUR_USD", CandlestickGranularity::S10);
        assert_eq!(spec.to_string(), "EUR_USD:S10");
        let with_price =
            CandleSpecification::new(InstrumentName::XauUsd, CandlestickGranularity::M1)
                .price(PricingComponent::BID.with_mid());
        assert_eq!(with_price.to_string(), "XAU_USD:M1:BM");
    }

    #[test]
    fn string_enums_tolerate_unknown_values() {
        let g: CandlestickGranularity = serde_json::from_str(r#""H16""#).unwrap();
        assert_eq!(g, CandlestickGranularity::Other("H16".to_owned()));
        assert_eq!(serde_json::to_string(&g).unwrap(), r#""H16""#);
        assert_eq!(CandlestickGranularity::from("D"), CandlestickGranularity::D);
        assert_eq!(
            WeeklyAlignment::from(String::from("Friday")),
            WeeklyAlignment::Friday
        );
    }
}
