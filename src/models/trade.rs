//! Trade models.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::{
    AccountUnits, ClientExtensions, DateTime, DecimalNumber, InstrumentName, OrderId, PriceValue,
    StopLossOrder, TakeProfitOrder, TradeId, TrailingStopLossOrder, TransactionId,
};

/// The specification of a Trade within an Account. This includes the full
/// representation of the Trade's dependent Orders in addition to the IDs of
/// those Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Trade {
    /// The Trade's identifier, unique within the Trade's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeId>,

    /// The Trade's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The execution price of the Trade.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The date/time when the Trade was opened.
    #[serde(rename = "openTime", skip_serializing_if = "Option::is_none")]
    pub open_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<TradeState>,

    /// The initial size of the Trade. Negative values indicate a short Trade,
    /// and positive values indicate a long Trade.
    #[serde(rename = "initialUnits", skip_serializing_if = "Option::is_none")]
    pub initial_units: Option<DecimalNumber>,

    /// The margin required at the time the Trade was created. Note, this is the
    /// 'pure' margin required, it is not the 'effective' margin used that
    /// factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<AccountUnits>,

    /// The number of units currently open for the Trade. This value is reduced
    /// to 0.0 as the Trade is closed.
    #[serde(rename = "currentUnits", skip_serializing_if = "Option::is_none")]
    pub current_units: Option<DecimalNumber>,

    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(rename = "realizedPL", skip_serializing_if = "Option::is_none")]
    pub realized_pl: Option<AccountUnits>,

    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// Margin currently used by the Trade.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// The average closing price of the Trade. Only present if the Trade has
    /// been closed or reduced at least once.
    #[serde(rename = "averageClosePrice", skip_serializing_if = "Option::is_none")]
    pub average_close_price: Option<PriceValue>,

    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(
        rename = "closingTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub closing_transaction_ids: Vec<TransactionId>,

    /// The financing paid/collected for this Trade.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The date/time when the Trade was fully closed. Only provided for Trades
    /// whose state is CLOSED.
    #[serde(rename = "closeTime", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<DateTime>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOrder` field.
    #[serde(rename = "takeProfitOrder", skip_serializing_if = "Option::is_none")]
    pub take_profit_order: Option<TakeProfitOrder>,

    /// The `stopLossOrder` field.
    #[serde(rename = "stopLossOrder", skip_serializing_if = "Option::is_none")]
    pub stop_loss_order: Option<StopLossOrder>,

    /// The `trailingStopLossOrder` field.
    #[serde(
        rename = "trailingStopLossOrder",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order: Option<TrailingStopLossOrder>,

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

/// The summary of a Trade within an Account. This representation does not
/// provide the full details of the Trade's dependent Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeSummary {
    /// The Trade's identifier, unique within the Trade's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeId>,

    /// The Trade's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The execution price of the Trade.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The date/time when the Trade was opened.
    #[serde(rename = "openTime", skip_serializing_if = "Option::is_none")]
    pub open_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<TradeState>,

    /// The initial size of the Trade. Negative values indicate a short Trade,
    /// and positive values indicate a long Trade.
    #[serde(rename = "initialUnits", skip_serializing_if = "Option::is_none")]
    pub initial_units: Option<DecimalNumber>,

    /// The margin required at the time the Trade was created. Note, this is the
    /// 'pure' margin required, it is not the 'effective' margin used that
    /// factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<AccountUnits>,

    /// The number of units currently open for the Trade. This value is reduced
    /// to 0.0 as the Trade is closed.
    #[serde(rename = "currentUnits", skip_serializing_if = "Option::is_none")]
    pub current_units: Option<DecimalNumber>,

    /// The total profit/loss realized on the closed portion of the Trade.
    #[serde(rename = "realizedPL", skip_serializing_if = "Option::is_none")]
    pub realized_pl: Option<AccountUnits>,

    /// The unrealized profit/loss on the open portion of the Trade.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// Margin currently used by the Trade.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// The average closing price of the Trade. Only present if the Trade has
    /// been closed or reduced at least once.
    #[serde(rename = "averageClosePrice", skip_serializing_if = "Option::is_none")]
    pub average_close_price: Option<PriceValue>,

    /// The IDs of the Transactions that have closed portions of this Trade.
    #[serde(
        rename = "closingTransactionIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub closing_transaction_ids: Vec<TransactionId>,

    /// The financing paid/collected for this Trade.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The date/time when the Trade was fully closed. Only provided for Trades
    /// whose state is CLOSED.
    #[serde(rename = "closeTime", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<DateTime>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// ID of the Trade's Take Profit Order, only provided if such an Order
    /// exists.
    #[serde(rename = "takeProfitOrderID", skip_serializing_if = "Option::is_none")]
    pub take_profit_order_id: Option<OrderId>,

    /// ID of the Trade's Stop Loss Order, only provided if such an Order
    /// exists.
    #[serde(rename = "stopLossOrderID", skip_serializing_if = "Option::is_none")]
    pub stop_loss_order_id: Option<OrderId>,

    /// ID of the Trade's Trailing Stop Loss Order, only provided if such an
    /// Order exists.
    #[serde(
        rename = "trailingStopLossOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_order_id: Option<OrderId>,

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

/// The dynamic (calculated) state of an open Trade
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalculatedTradeState {
    /// The Trade's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TradeId>,

    /// The Trade's unrealized profit/loss.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// Margin currently used by the Trade.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,
}

string_enum! {
    /// The classification of TradePLs.
    pub enum TradePL {
        Positive => "POSITIVE",
        Negative => "NEGATIVE",
        Zero => "ZERO",
    }
}

string_enum! {
    /// The current state of the Trade.
    pub enum TradeState {
        Open => "OPEN",
        Closed => "CLOSED",
        CloseWhenTradeable => "CLOSE_WHEN_TRADEABLE",
    }
}

string_enum! {
    /// The state to filter the Trades by
    pub enum TradeStateFilter {
        Open => "OPEN",
        Closed => "CLOSED",
        CloseWhenTradeable => "CLOSE_WHEN_TRADEABLE",
        All => "ALL",
    }
}

/// A TradeOpen object represents a Trade for an instrument that was opened in
/// an Account. It is found embedded in Transactions that affect the position of
/// an instrument in the Account, specifically the OrderFill Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeOpen {
    /// The ID of the Trade that was opened
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The number of units opened by the Trade
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The average price that the units were opened at.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// This is the fee charged for opening the trade if it has a guaranteed
    /// Stop Loss Order attached to it.
    #[serde(
        rename = "guaranteedExecutionFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<AccountUnits>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The half spread cost for the trade open. This can be a positive or
    /// negative value and is represented in the home currency of the Account.
    #[serde(rename = "halfSpreadCost", skip_serializing_if = "Option::is_none")]
    pub half_spread_cost: Option<AccountUnits>,

    /// The margin required at the time the Trade was created. Note, this is the
    /// 'pure' margin required, it is not the 'effective' margin used that
    /// factors in the trade risk if a GSLO is attached to the trade.
    #[serde(
        rename = "initialMarginRequired",
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_margin_required: Option<AccountUnits>,

    /// The guaranteed execution fee expressed in the Instrument's quote
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_guaranteed_execution_fee: Option<DecimalNumber>,

    /// The cost of converting the guaranteed execution fee to the Account's
    /// home currency. This field is returned by the live v20 API but is not
    /// present in OANDA's official documentation.
    #[serde(
        rename = "guaranteedExecutionFeeHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee_home_conversion_cost: Option<DecimalNumber>,
}

