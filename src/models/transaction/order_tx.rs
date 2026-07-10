//! Order create/reject transaction subtypes and their reason enums.

use serde::{Deserialize, Serialize};

use crate::models::macros::string_enum;
use crate::models::transaction::TransactionRejectReason;
use crate::models::{
    AccountId, ClientExtensions, DateTime, DecimalNumber, InstrumentName, LimitOrderTimeInForce,
    MarketIfTouchedOrderTimeInForce, MarketOrderDelayedTradeClose, MarketOrderMarginCloseout,
    MarketOrderPositionCloseout, MarketOrderTimeInForce, MarketOrderTradeClose, OrderId,
    OrderPositionFill, OrderTriggerCondition, PriceValue, RequestId, StopLossDetails,
    StopLossOrderTimeInForce, StopOrderTimeInForce, TakeProfitDetails, TakeProfitOrderTimeInForce,
    TradeId, TradeState, TrailingStopLossDetails, TrailingStopLossOrderTimeInForce, TransactionId,
};

/// A MarketOrderTransaction represents the creation of a Market Order in the
/// user's account. A Market Order is an Order that is filled immediately at the
/// current market price. Market Orders can be specialized when they are created
/// to accomplish a specific task: to close a Trade, to closeout a Position or
/// to particiate in in a Margin closeout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderTransaction {
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

    // type is pinned to "MARKET_ORDER" by the enum wrapper
    /// The Market Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Market Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketOrderTimeInForce>,

    /// The worst price that the client is willing to have the Market Order
    /// filled at.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `tradeClose` field.
    #[serde(rename = "tradeClose", skip_serializing_if = "Option::is_none")]
    pub trade_close: Option<MarketOrderTradeClose>,

    /// The `longPositionCloseout` field.
    #[serde(
        rename = "longPositionCloseout",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position_closeout: Option<MarketOrderPositionCloseout>,

    /// The `shortPositionCloseout` field.
    #[serde(
        rename = "shortPositionCloseout",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position_closeout: Option<MarketOrderPositionCloseout>,

    /// The `marginCloseout` field.
    #[serde(rename = "marginCloseout", skip_serializing_if = "Option::is_none")]
    pub margin_closeout: Option<MarketOrderMarginCloseout>,

    /// The `delayedTradeClose` field.
    #[serde(rename = "delayedTradeClose", skip_serializing_if = "Option::is_none")]
    pub delayed_trade_close: Option<MarketOrderDelayedTradeClose>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A MarketOrderRejectTransaction represents the rejection of the creation of a
/// Market Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderRejectTransaction {
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

    // type is pinned to "MARKET_ORDER_REJECT" by the enum wrapper
    /// The Market Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Market Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketOrderTimeInForce>,

    /// The worst price that the client is willing to have the Market Order
    /// filled at.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `tradeClose` field.
    #[serde(rename = "tradeClose", skip_serializing_if = "Option::is_none")]
    pub trade_close: Option<MarketOrderTradeClose>,

    /// The `longPositionCloseout` field.
    #[serde(
        rename = "longPositionCloseout",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_position_closeout: Option<MarketOrderPositionCloseout>,

    /// The `shortPositionCloseout` field.
    #[serde(
        rename = "shortPositionCloseout",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_position_closeout: Option<MarketOrderPositionCloseout>,

    /// The `marginCloseout` field.
    #[serde(rename = "marginCloseout", skip_serializing_if = "Option::is_none")]
    pub margin_closeout: Option<MarketOrderMarginCloseout>,

    /// The `delayedTradeClose` field.
    #[serde(rename = "delayedTradeClose", skip_serializing_if = "Option::is_none")]
    pub delayed_trade_close: Option<MarketOrderDelayedTradeClose>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A FixedPriceOrderTransaction represents the creation of a Fixed Price Order
