//! Position models.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::{
    AccountFinancingMode, AccountUnits, DecimalNumber, HomeConversionFactors, InstrumentName,
    OpenTradeFinancing, PriceValue, TradeId,
};

/// The specification of a Position within an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Position {
    /// The Position's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// Profit/loss realized by the Position over the lifetime of the Account.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<AccountUnits>,

    /// The unrealized profit/loss of all open Trades that contribute to this
    /// Position.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// Margin currently used by the Position.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// Profit/loss realized by the Position since the Account's resettablePL
    /// was last reset by the client.
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none")]
    pub resettable_pl: Option<AccountUnits>,

    /// The total amount of financing paid/collected for this instrument over
    /// the lifetime of the Account.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The total amount of commission paid for this instrument over the
    /// lifetime of the Account.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<AccountUnits>,

    /// The total amount of fees charged over the lifetime of the Account for
    /// the execution of guaranteed Stop Loss Orders for this instrument.
    #[serde(
        rename = "guaranteedExecutionFees",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<AccountUnits>,

    /// The `long` field.
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<PositionSide>,

    /// The `short` field.
    #[serde(rename = "short", skip_serializing_if = "Option::is_none")]
    pub short: Option<PositionSide>,

    /// The total amount of dividend adjustment paid. This field is returned by
    /// the live v20 API but is not present in OANDA's official documentation.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The unrealized profit/loss inclusive of unsettled amounts. This field is
    /// returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,
}

/// The representation of a Position for a single direction (long or short).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionSide {
    /// Number of units in the position (negative value indicates short
    /// position, positive indicates long position).
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// Volume-weighted average of the underlying Trade open prices for the
    /// Position.
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<PriceValue>,

    /// List of the open Trade IDs which contribute to the open Position.
    #[serde(rename = "tradeIDs", default, skip_serializing_if = "Vec::is_empty")]
    pub trade_ids: Vec<TradeId>,

    /// Profit/loss realized by the PositionSide over the lifetime of the
    /// Account.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<AccountUnits>,

    /// The unrealized profit/loss of all open Trades that contribute to this
    /// PositionSide.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// Profit/loss realized by the PositionSide since the Account's
    /// resettablePL was last reset by the client.
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none")]
    pub resettable_pl: Option<AccountUnits>,

    /// The total amount of financing paid/collected for this PositionSide over
    /// the lifetime of the Account.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The total amount of fees charged over the lifetime of the Account for
    /// the execution of guaranteed Stop Loss Orders attached to Trades for this
    /// PositionSide.
    #[serde(
        rename = "guaranteedExecutionFees",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<AccountUnits>,

    /// The total amount of dividend adjustment paid. This field is returned by
    /// the live v20 API but is not present in OANDA's official documentation.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The unrealized profit/loss inclusive of unsettled amounts. This field is
    /// returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,
}

/// The dynamic (calculated) state of a Position
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalculatedPositionState {
    /// The Position's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The Position's net unrealized profit/loss
    #[serde(rename = "netUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub net_unrealized_pl: Option<AccountUnits>,

    /// The unrealized profit/loss of the Position's long open Trades
    #[serde(rename = "longUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub long_unrealized_pl: Option<AccountUnits>,

    /// The unrealized profit/loss of the Position's short open Trades
    #[serde(rename = "shortUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub short_unrealized_pl: Option<AccountUnits>,

    /// Margin currently used by the Position.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,
}

string_enum! {
    /// The way that position values for an Account are calculated and
    /// aggregated.
    pub enum PositionAggregationMode {
        AbsoluteSum => "ABSOLUTE_SUM",
        MaximalSide => "MAXIMAL_SIDE",
        NetSum => "NET_SUM",
    }
}

/// OpenTradeFinancing is used to pay/collect daily financing charge for a
/// Position within an Account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PositionFinancing {
    /// The instrument of the Position that financing is being paid/collected
    /// for.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The amount of financing paid/collected for the Position.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The financing paid/collecte for each open Trade within the Position.
    #[serde(
        rename = "openTradeFinancings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub open_trade_financings: Vec<OpenTradeFinancing>,

    /// The amount of financing paid/collected in the Instrument's base
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(rename = "baseFinancing", skip_serializing_if = "Option::is_none")]
    pub base_financing: Option<DecimalNumber>,

    /// The `accountFinancingMode` field.
    #[serde(
        rename = "accountFinancingMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_financing_mode: Option<AccountFinancingMode>,

    /// The `homeConversionFactors` field.
    #[serde(
        rename = "homeConversionFactors",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,

    /// The total cost of currency conversions for the Position's financing, in
    /// the Account's home currency. This field is returned by the live v20 API
    /// but is not present in OANDA's official documentation.
    #[serde(rename = "homeConversionCost", skip_serializing_if = "Option::is_none")]
    pub home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the base financing to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "baseHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_home_conversion_cost: Option<DecimalNumber>,
}

/// Used to pay or collect a dividend adjustment amount for an open Trade within
/// the Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenTradeDividendAdjustment {
    /// The ID of the Trade for which the dividend adjustment is to be paid or
    /// collected.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,

    /// The dividend adjustment amount to pay or collect for the Trade, in the
    /// Account's home currency.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The dividend adjustment amount to pay or collect for the Trade, in the
    /// Instrument's quote currency.
    #[serde(
        rename = "quoteDividendAdjustment",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_dividend_adjustment: Option<DecimalNumber>,

    /// The cost of converting the dividend adjustment to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(rename = "homeConversionCost", skip_serializing_if = "Option::is_none")]
    pub home_conversion_cost: Option<DecimalNumber>,
}
