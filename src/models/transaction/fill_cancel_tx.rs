//! Order fill/cancel and client-extension-modify transaction subtypes.

use serde::{Deserialize, Serialize};

use crate::models::macros::string_enum;
use crate::models::transaction::TransactionRejectReason;
use crate::models::{
    AccountId, AccountUnits, ClientExtensions, ClientPrice, DateTime, DecimalNumber,
    HomeConversionFactors, InstrumentName, OrderId, PriceValue, RequestId, TradeId, TradeOpen,
    TradeReduce, TransactionId,
};

/// An OrderFillTransaction represents the filling of an Order in the client's
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderFillTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "ORDER_FILL" by the enum wrapper
    /// The ID of the Order filled.
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The client Order ID of the Order filled (only provided if the client has
    /// assigned one).
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// The name of the filled Order's instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The number of units filled by the OrderFill.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// This is the conversion factor in effect for the Account at the time of
    /// the OrderFill for converting any gains realized in Instrument quote
    /// units into units of the Account's home currency.
    #[serde(
        rename = "gainQuoteHomeConversionFactor",
        skip_serializing_if = "Option::is_none"
    )]
    pub gain_quote_home_conversion_factor: Option<DecimalNumber>,

    /// This is the conversion factor in effect for the Account at the time of
    /// the OrderFill for converting any losses realized in Instrument quote
    /// units into units of the Account's home currency.
    #[serde(
        rename = "lossQuoteHomeConversionFactor",
        skip_serializing_if = "Option::is_none"
    )]
    pub loss_quote_home_conversion_factor: Option<DecimalNumber>,

    /// This field is now deprecated and should no longer be used. The
    /// individual tradesClosed, tradeReduced and tradeOpened fields contain the
    /// exact/official price each unit was filled at.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The price that all of the units of the OrderFill should have been filled
    /// at, in the absence of guaranteed price execution. This factors in the
    /// Account's current ClientPrice, used liquidity and the units of the
    /// OrderFill only. If no Trades were closed with their price clamped for
    /// guaranteed stop loss enforcement, then this value will match the price
    /// fields of each Trade opened, closed, and reduced, and they will all be
    /// the exact same.
    #[serde(rename = "fullVWAP", skip_serializing_if = "Option::is_none")]
    pub full_vwap: Option<PriceValue>,

    /// The `fullPrice` field.
    #[serde(rename = "fullPrice", skip_serializing_if = "Option::is_none")]
    pub full_price: Option<ClientPrice>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<OrderFillReason>,

    /// The profit or loss incurred when the Order was filled.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<AccountUnits>,

    /// The financing paid or collected when the Order was filled.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The commission charged in the Account's home currency as a result of
    /// filling the Order. The commission is always represented as a positive
    /// quantity of the Account's home currency, however it reduces the balance
    /// in the Account.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<AccountUnits>,

    /// The total guaranteed execution fees charged for all Trades opened,
    /// closed or reduced with guaranteed Stop Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFee",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fee: Option<AccountUnits>,

    /// The Account's balance after the Order was filled.
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<AccountUnits>,

    /// The `tradeOpened` field.
    #[serde(rename = "tradeOpened", skip_serializing_if = "Option::is_none")]
    pub trade_opened: Option<TradeOpen>,

    /// The Trades that were closed when the Order was filled (only provided if
    /// filling the Order resulted in a closing open Trades).
    #[serde(
        rename = "tradesClosed",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trades_closed: Vec<TradeReduce>,

    /// The `tradeReduced` field.
    #[serde(rename = "tradeReduced", skip_serializing_if = "Option::is_none")]
    pub trade_reduced: Option<TradeReduce>,

    /// The half spread cost for the OrderFill, which is the sum of the
    /// halfSpreadCost values in the tradeOpened, tradesClosed and tradeReduced
    /// fields. This can be a positive or negative value and is represented in
    /// the home currency of the Account.
    #[serde(rename = "halfSpreadCost", skip_serializing_if = "Option::is_none")]
    pub half_spread_cost: Option<AccountUnits>,

    /// The number of units the Order requested to be filled. This field is
    /// returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(rename = "requestedUnits", skip_serializing_if = "Option::is_none")]
    pub requested_units: Option<DecimalNumber>,

    /// The profit or loss of the fill expressed in the Instrument's quote
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(rename = "quotePL", skip_serializing_if = "Option::is_none")]
    pub quote_pl: Option<DecimalNumber>,

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

    /// The `homeConversionFactors` field.
    #[serde(
        rename = "homeConversionFactors",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,

    /// The total cost of currency conversions for the fill, in the Account's
    /// home currency. This field is returned by the live v20 API but is not
    /// present in OANDA's official documentation.
    #[serde(rename = "homeConversionCost", skip_serializing_if = "Option::is_none")]
    pub home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the fill's profit/loss to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(
        rename = "plHomeConversionCost",
        skip_serializing_if = "Option::is_none"
    )]
    pub pl_home_conversion_cost: Option<DecimalNumber>,

    /// The cost of converting the fill's base financing to the Account's home
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