/// in the user's account. A Fixed Price Order is an Order that is filled
/// immediately at a specified price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FixedPriceOrderTransaction {
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

    // type is pinned to "FIXED_PRICE_ORDER" by the enum wrapper
    /// The Fixed Price Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Fixed Price Order. A
    /// posititive number of units results in a long Order, and a negative
    /// number of units results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price specified for the Fixed Price Order. This price is the exact
    /// price that the Fixed Price Order will be filled at.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `tradeState` field.
    #[serde(rename = "tradeState", skip_serializing_if = "Option::is_none")]
    pub trade_state: Option<TradeState>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<FixedPriceOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

/// A LimitOrderTransaction represents the creation of a Limit Order in the
/// user's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LimitOrderTransaction {
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

    // type is pinned to "LIMIT_ORDER" by the enum wrapper
    /// The Limit Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Limit Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the Limit Order. The Limit Order will
    /// only be filled by a market price that is equal to or better than this
    /// price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<LimitOrderTimeInForce>,

    /// The date/time when the Limit Order will be cancelled if its timeInForce
    /// is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<LimitOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A LimitOrderRejectTransaction represents the rejection of the creation of a
/// Limit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LimitOrderRejectTransaction {
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

    // type is pinned to "LIMIT_ORDER_REJECT" by the enum wrapper
    /// The Limit Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Limit Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the Limit Order. The Limit Order will
    /// only be filled by a market price that is equal to or better than this
    /// price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<LimitOrderTimeInForce>,

    /// The date/time when the Limit Order will be cancelled if its timeInForce
    /// is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<LimitOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A StopOrderTransaction represents the creation of a Stop Order in the user's
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopOrderTransaction {
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

    // type is pinned to "STOP_ORDER" by the enum wrapper
    /// The Stop Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Stop Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the Stop Order. The Stop Order will
    /// only be filled by a market price that is equal to or worse than this
    /// price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The worst market price that may be used to fill this Stop Order. If the
    /// market gaps and crosses through both the price and the priceBound, the
    /// Stop Order will be cancelled instead of being filled.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopOrderTimeInForce>,

    /// The date/time when the Stop Order will be cancelled if its timeInForce
    /// is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,

    /// The price trigger mode of the Order. Observed value: "TOP_OF_BOOK". This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// A StopOrderRejectTransaction represents the rejection of the creation of a
/// Stop Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopOrderRejectTransaction {
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

    // type is pinned to "STOP_ORDER_REJECT" by the enum wrapper
    /// The Stop Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the Stop Order. A posititive
    /// number of units results in a long Order, and a negative number of units
    /// results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the Stop Order. The Stop Order will
    /// only be filled by a market price that is equal to or worse than this
    /// price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The worst market price that may be used to fill this Stop Order. If the
    /// market gaps and crosses through both the price and the priceBound, the
    /// Stop Order will be cancelled instead of being filled.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopOrderTimeInForce>,

    /// The date/time when the Stop Order will be cancelled if its timeInForce
    /// is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,

    /// The price trigger mode of the Order. Observed value: "TOP_OF_BOOK". This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// A MarketIfTouchedOrderTransaction represents the creation of a
/// MarketIfTouched Order in the user's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketIfTouchedOrderTransaction {
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

    // type is pinned to "MARKET_IF_TOUCHED_ORDER" by the enum wrapper
    /// The MarketIfTouched Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the MarketIfTouched Order. A
    /// posititive number of units results in a long Order, and a negative
    /// number of units results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the MarketIfTouched Order. The
    /// MarketIfTouched Order will only be filled by a market price that crosses
    /// this price from the direction of the market price at the time when the
    /// Order was created (the initialMarketPrice). Depending on the value of
    /// the Order's price and initialMarketPrice, the MarketIfTouchedOrder will
    /// behave like a Limit or a Stop Order.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The worst market price that may be used to fill this MarketIfTouched
    /// Order.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketIfTouchedOrderTimeInForce>,

    /// The date/time when the MarketIfTouched Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketIfTouchedOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A MarketIfTouchedOrderRejectTransaction represents the rejection of the
/// creation of a MarketIfTouched Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketIfTouchedOrderRejectTransaction {
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

    // type is pinned to "MARKET_IF_TOUCHED_ORDER_REJECT" by the enum wrapper
    /// The MarketIfTouched Order's Instrument.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The quantity requested to be filled by the MarketIfTouched Order. A
    /// posititive number of units results in a long Order, and a negative
    /// number of units results in a short Order.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<DecimalNumber>,

    /// The price threshold specified for the MarketIfTouched Order. The
    /// MarketIfTouched Order will only be filled by a market price that crosses
    /// this price from the direction of the market price at the time when the
    /// Order was created (the initialMarketPrice). Depending on the value of
    /// the Order's price and initialMarketPrice, the MarketIfTouchedOrder will
    /// behave like a Limit or a Stop Order.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The worst market price that may be used to fill this MarketIfTouched
    /// Order.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketIfTouchedOrderTimeInForce>,

    /// The date/time when the MarketIfTouched Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `positionFill` field.
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketIfTouchedOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The `takeProfitOnFill` field.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,

    /// The `stopLossOnFill` field.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,

    /// The `trailingStopLossOnFill` field.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,

    /// The `tradeClientExtensions` field.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A TakeProfitOrderTransaction represents the creation of a TakeProfit Order
/// in the user's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakeProfitOrderTransaction {
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

    // type is pinned to "TAKE_PROFIT_ORDER" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price threshold specified for the TakeProfit Order. The associated
    /// Trade will be closed by a market price that is equal to or better than
    /// this threshold.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TakeProfitOrderTimeInForce>,

    /// The date/time when the TakeProfit Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<TakeProfitOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,
}