/// A TradeReduce object represents a Trade for an instrument that was reduced
/// (either partially or fully) in an Account. It is found embedded in
/// Transactions that affect the position of an instrument in the account,
/// specifically the OrderFill Transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeReduce {
    /// The ID of the Trade that was reduced or closed
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The number of units that the Trade was reduced by
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The average price that the units were closed at. This price may be
    /// clamped for guaranteed Stop Loss Orders.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The PL realized when reducing the Trade
    #[serde(rename = "realizedPL", skip_serializing_if = "Option::is_none")]
    pub realized_pl: Option<AccountUnits>,

    /// The financing paid/collected when reducing the Trade
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// This is the fee that is charged for closing the Trade if it has a
    /// guaranteed Stop Loss Order attached to it.
    #[serde(
        rename = "guaranteedExecutionFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<AccountUnits>,

    /// The half spread cost for the trade reduce/close. This can be a positive
    /// or negative value and is represented in the home currency of the
    /// Account.
    #[serde(rename = "halfSpreadCost", skip_serializing_if = "Option::is_none")]
    pub half_spread_cost: Option<AccountUnits>,

    /// The financing paid or collected in the Instrument's base currency. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "baseFinancing", skip_serializing_if = "Option::is_none")]
    pub base_financing: Option<DecimalNumber>,

    /// The guaranteed execution fee expressed in the Instrument's quote
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "quoteGuaranteedExecutionFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_guaranteed_execution_fee: Option<DecimalNumber>,

    /// The financing rate in effect for the reduced Trade. This field is
    /// returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(rename = "financingRate", skip_serializing_if = "Option::is_none")]
    pub financing_rate: Option<DecimalNumber>,

    /// The total cost of currency conversions, in the Account's home currency.
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "homeConversionCost", skip_serializing_if = "Option::is_none")]
    pub home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the realized profit/loss to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "plHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub pl_home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the base financing to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "baseFinancingHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub base_financing_home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the guaranteed execution fee to the Account's
    /// home currency. This field is returned by the live v20 API but is not
    /// present in OANDA's official documentation.
    #[serde(
        rename = "guaranteedExecutionFeeHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee_home_conversion_cost: Option<DecimalNumber>,
}

/// OpenTradeFinancing is used to pay/collect daily financing charge for an open
/// Trade within an Account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OpenTradeFinancing {
    /// The ID of the Trade that financing is being paid/collected for.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The amount of financing paid/collected for the Trade.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The amount of financing paid/collected in the Instrument's base
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(rename = "baseFinancing", skip_serializing_if = "Option::is_none")]
    pub base_financing: Option<DecimalNumber>,

    /// The financing rate in effect for the Trade. This field is returned by
    /// the live v20 API but is not present in OANDA's official documentation.
    #[serde(rename = "financingRate", skip_serializing_if = "Option::is_none")]
    pub financing_rate: Option<DecimalNumber>,

    /// The total cost of currency conversions for the Trade's financing, in the
    /// Account's home currency. This field is returned by the live v20 API but
    /// is not present in OANDA's official documentation.
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