/// An OrderCancelTransaction represents the cancellation of an Order in the
/// client's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "ORDER_CANCEL" by the enum wrapper
    /// The ID of the Order cancelled
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The client ID of the Order cancelled (only provided if the Order has a
    /// client Order ID).
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<OrderId>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<OrderCancelReason>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled for replacement).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,

    /// The ID of the Trade whose closure triggered cancellation of this linked
    /// Order. This field is returned by the live v20 API but is not present in
    /// OANDA's official documentation.
    #[serde(rename = "closedTradeID", skip_serializing_if = "Option::is_none")]
    pub closed_trade_id: Option<String>,

    /// The ID of the Transaction that closed the linked Trade. This field is
    /// returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(
        rename = "tradeCloseTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_close_transaction_id: Option<String>,
}

/// An OrderCancelRejectTransaction represents the rejection of the cancellation
/// of an Order in the client's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderCancelRejectTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "ORDER_CANCEL_REJECT" by the enum wrapper
    /// The ID of the Order intended to be cancelled
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The client ID of the Order intended to be cancelled (only provided if
    /// the Order has a client Order ID).
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A OrderClientExtensionsModifyTransaction represents the modification of an
/// Order's Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderClientExtensionsModifyTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "ORDER_CLIENT_EXTENSIONS_MODIFY" by the enum wrapper
    /// The ID of the Order who's client extensions are to be modified.
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The original Client ID of the Order who's client extensions are to be
    /// modified.
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// The `clientExtensionsModify` field.
    #[serde(
        rename = "clientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions_modify: Option<ClientExtensions>,

    /// The `tradeClientExtensionsModify` field.
    #[serde(
        rename = "tradeClientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
}

/// A OrderClientExtensionsModifyRejectTransaction represents the rejection of
/// the modification of an Order's Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderClientExtensionsModifyRejectTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" by the enum wrapper
    /// The ID of the Order who's client extensions are to be modified.
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The original Client ID of the Order who's client extensions are to be
    /// modified.
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,

    /// The `clientExtensionsModify` field.
    #[serde(
        rename = "clientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub client_extensions_modify: Option<ClientExtensions>,

    /// The `tradeClientExtensionsModify` field.
    #[serde(
        rename = "tradeClientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A TradeClientExtensionsModifyTransaction represents the modification of a
/// Trade's Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeClientExtensionsModifyTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "TRADE_CLIENT_EXTENSIONS_MODIFY" by the enum wrapper
    /// The ID of the Trade who's client extensions are to be modified.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The original Client ID of the Trade who's client extensions are to be
    /// modified.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The `tradeClientExtensionsModify` field.
    #[serde(
        rename = "tradeClientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,
}

/// A TradeClientExtensionsModifyRejectTransaction represents the rejection of
/// the modification of a Trade's Client Extensions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TradeClientExtensionsModifyRejectTransaction {
    /// The Transaction's Identifier.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<TransactionId>,

    /// The date/time when the Transaction was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,

    /// The ID of the user that initiated the creation of the Transaction.
    #[serde(rename = "userID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,

    /// The ID of the Account the Transaction was created for.
    #[serde(rename = "accountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,

    /// The ID of the "batch" that the Transaction belongs to. Transactions in
    /// the same batch are applied to the Account simultaneously.
    #[serde(rename = "batchID", skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<TransactionId>,

    /// The Request ID of the request which generated the transaction.
    #[serde(rename = "requestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<RequestId>,

    // type is pinned to "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" by the enum wrapper
    /// The ID of the Trade who's client extensions are to be modified.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The original Client ID of the Trade who's client extensions are to be
    /// modified.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The `tradeClientExtensionsModify` field.
    #[serde(
        rename = "tradeClientExtensionsModify",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions_modify: Option<ClientExtensions>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