/// A TakeProfitOrderRejectTransaction represents the rejection of the creation
/// of a TakeProfit Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakeProfitOrderRejectTransaction {
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

    // type is pinned to "TAKE_PROFIT_ORDER_REJECT" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price threshold specified for the TakeProfit Order. The associated
    /// Trade will be closed by a market price that is equal to or better than
    /// this threshold.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TakeProfitOrderTimeInForce>,

    /// The date/time when the TakeProfit Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<TakeProfitOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

/// A StopLossOrderTransaction represents the creation of a StopLoss Order in
/// the user's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopLossOrderTransaction {
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

    // type is pinned to "STOP_LOSS_ORDER" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price threshold specified for the Stop Loss Order. If the guaranteed
    /// flag is false, the associated Trade will be closed by a market price
    /// that is equal to or worse than this threshold. If the flag is true the
    /// associated Trade will be closed at this price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// Specifies the distance (in price units) from the Account's current price
    /// to use as the Stop Loss Order price. If the Trade is short the
    /// Instrument's bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopLossOrderTimeInForce>,

    /// The date/time when the StopLoss Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// Flag indicating that the Stop Loss Order is guaranteed. The default
    /// value depends on the GuaranteedStopLossOrderMode of the account, if it
    /// is REQUIRED, the default will be true, for DISABLED or ENABLED the
    /// default is false.
    #[serde(rename = "guaranteed", skip_serializing_if = "Option::is_none")]
    pub guaranteed: Option<bool>,

    /// The fee that will be charged if the Stop Loss Order is guaranteed and
    /// the Order is filled at the guaranteed price. The value is determined at
    /// Order creation time. It is in price units and is charged for each unit
    /// of the Trade.
    #[serde(
        rename = "guaranteedExecutionPremium",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_premium: Option<DecimalNumber>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopLossOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// The price trigger mode of the Order. Observed value: "TOP_OF_BOOK". This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// A StopLossOrderRejectTransaction represents the rejection of the creation of