string_enum! {
    /// The reason that an Order was filled
    pub enum OrderFillReason {
        LimitOrder => "LIMIT_ORDER",
        StopOrder => "STOP_ORDER",
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        StopLossOrder => "STOP_LOSS_ORDER",
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        MarketOrder => "MARKET_ORDER",
        MarketOrderTradeClose => "MARKET_ORDER_TRADE_CLOSE",
        MarketOrderPositionCloseout => "MARKET_ORDER_POSITION_CLOSEOUT",
        MarketOrderMarginCloseout => "MARKET_ORDER_MARGIN_CLOSEOUT",
        MarketOrderDelayedTradeClose => "MARKET_ORDER_DELAYED_TRADE_CLOSE",
    }
}

string_enum! {
    /// The reason that an Order was cancelled.
    pub enum OrderCancelReason {
        InternalServerError => "INTERNAL_SERVER_ERROR",
        AccountLocked => "ACCOUNT_LOCKED",
        AccountNewPositionsLocked => "ACCOUNT_NEW_POSITIONS_LOCKED",
        AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
        AccountOrderFillLocked => "ACCOUNT_ORDER_FILL_LOCKED",
        ClientRequest => "CLIENT_REQUEST",
        Migration => "MIGRATION",
        MarketHalted => "MARKET_HALTED",
        LinkedTradeClosed => "LINKED_TRADE_CLOSED",
        TimeInForceExpired => "TIME_IN_FORCE_EXPIRED",
        InsufficientMargin => "INSUFFICIENT_MARGIN",
        FifoViolation => "FIFO_VIOLATION",
        BoundsViolation => "BOUNDS_VIOLATION",
        ClientRequestReplaced => "CLIENT_REQUEST_REPLACED",
        InsufficientLiquidity => "INSUFFICIENT_LIQUIDITY",
        TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TakeProfitOnFillLoss => "TAKE_PROFIT_ON_FILL_LOSS",
        LosingTakeProfit => "LOSING_TAKE_PROFIT",
        StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        StopLossOnFillLoss => "STOP_LOSS_ON_FILL_LOSS",
        StopLossOnFillPriceDistanceMaximumExceeded => "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        StopLossOnFillRequired => "STOP_LOSS_ON_FILL_REQUIRED",
        StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        StopLossOnFillGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOnFillGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOnFillGuaranteedHedgingNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED",
        StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        StopLossOnFillTriggerConditionInvalid => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        TakeProfitOnFillPriceDistanceMaximumExceeded => "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        TrailingStopLossOnFillGtdTimestampInPast => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
        PositionCloseoutFailed => "POSITION_CLOSEOUT_FAILED",
        OpenTradesAllowedExceeded => "OPEN_TRADES_ALLOWED_EXCEEDED",
        PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
        TakeProfitOnFillClientOrderIdAlreadyExists => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        StopLossOnFillClientOrderIdAlreadyExists => "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        TrailingStopLossOnFillClientOrderIdAlreadyExists => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS",
        PositionSizeExceeded => "POSITION_SIZE_EXCEEDED",
        HedgingGsloViolation => "HEDGING_GSLO_VIOLATION",
        AccountPositionValueLimitExceeded => "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED",
        InstrumentBidReduceOnly => "INSTRUMENT_BID_REDUCE_ONLY",
        InstrumentAskReduceOnly => "INSTRUMENT_ASK_REDUCE_ONLY",
        InstrumentBidHalted => "INSTRUMENT_BID_HALTED",
        InstrumentAskHalted => "INSTRUMENT_ASK_HALTED",
        StopLossOnFillGuaranteedBidHalted => "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED",
        StopLossOnFillGuaranteedAskHalted => "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED",
    }
}