/// a StopLoss Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopLossOrderRejectTransaction {
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

    // type is pinned to "STOP_LOSS_ORDER_REJECT" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price threshold specified for the Stop Loss Order. If the guaranteed
    /// flag is false, the associated Trade will be closed by a market price
    /// that is equal to or worse than this threshold. If the flag is true the
    /// associated Trade will be closed at this price.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// Specifies the distance (in price units) from the Account's current price
    /// to use as the Stop Loss Order price. If the Trade is short the
    /// Instrument's bid price is used, and for long Trades the ask is used.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopLossOrderTimeInForce>,

    /// The date/time when the StopLoss Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// Flag indicating that the Stop Loss Order is guaranteed. The default
    /// value depends on the GuaranteedStopLossOrderMode of the account, if it
    /// is REQUIRED, the default will be true, for DISABLED or ENABLED the
    /// default is false.
    #[serde(rename = "guaranteed", skip_serializing_if = "Option::is_none")]
    pub guaranteed: Option<bool>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<StopLossOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,

    /// The price trigger mode of the Order. Observed value: "TOP_OF_BOOK". This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// A TrailingStopLossOrderTransaction represents the creation of a
/// TrailingStopLoss Order in the user's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TrailingStopLossOrderTransaction {
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

    // type is pinned to "TRAILING_STOP_LOSS_ORDER" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price distance (in price units) specified for the TrailingStopLoss
    /// Order.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TrailingStopLossOrderTimeInForce>,

    /// The date/time when the StopLoss Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<TrailingStopLossOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order replaces (only provided if this
    /// Order replaces an existing Order).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Transaction that cancels the replaced Order (only provided
    /// if this Order replaces an existing Order).
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,
}

/// A TrailingStopLossOrderRejectTransaction represents the rejection of the
/// creation of a TrailingStopLoss Order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TrailingStopLossOrderRejectTransaction {
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

    // type is pinned to "TRAILING_STOP_LOSS_ORDER_REJECT" by the enum wrapper
    /// The ID of the Trade to close when the price threshold is breached.
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade to be closed when the price threshold is
    /// breached.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// The price distance (in price units) specified for the TrailingStopLoss
    /// Order.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TrailingStopLossOrderTimeInForce>,

    /// The date/time when the StopLoss Order will be cancelled if its
    /// timeInForce is "GTD".
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `triggerCondition` field.
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,

    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<TrailingStopLossOrderReason>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// The ID of the OrderFill Transaction that caused this Order to be created
    /// (only provided if this Order was created automatically when another
    /// Order was filled).
    #[serde(
        rename = "orderFillTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_fill_transaction_id: Option<TransactionId>,

    /// The ID of the Order that this Order was intended to replace (only
    /// provided if this Order was intended to replace an existing Order).
    #[serde(
        rename = "intendedReplacesOrderID",
        skip_serializing_if = "Option::is_none"
    )]
    pub intended_replaces_order_id: Option<OrderId>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

string_enum! {
    /// The reason that the Market Order was created
    pub enum MarketOrderReason {
        ClientOrder => "CLIENT_ORDER",
        TradeClose => "TRADE_CLOSE",
        PositionCloseout => "POSITION_CLOSEOUT",
        MarginCloseout => "MARGIN_CLOSEOUT",
        DelayedTradeClose => "DELAYED_TRADE_CLOSE",
    }
}

string_enum! {
    /// The reason that the Fixed Price Order was created
    pub enum FixedPriceOrderReason {
        PlatformAccountMigration => "PLATFORM_ACCOUNT_MIGRATION",
    }
}

string_enum! {
    /// The reason that the Limit Order was initiated
    pub enum LimitOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
    }
}

string_enum! {
    /// The reason that the Stop Order was initiated
    pub enum StopOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
    }
}

string_enum! {
    /// The reason that the Market-if-touched Order was initiated
    pub enum MarketIfTouchedOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
    }
}

string_enum! {
    /// The reason that the Take Profit Order was initiated
    pub enum TakeProfitOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
        OnFill => "ON_FILL",
    }
}

string_enum! {
    /// The reason that the Stop Loss Order was initiated
    pub enum StopLossOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
        OnFill => "ON_FILL",
    }
}

string_enum! {
    /// The reason that the Trailing Stop Loss Order was initiated
    pub enum TrailingStopLossOrderReason {
        ClientOrder => "CLIENT_ORDER",
        Replacement => "REPLACEMENT",
        OnFill => "ON_FILL",
    }
}
