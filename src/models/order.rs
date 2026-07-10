//! Order models: the `Order` union, order request specifications with
//! builders, and their supporting enums and detail structs.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::{
    ClientId, DateTime, DecimalNumber, GuaranteedStopLossOrderLevelRestriction, InstrumentName,
    OrderId, PriceValue, TradeId, TradeState, TransactionId,
};

string_enum! {
    /// The type of the Order.
    pub enum OrderType {
        Market => "MARKET",
        Limit => "LIMIT",
        Stop => "STOP",
        MarketIfTouched => "MARKET_IF_TOUCHED",
        TakeProfit => "TAKE_PROFIT",
        StopLoss => "STOP_LOSS",
        TrailingStopLoss => "TRAILING_STOP_LOSS",
        FixedPrice => "FIXED_PRICE",
    }
}

string_enum! {
    /// The type of the Order.
    pub enum CancellableOrderType {
        Limit => "LIMIT",
        Stop => "STOP",
        MarketIfTouched => "MARKET_IF_TOUCHED",
        TakeProfit => "TAKE_PROFIT",
        StopLoss => "STOP_LOSS",
        TrailingStopLoss => "TRAILING_STOP_LOSS",
    }
}

string_enum! {
    /// The current state of the Order.
    pub enum OrderState {
        Pending => "PENDING",
        Filled => "FILLED",
        Triggered => "TRIGGERED",
        Cancelled => "CANCELLED",
    }
}

string_enum! {
    /// The state to filter the requested Orders by.
    pub enum OrderStateFilter {
        Pending => "PENDING",
        Filled => "FILLED",
        Triggered => "TRIGGERED",
        Cancelled => "CANCELLED",
        All => "ALL",
    }
}

string_enum! {
    /// The time-in-force of an Order. TimeInForce describes how long an Order
    /// should remain pending before being automatically cancelled by the
    /// execution system.
    pub enum TimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// Specification of which price component should be used when determining
    /// if an Order should be triggered and filled. This allows Orders to be
    /// triggered based on the bid, ask, mid, default (ask for buy, bid for
    /// sell) or inverse (ask for sell, bid for buy) price depending on the
    /// desired behaviour. Orders are always filled using their default price
    /// component. This feature is only provided through the REST API. Clients
    /// who choose to specify a non-default trigger condition will not see it
    /// reflected in any of OANDA's proprietary or partner trading platforms,
    /// their transaction history or their account statements. OANDA platforms
    /// always assume that an Order's trigger condition is set to the default
    /// value when indicating the distance from an Order's trigger price, and
    /// will always provide the default trigger condition when creating or
    /// modifying an Order. A special restriction applies when creating a
    /// guaranteed Stop Loss Order. In this case the TriggerCondition value must
    /// either be "DEFAULT", or the "natural" trigger side "DEFAULT" results in.
    /// So for a Stop Loss Order for a long trade valid values are "DEFAULT" and
    /// "BID", and for short trades "DEFAULT" and "ASK" are valid.
    pub enum OrderTriggerCondition {
        Default => "DEFAULT",
        Inverse => "INVERSE",
        Bid => "BID",
        Ask => "ASK",
        Mid => "MID",
    }
}

string_enum! {
    /// Specification of how Positions in the Account are modified when the
    /// Order is filled.
    pub enum OrderPositionFill {
        OpenOnly => "OPEN_ONLY",
        ReduceFirst => "REDUCE_FIRST",
        ReduceOnly => "REDUCE_ONLY",
        Default => "DEFAULT",
    }
}

string_enum! {
    /// The time-in-force requested for the Market Order. Restricted to FOK or
    /// IOC for a MarketOrder.
    pub enum MarketOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the Limit Order.
    pub enum LimitOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the Stop Order.
    pub enum StopOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the MarketIfTouched Order. Restricted to
    /// "GTC", "GFD" and "GTD" for MarketIfTouched Orders.
    pub enum MarketIfTouchedOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the TakeProfit Order. Restricted to
    /// "GTC", "GFD" and "GTD" for TakeProfit Orders.
    pub enum TakeProfitOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the StopLoss Order. Restricted to "GTC",
    /// "GFD" and "GTD" for StopLoss Orders.
    pub enum StopLossOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

string_enum! {
    /// The time-in-force requested for the TrailingStopLoss Order. Restricted
    /// to "GTC", "GFD" and "GTD" for TrailingStopLoss Orders.
    pub enum TrailingStopLossOrderTimeInForce {
        Gtc => "GTC",
        Gtd => "GTD",
        Gfd => "GFD",
        Fok => "FOK",
        Ioc => "IOC",
    }
}

/// A ClientExtensions object allows a client to attach a clientID, tag and
/// comment to Orders and Trades in their Account. Do not set, modify, or delete
/// this field if your account is associated with MT4.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClientExtensions {
    /// The Client ID of the Order/Trade
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A tag associated with the Order/Trade
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// A comment associated with the Order/Trade
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// TakeProfitDetails specifies the details of a Take Profit Order to be created
/// on behalf of a client. This may happen when an Order is filled that opens a
/// Trade requiring a Take Profit, or when a Trade's dependent Take Profit Order
/// is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakeProfitDetails {
    /// The price that the Take Profit Order will be triggered at. Only one of
    /// the price and distance fields may be specified.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// The date when the Take Profit Order will be cancelled on if timeInForce
    /// is GTD.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}

/// StopLossDetails specifies the details of a Stop Loss Order to be created on
/// behalf of a client. This may happen when an Order is filled that opens a
/// Trade requiring a Stop Loss, or when a Trade's dependent Stop Loss Order is
/// modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopLossDetails {
    /// The price that the Stop Loss Order will be triggered at. Only one of the
    /// price and distance fields may be specified.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,

    /// Specifies the distance (in price units) from the Trade's open price to
    /// use as the Stop Loss Order price. Only one of the distance and price
    /// fields may be specified.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// The date when the Stop Loss Order will be cancelled on if timeInForce is
    /// GTD.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    /// Flag indicating that the price for the Stop Loss Order is guaranteed.
    /// The default value depends on the GuaranteedStopLossOrderMode of the
    /// account, if it is REQUIRED, the default will be true, for DISABLED or
    /// ENABLED the default is false.
    #[serde(rename = "guaranteed", skip_serializing_if = "Option::is_none")]
    pub guaranteed: Option<bool>,

    /// The price trigger mode of the Stop Loss Order to be created. Observed
    /// value: "TOP_OF_BOOK". This field is returned by the live v20 API but is
    /// not present in OANDA's official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// TrailingStopLossDetails specifies the details of a Trailing Stop Loss Order
/// to be created on behalf of a client. This may happen when an Order is filled
/// that opens a Trade requiring a Trailing Stop Loss, or when a Trade's
/// dependent Trailing Stop Loss Order is modified directly through the Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TrailingStopLossDetails {
    /// The distance (in price units) from the Trade's fill price that the
    /// Trailing Stop Loss Order will be triggered at.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,

    /// The `timeInForce` field.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,

    /// The date when the Trailing Stop Loss Order will be cancelled on if
    /// timeInForce is GTD.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}

/// Details required by clients creating a Guaranteed Stop Loss Order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuaranteedStopLossOrderEntryData {
    /// The minimum distance allowed between the Trade's fill price and the
    /// configured price for guaranteed Stop Loss Orders created for this
    /// instrument. Specified in price units.
    #[serde(rename = "minimumDistance", skip_serializing_if = "Option::is_none")]
    pub minimum_distance: Option<DecimalNumber>,

    /// The amount that is charged to the account if a guaranteed Stop Loss
    /// Order is triggered and filled. The value is in price units and is
    /// charged for each unit of the Trade.
    #[serde(rename = "premium", skip_serializing_if = "Option::is_none")]
    pub premium: Option<DecimalNumber>,

    /// The `levelRestriction` field.
    #[serde(rename = "levelRestriction", skip_serializing_if = "Option::is_none")]
    pub level_restriction: Option<GuaranteedStopLossOrderLevelRestriction>,
}

/// The dynamic state of an Order. This is only relevant to TrailingStopLoss
/// Orders, as no other Order type has dynamic state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DynamicOrderState {
    /// The Order's ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The Order's calculated trailing stop value.
    #[serde(rename = "trailingStopValue", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_value: Option<PriceValue>,

    /// The distance between the Trailing Stop Loss Order's trailingStopValue
    /// and the current Market Price. This represents the distance (in price
    /// units) of the Order from a triggering price. If the distance could not
    /// be determined, this value will not be set.
    #[serde(rename = "triggerDistance", skip_serializing_if = "Option::is_none")]
    pub trigger_distance: Option<PriceValue>,

    /// True if an exact trigger distance could be calculated. If false, it
    /// means the provided trigger distance is a best estimate. If the distance
    /// could not be determined, this value will not be set.
    #[serde(
        rename = "isTriggerDistanceExact",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_trigger_distance_exact: Option<bool>,
}

/// An OrderIdentifier is used to refer to an Order, and contains both the
/// OrderID and the ClientOrderID.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OrderIdentifier {
    /// The OANDA-assigned Order ID
    #[serde(rename = "orderID", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,

    /// The client-provided client Order ID
    #[serde(rename = "clientOrderID", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
}

/// Representation of how many units of an Instrument are available to be traded
/// by an Order depending on its postionFill option.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnitsAvailable {
    /// The `default` field.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<UnitsAvailableDetails>,

    /// The `reduceFirst` field.
    #[serde(rename = "reduceFirst", skip_serializing_if = "Option::is_none")]
    pub reduce_first: Option<UnitsAvailableDetails>,

    /// The `reduceOnly` field.
    #[serde(rename = "reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<UnitsAvailableDetails>,

    /// The `openOnly` field.
    #[serde(rename = "openOnly", skip_serializing_if = "Option::is_none")]
    pub open_only: Option<UnitsAvailableDetails>,
}

/// Representation of many units of an Instrument are available to be traded for
/// both long and short Orders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnitsAvailableDetails {
    /// The units available for long Orders.
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<DecimalNumber>,

    /// The units available for short Orders.
    #[serde(rename = "short", skip_serializing_if = "Option::is_none")]
    pub short: Option<DecimalNumber>,
}

/// A MarketOrderTradeClose specifies the extensions to a Market Order that has
/// been created specifically to close a Trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderTradeClose {
    /// The ID of the Trade requested to be closed
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The client ID of the Trade requested to be closed
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<String>,

    /// Indication of how much of the Trade to close. Either "ALL", or a
    /// DecimalNumber reflection a partial close of the Trade.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

/// A MarketOrderPositionCloseout specifies the extensions to a Market Order
/// when it has been created to closeout a specific Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderPositionCloseout {
    /// The instrument of the Position being closed out.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// Indication of how much of the Position to close. Either "ALL", or a
    /// DecimalNumber reflection a partial close of the Trade. The DecimalNumber
    /// must always be positive, and represent a number that doesn't exceed the
    /// absolute size of the Position.
    #[serde(rename = "units", skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
}

/// Details for the Market Order extensions specific to a Market Order placed
/// that is part of a Market Order Margin Closeout in a client's account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderMarginCloseout {
    /// The reason the Market Order was created to perform a margin closeout
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderMarginCloseoutReason>,
}

/// Details for the Market Order extensions specific to a Market Order placed
/// with the intent of fully closing a specific open trade that should have
/// already been closed but wasn't due to halted market conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrderDelayedTradeClose {
    /// The ID of the Trade being closed
    #[serde(rename = "tradeID", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<TradeId>,

    /// The Client ID of the Trade being closed
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<TradeId>,

    /// The Transaction ID of the DelayedTradeClosure transaction to which this
    /// Delayed Trade Close belongs to
    #[serde(
        rename = "sourceTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_transaction_id: Option<TransactionId>,
}

/// A MarketOrder is an order that is filled immediately upon creation using the
/// current market price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "MARKET" by the enum wrapper
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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,
}

/// A LimitOrder is an order that is created with a price threshold, and will
/// only be filled by a price that is equal to or better than the threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LimitOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "LIMIT" by the enum wrapper
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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A StopOrder is an order that is created with a price threshold, and will
/// only be filled by a price that is equal to or worse than the threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "STOP" by the enum wrapper
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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,

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

/// A MarketIfTouchedOrder is an order that is created with a price threshold,
/// and will only be filled by a market price that is touches or crosses the
/// threshold.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarketIfTouchedOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "MARKET_IF_TOUCHED" by the enum wrapper
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

    /// The Market price at the time when the MarketIfTouched Order was created.
    #[serde(rename = "initialMarketPrice", skip_serializing_if = "Option::is_none")]
    pub initial_market_price: Option<PriceValue>,

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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,

    /// How the Order may be partially filled. Observed value: "DEFAULT_FILL".
    /// This field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "partialFill", skip_serializing_if = "Option::is_none")]
    pub partial_fill: Option<String>,
}

/// A TakeProfitOrder is an order that is linked to an open Trade and created
/// with a price threshold. The Order will be filled (closing the Trade) by the
/// first price that is equal to or better than the threshold. A TakeProfitOrder
/// cannot be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TakeProfitOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "TAKE_PROFIT" by the enum wrapper
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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,
}

/// A StopLossOrder is an order that is linked to an open Trade and created with
/// a price threshold. The Order will be filled (closing the Trade) by the first
/// price that is equal to or worse than the threshold. A StopLossOrder cannot
/// be used to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StopLossOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "STOP_LOSS" by the enum wrapper
    /// The premium that will be charged if the Stop Loss Order is guaranteed
    /// and the Order is filled at the guaranteed price. It is in price units
    /// and is charged for each unit of the Trade.
    #[serde(
        rename = "guaranteedExecutionPremium",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_premium: Option<DecimalNumber>,

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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,

    /// The price trigger mode of the Order. Observed value: "TOP_OF_BOOK". This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "triggerMode", skip_serializing_if = "Option::is_none")]
    pub trigger_mode: Option<String>,
}

/// A TrailingStopLossOrder is an order that is linked to an open Trade and
/// created with a price distance. The price distance is used to calculate a
/// trailing stop value for the order that is in the losing direction from the
/// market price at the time of the order's creation. The trailing stop value
/// will follow the market price as it moves in the winning direction, and the
/// order will filled (closing the Trade) by the first price that is equal to or
/// worse than the trailing stop value. A TrailingStopLossOrder cannot be used
/// to open a new Position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TrailingStopLossOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "TRAILING_STOP_LOSS" by the enum wrapper
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

    /// The trigger price for the Trailing Stop Loss Order. The trailing stop
    /// value will trail (follow) the market price by the TSL order's configured
    /// "distance" as the market price moves in the winning direction. If the
    /// market price moves to a level that is equal to or worse than the
    /// trailing stop value, the order will be filled and the Trade will be
    /// closed.
    #[serde(rename = "trailingStopValue", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_value: Option<PriceValue>,

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,

    /// The ID of the Order that was replaced by this Order (only provided if
    /// this Order was created as part of a cancel/replace).
    #[serde(rename = "replacesOrderID", skip_serializing_if = "Option::is_none")]
    pub replaces_order_id: Option<OrderId>,

    /// The ID of the Order that replaced this Order (only provided if this
    /// Order was cancelled as part of a cancel/replace).
    #[serde(rename = "replacedByOrderID", skip_serializing_if = "Option::is_none")]
    pub replaced_by_order_id: Option<OrderId>,
}

/// A FixedPriceOrder is an order that is filled immediately upon creation using
/// a fixed price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FixedPriceOrder {
    /// The Order's identifier, unique within the Order's Account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrderId>,

    /// The time when the Order was created.
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime>,

    /// The `state` field.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<OrderState>,

    /// The `clientExtensions` field.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,

    // type is pinned to "FIXED_PRICE" by the enum wrapper
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

    /// ID of the Transaction that filled this Order (only provided when the
    /// Order's state is FILLED)
    #[serde(
        rename = "fillingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub filling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was filled (only provided when the Order's
    /// state is FILLED)
    #[serde(rename = "filledTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<DateTime>,

    /// Trade ID of Trade opened when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was opened as a result of the
    /// fill)
    #[serde(rename = "tradeOpenedID", skip_serializing_if = "Option::is_none")]
    pub trade_opened_id: Option<TradeId>,

    /// Trade ID of Trade reduced when the Order was filled (only provided when
    /// the Order's state is FILLED and a Trade was reduced as a result of the
    /// fill)
    #[serde(rename = "tradeReducedID", skip_serializing_if = "Option::is_none")]
    pub trade_reduced_id: Option<TradeId>,

    /// Trade IDs of Trades closed when the Order was filled (only provided when
    /// the Order's state is FILLED and one or more Trades were closed as a
    /// result of the fill)
    #[serde(
        rename = "tradeClosedIDs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trade_closed_ids: Vec<TradeId>,

    /// ID of the Transaction that cancelled the Order (only provided when the
    /// Order's state is CANCELLED)
    #[serde(
        rename = "cancellingTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub cancelling_transaction_id: Option<TransactionId>,

    /// Date/time when the Order was cancelled (only provided when the state of
    /// the Order is CANCELLED)
    #[serde(rename = "cancelledTime", skip_serializing_if = "Option::is_none")]
    pub cancelled_time: Option<DateTime>,
}

string_enum! {
    /// The reason that the Market Order was created to perform a margin
    /// closeout.
    pub enum MarketOrderMarginCloseoutReason {
        MarginCheckViolation => "MARGIN_CHECK_VIOLATION",
        RegulatoryMarginCallViolation => "REGULATORY_MARGIN_CALL_VIOLATION",
        RegulatoryMarginCheckViolation => "REGULATORY_MARGIN_CHECK_VIOLATION",
    }
}

/// Internal helper running the same expression against the inner value of
/// every known [`Order`] variant.
macro_rules! for_each_order {
    ($order:expr, $inner:ident => $body:expr, $unknown:expr) => {
        match $order {
            Order::Market($inner) => $body,
            Order::Limit($inner) => $body,
            Order::Stop($inner) => $body,
            Order::MarketIfTouched($inner) => $body,
            Order::TakeProfit($inner) => $body,
            Order::StopLoss($inner) => $body,
            Order::TrailingStopLoss($inner) => $body,
            Order::FixedPrice($inner) => $body,
            Order::Unknown(_) => $unknown,
        }
    };
}

/// An order in an account, discriminated by its `type` field.
///
/// Orders of a type unknown to this SDK deserialize into
/// [`Order::Unknown`] with the raw JSON preserved, so new order types on
/// OANDA's side never break deserialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Order {
    /// A market order (`MARKET`).
    #[serde(rename = "MARKET")]
    Market(MarketOrder),
    /// A limit order (`LIMIT`).
    #[serde(rename = "LIMIT")]
    Limit(LimitOrder),
    /// A stop order (`STOP`).
    #[serde(rename = "STOP")]
    Stop(StopOrder),
    /// A market-if-touched order (`MARKET_IF_TOUCHED`).
    #[serde(rename = "MARKET_IF_TOUCHED")]
    MarketIfTouched(MarketIfTouchedOrder),
    /// A take-profit order (`TAKE_PROFIT`).
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit(TakeProfitOrder),
    /// A stop-loss order (`STOP_LOSS`).
    #[serde(rename = "STOP_LOSS")]
    StopLoss(StopLossOrder),
    /// A trailing stop-loss order (`TRAILING_STOP_LOSS`).
    #[serde(rename = "TRAILING_STOP_LOSS")]
    TrailingStopLoss(TrailingStopLossOrder),
    /// A fixed-price order (`FIXED_PRICE`).
    #[serde(rename = "FIXED_PRICE")]
    FixedPrice(FixedPriceOrder),
    /// An order of a type not (yet) known to this SDK; the raw JSON object
    /// is preserved.
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

impl Order {
    /// The order's identifier, when known.
    pub fn id(&self) -> Option<&OrderId> {
        for_each_order!(self, o => o.id.as_ref(), None)
    }

    /// The time the order was created, when known.
    pub fn create_time(&self) -> Option<&DateTime> {
        for_each_order!(self, o => o.create_time.as_ref(), None)
    }

    /// The current state of the order, when known.
    pub fn state(&self) -> Option<&OrderState> {
        for_each_order!(self, o => o.state.as_ref(), None)
    }

    /// The client extensions of the order, when known.
    pub fn client_extensions(&self) -> Option<&ClientExtensions> {
        for_each_order!(self, o => o.client_extensions.as_ref(), None)
    }

    /// The wire name of the order's type (e.g. `MARKET`), or the raw
    /// `type` value for unknown orders.
    pub fn type_name(&self) -> Option<&str> {
        match self {
            Order::Market(_) => Some("MARKET"),
            Order::Limit(_) => Some("LIMIT"),
            Order::Stop(_) => Some("STOP"),
            Order::MarketIfTouched(_) => Some("MARKET_IF_TOUCHED"),
            Order::TakeProfit(_) => Some("TAKE_PROFIT"),
            Order::StopLoss(_) => Some("STOP_LOSS"),
            Order::TrailingStopLoss(_) => Some("TRAILING_STOP_LOSS"),
            Order::FixedPrice(_) => Some("FIXED_PRICE"),
            Order::Unknown(value) => value.get("type").and_then(serde_json::Value::as_str),
        }
    }
}

/// The specification of an order to create, discriminated by its `type`
/// field. Built from one of the typed request structs:
///
/// ```
/// use oanda_rs::models::{MarketOrderRequest, OrderRequest, TakeProfitDetails};
///
/// let order: OrderRequest = MarketOrderRequest::new("EUR_USD", 100)
///     .take_profit_on_fill(TakeProfitDetails::at_price("1.1050".parse().unwrap()))
///     .into();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum OrderRequest {
    /// Create a market order (`MARKET`).
    #[serde(rename = "MARKET")]
    Market(MarketOrderRequest),
    /// Create a limit order (`LIMIT`).
    #[serde(rename = "LIMIT")]
    Limit(LimitOrderRequest),
    /// Create a stop order (`STOP`).
    #[serde(rename = "STOP")]
    Stop(StopOrderRequest),
    /// Create a market-if-touched order (`MARKET_IF_TOUCHED`).
    #[serde(rename = "MARKET_IF_TOUCHED")]
    MarketIfTouched(MarketIfTouchedOrderRequest),
    /// Create a take-profit order (`TAKE_PROFIT`).
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit(TakeProfitOrderRequest),
    /// Create a stop-loss order (`STOP_LOSS`).
    #[serde(rename = "STOP_LOSS")]
    StopLoss(StopLossOrderRequest),
    /// Create a trailing stop-loss order (`TRAILING_STOP_LOSS`).
    #[serde(rename = "TRAILING_STOP_LOSS")]
    TrailingStopLoss(TrailingStopLossOrderRequest),
}

macro_rules! order_request_from {
    ($($struct:ident => $variant:ident,)+) => {
        $(
            impl From<$struct> for OrderRequest {
                fn from(request: $struct) -> Self {
                    OrderRequest::$variant(request)
                }
            }
        )+
    };
}

order_request_from! {
    MarketOrderRequest => Market,
    LimitOrderRequest => Limit,
    StopOrderRequest => Stop,
    MarketIfTouchedOrderRequest => MarketIfTouched,
    TakeProfitOrderRequest => TakeProfit,
    StopLossOrderRequest => StopLoss,
    TrailingStopLossOrderRequest => TrailingStopLoss,
}

/// Generates `with`-style optional-field setters for request builders.
macro_rules! setters {
    ($($(#[$meta:meta])* $name:ident: $ty:ty,)+) => {
        $(
            $(#[$meta])*
            pub fn $name(mut self, value: impl Into<$ty>) -> Self {
                self.$name = Some(value.into());
                self
            }
        )+
    };
}

/// Specification of a market order. Required fields are set by
/// [`MarketOrderRequest::new`]; everything else is optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketOrderRequest {
    /// The market order's instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled. A positive number creates a
    /// long order, a negative number a short order.
    #[serde(rename = "units")]
    pub units: DecimalNumber,
    /// The time-in-force for the order (default `FOK`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketOrderTimeInForce>,
    /// The worst price the client is willing to have the order filled at.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,
    /// How positions in the account are modified when the order fills
    /// (default `DEFAULT`).
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
    /// A take-profit order to create for a trade opened when the order
    /// fills.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// A stop-loss order to create for a trade opened when the order fills.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// A trailing stop-loss order to create for a trade opened when the
    /// order fills.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The client extensions to add to a trade opened when the order fills.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

impl MarketOrderRequest {
    /// A market order for `units` of `instrument` (positive units = long,
    /// negative = short).
    pub fn new(instrument: impl Into<InstrumentName>, units: impl Into<DecimalNumber>) -> Self {
        MarketOrderRequest {
            instrument: instrument.into(),
            units: units.into(),
            time_in_force: None,
            price_bound: None,
            position_fill: None,
            client_extensions: None,
            take_profit_on_fill: None,
            stop_loss_on_fill: None,
            trailing_stop_loss_on_fill: None,
            trade_client_extensions: None,
        }
    }

    setters! {
        /// Sets the time-in-force (default `FOK`).
        time_in_force: MarketOrderTimeInForce,
        /// Sets the worst acceptable fill price.
        price_bound: PriceValue,
        /// Sets how positions are modified on fill (default `DEFAULT`).
        position_fill: OrderPositionFill,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
        /// Attaches a take-profit to the trade opened on fill.
        take_profit_on_fill: TakeProfitDetails,
        /// Attaches a stop-loss to the trade opened on fill.
        stop_loss_on_fill: StopLossDetails,
        /// Attaches a trailing stop-loss to the trade opened on fill.
        trailing_stop_loss_on_fill: TrailingStopLossDetails,
        /// Attaches client extensions to the trade opened on fill.
        trade_client_extensions: ClientExtensions,
    }
}

/// Specification of a limit order. Required fields are set by
/// [`LimitOrderRequest::new`]; everything else is optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitOrderRequest {
    /// The limit order's instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled. A positive number creates a
    /// long order, a negative number a short order.
    #[serde(rename = "units")]
    pub units: DecimalNumber,
    /// The price threshold: the order will only be filled at this price or
    /// better.
    #[serde(rename = "price")]
    pub price: PriceValue,
    /// The time-in-force for the order (default `GTC`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<LimitOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// How positions in the account are modified when the order fills
    /// (default `DEFAULT`).
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
    /// A take-profit order to create for a trade opened when the order
    /// fills.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// A stop-loss order to create for a trade opened when the order fills.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// A trailing stop-loss order to create for a trade opened when the
    /// order fills.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The client extensions to add to a trade opened when the order fills.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

impl LimitOrderRequest {
    /// A limit order for `units` of `instrument` at `price` or better.
    pub fn new(
        instrument: impl Into<InstrumentName>,
        units: impl Into<DecimalNumber>,
        price: impl Into<PriceValue>,
    ) -> Self {
        LimitOrderRequest {
            instrument: instrument.into(),
            units: units.into(),
            price: price.into(),
            time_in_force: None,
            gtd_time: None,
            position_fill: None,
            trigger_condition: None,
            client_extensions: None,
            take_profit_on_fill: None,
            stop_loss_on_fill: None,
            trailing_stop_loss_on_fill: None,
            trade_client_extensions: None,
        }
    }

    setters! {
        /// Sets the time-in-force (default `GTC`).
        time_in_force: LimitOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets how positions are modified on fill (default `DEFAULT`).
        position_fill: OrderPositionFill,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
        /// Attaches a take-profit to the trade opened on fill.
        take_profit_on_fill: TakeProfitDetails,
        /// Attaches a stop-loss to the trade opened on fill.
        stop_loss_on_fill: StopLossDetails,
        /// Attaches a trailing stop-loss to the trade opened on fill.
        trailing_stop_loss_on_fill: TrailingStopLossDetails,
        /// Attaches client extensions to the trade opened on fill.
        trade_client_extensions: ClientExtensions,
    }
}

/// Specification of a stop order. Required fields are set by
/// [`StopOrderRequest::new`]; everything else is optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopOrderRequest {
    /// The stop order's instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled. A positive number creates a
    /// long order, a negative number a short order.
    #[serde(rename = "units")]
    pub units: DecimalNumber,
    /// The price threshold: the order will only be filled at this price or
    /// worse.
    #[serde(rename = "price")]
    pub price: PriceValue,
    /// The worst market price that may be used to fill the order (only
    /// with `TimeInForce` `FOK` or `IOC`).
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,
    /// The time-in-force for the order (default `GTC`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// How positions in the account are modified when the order fills
    /// (default `DEFAULT`).
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
    /// A take-profit order to create for a trade opened when the order
    /// fills.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// A stop-loss order to create for a trade opened when the order fills.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// A trailing stop-loss order to create for a trade opened when the
    /// order fills.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The client extensions to add to a trade opened when the order fills.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

impl StopOrderRequest {
    /// A stop order for `units` of `instrument` triggered at `price`.
    pub fn new(
        instrument: impl Into<InstrumentName>,
        units: impl Into<DecimalNumber>,
        price: impl Into<PriceValue>,
    ) -> Self {
        StopOrderRequest {
            instrument: instrument.into(),
            units: units.into(),
            price: price.into(),
            price_bound: None,
            time_in_force: None,
            gtd_time: None,
            position_fill: None,
            trigger_condition: None,
            client_extensions: None,
            take_profit_on_fill: None,
            stop_loss_on_fill: None,
            trailing_stop_loss_on_fill: None,
            trade_client_extensions: None,
        }
    }

    setters! {
        /// Sets the worst acceptable fill price (with `FOK`/`IOC`).
        price_bound: PriceValue,
        /// Sets the time-in-force (default `GTC`).
        time_in_force: StopOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets how positions are modified on fill (default `DEFAULT`).
        position_fill: OrderPositionFill,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
        /// Attaches a take-profit to the trade opened on fill.
        take_profit_on_fill: TakeProfitDetails,
        /// Attaches a stop-loss to the trade opened on fill.
        stop_loss_on_fill: StopLossDetails,
        /// Attaches a trailing stop-loss to the trade opened on fill.
        trailing_stop_loss_on_fill: TrailingStopLossDetails,
        /// Attaches client extensions to the trade opened on fill.
        trade_client_extensions: ClientExtensions,
    }
}

/// Specification of a market-if-touched order. Required fields are set by
/// [`MarketIfTouchedOrderRequest::new`]; everything else is optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketIfTouchedOrderRequest {
    /// The order's instrument.
    #[serde(rename = "instrument")]
    pub instrument: InstrumentName,
    /// The quantity requested to be filled. A positive number creates a
    /// long order, a negative number a short order.
    #[serde(rename = "units")]
    pub units: DecimalNumber,
    /// The price threshold: the order will only be filled by a market
    /// price that crosses this price from the direction of the market
    /// price at creation time.
    #[serde(rename = "price")]
    pub price: PriceValue,
    /// The worst market price that may be used to fill the order.
    #[serde(rename = "priceBound", skip_serializing_if = "Option::is_none")]
    pub price_bound: Option<PriceValue>,
    /// The time-in-force for the order (default `GTC`; only `GTC`, `GFD`
    /// and `GTD` are supported).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<MarketIfTouchedOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// How positions in the account are modified when the order fills
    /// (default `DEFAULT`).
    #[serde(rename = "positionFill", skip_serializing_if = "Option::is_none")]
    pub position_fill: Option<OrderPositionFill>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
    /// A take-profit order to create for a trade opened when the order
    /// fills.
    #[serde(rename = "takeProfitOnFill", skip_serializing_if = "Option::is_none")]
    pub take_profit_on_fill: Option<TakeProfitDetails>,
    /// A stop-loss order to create for a trade opened when the order fills.
    #[serde(rename = "stopLossOnFill", skip_serializing_if = "Option::is_none")]
    pub stop_loss_on_fill: Option<StopLossDetails>,
    /// A trailing stop-loss order to create for a trade opened when the
    /// order fills.
    #[serde(
        rename = "trailingStopLossOnFill",
        skip_serializing_if = "Option::is_none"
    )]
    pub trailing_stop_loss_on_fill: Option<TrailingStopLossDetails>,
    /// The client extensions to add to a trade opened when the order fills.
    #[serde(
        rename = "tradeClientExtensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub trade_client_extensions: Option<ClientExtensions>,
}

impl MarketIfTouchedOrderRequest {
    /// A market-if-touched order for `units` of `instrument` triggered at
    /// `price`.
    pub fn new(
        instrument: impl Into<InstrumentName>,
        units: impl Into<DecimalNumber>,
        price: impl Into<PriceValue>,
    ) -> Self {
        MarketIfTouchedOrderRequest {
            instrument: instrument.into(),
            units: units.into(),
            price: price.into(),
            price_bound: None,
            time_in_force: None,
            gtd_time: None,
            position_fill: None,
            trigger_condition: None,
            client_extensions: None,
            take_profit_on_fill: None,
            stop_loss_on_fill: None,
            trailing_stop_loss_on_fill: None,
            trade_client_extensions: None,
        }
    }

    setters! {
        /// Sets the worst acceptable fill price.
        price_bound: PriceValue,
        /// Sets the time-in-force (default `GTC`).
        time_in_force: MarketIfTouchedOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets how positions are modified on fill (default `DEFAULT`).
        position_fill: OrderPositionFill,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
        /// Attaches a take-profit to the trade opened on fill.
        take_profit_on_fill: TakeProfitDetails,
        /// Attaches a stop-loss to the trade opened on fill.
        stop_loss_on_fill: StopLossDetails,
        /// Attaches a trailing stop-loss to the trade opened on fill.
        trailing_stop_loss_on_fill: TrailingStopLossDetails,
        /// Attaches client extensions to the trade opened on fill.
        trade_client_extensions: ClientExtensions,
    }
}

/// Specification of a take-profit order attached to an existing trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeProfitOrderRequest {
    /// The ID of the trade to close when the order is filled.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeId,
    /// The client ID of the trade to be closed when the order is filled.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<ClientId>,
    /// The price threshold: the associated trade will be closed at this
    /// price or better.
    #[serde(rename = "price")]
    pub price: PriceValue,
    /// The time-in-force for the order (default `GTC`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TakeProfitOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}

impl TakeProfitOrderRequest {
    /// A take-profit order closing `trade_id` at `price` or better.
    pub fn new(trade_id: impl Into<TradeId>, price: impl Into<PriceValue>) -> Self {
        TakeProfitOrderRequest {
            trade_id: trade_id.into(),
            client_trade_id: None,
            price: price.into(),
            time_in_force: None,
            gtd_time: None,
            trigger_condition: None,
            client_extensions: None,
        }
    }

    setters! {
        /// Targets the trade by its client-provided ID instead.
        client_trade_id: ClientId,
        /// Sets the time-in-force (default `GTC`).
        time_in_force: TakeProfitOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
    }
}

/// Specification of a stop-loss order attached to an existing trade.
/// Exactly one of `price` or `distance` must be set (see
/// [`StopLossOrderRequest::at_price`] and
/// [`StopLossOrderRequest::at_distance`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopLossOrderRequest {
    /// The ID of the trade to close when the order is filled.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeId,
    /// The client ID of the trade to be closed when the order is filled.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<ClientId>,
    /// The price threshold: the associated trade will be closed by a
    /// market price that is equal to or worse than this threshold.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceValue>,
    /// The distance (in price units) from the trade's open price to use as
    /// the stop-loss price instead of an absolute `price`.
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<DecimalNumber>,
    /// The time-in-force for the order (default `GTC`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<StopLossOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// Deprecated: whether the order is guaranteed (only allowed when the
    /// account's `guaranteedStopLossOrderMode` permits it).
    #[serde(rename = "guaranteed", skip_serializing_if = "Option::is_none")]
    pub guaranteed: Option<bool>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}

impl StopLossOrderRequest {
    /// A stop-loss order closing `trade_id` at the absolute price
    /// threshold `price`.
    pub fn at_price(trade_id: impl Into<TradeId>, price: impl Into<PriceValue>) -> Self {
        StopLossOrderRequest {
            trade_id: trade_id.into(),
            client_trade_id: None,
            price: Some(price.into()),
            distance: None,
            time_in_force: None,
            gtd_time: None,
            trigger_condition: None,
            guaranteed: None,
            client_extensions: None,
        }
    }

    /// A stop-loss order closing `trade_id` at `distance` price units from
    /// the trade's open price.
    pub fn at_distance(trade_id: impl Into<TradeId>, distance: impl Into<DecimalNumber>) -> Self {
        StopLossOrderRequest {
            trade_id: trade_id.into(),
            client_trade_id: None,
            price: None,
            distance: Some(distance.into()),
            time_in_force: None,
            gtd_time: None,
            trigger_condition: None,
            guaranteed: None,
            client_extensions: None,
        }
    }

    setters! {
        /// Targets the trade by its client-provided ID instead.
        client_trade_id: ClientId,
        /// Sets the time-in-force (default `GTC`).
        time_in_force: StopLossOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
    }
}

/// Specification of a trailing stop-loss order attached to an existing
/// trade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrailingStopLossOrderRequest {
    /// The ID of the trade to close when the order is filled.
    #[serde(rename = "tradeID")]
    pub trade_id: TradeId,
    /// The client ID of the trade to be closed when the order is filled.
    #[serde(rename = "clientTradeID", skip_serializing_if = "Option::is_none")]
    pub client_trade_id: Option<ClientId>,
    /// The distance (in price units) from the trade's fill price that the
    /// trailing stop-loss trails.
    #[serde(rename = "distance")]
    pub distance: DecimalNumber,
    /// The time-in-force for the order (default `GTC`).
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TrailingStopLossOrderTimeInForce>,
    /// The date/time when the order is cancelled if `time_in_force` is
    /// `GTD`.
    #[serde(rename = "gtdTime", skip_serializing_if = "Option::is_none")]
    pub gtd_time: Option<DateTime>,
    /// Which price component is used to trigger the order (default
    /// `DEFAULT`).
    #[serde(rename = "triggerCondition", skip_serializing_if = "Option::is_none")]
    pub trigger_condition: Option<OrderTriggerCondition>,
    /// The client extensions to add to the order.
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}

impl TrailingStopLossOrderRequest {
    /// A trailing stop-loss order for `trade_id` trailing at `distance`
    /// price units.
    pub fn new(trade_id: impl Into<TradeId>, distance: impl Into<DecimalNumber>) -> Self {
        TrailingStopLossOrderRequest {
            trade_id: trade_id.into(),
            client_trade_id: None,
            distance: distance.into(),
            time_in_force: None,
            gtd_time: None,
            trigger_condition: None,
            client_extensions: None,
        }
    }

    setters! {
        /// Targets the trade by its client-provided ID instead.
        client_trade_id: ClientId,
        /// Sets the time-in-force (default `GTC`).
        time_in_force: TrailingStopLossOrderTimeInForce,
        /// Sets the cancellation time used with `GTD`.
        gtd_time: DateTime,
        /// Sets the price component used for triggering.
        trigger_condition: OrderTriggerCondition,
        /// Attaches client extensions to the order.
        client_extensions: ClientExtensions,
    }
}

impl ClientExtensions {
    /// Empty client extensions to fill via the setters.
    pub fn new() -> Self {
        ClientExtensions {
            id: None,
            tag: None,
            comment: None,
        }
    }

    /// Sets the client-provided ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the client-provided tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Sets the client-provided comment.
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }
}

impl Default for ClientExtensions {
    fn default() -> Self {
        ClientExtensions::new()
    }
}

impl TakeProfitDetails {
    /// Take-profit details closing the trade at `price` or better.
    pub fn at_price(price: PriceValue) -> Self {
        TakeProfitDetails {
            price: Some(price),
            time_in_force: None,
            gtd_time: None,
            client_extensions: None,
        }
    }
}

impl StopLossDetails {
    /// Stop-loss details closing the trade at the absolute `price`
    /// threshold.
    pub fn at_price(price: PriceValue) -> Self {
        StopLossDetails {
            price: Some(price),
            distance: None,
            time_in_force: None,
            gtd_time: None,
            client_extensions: None,
            guaranteed: None,
            trigger_mode: None,
        }
    }

    /// Stop-loss details closing the trade at `distance` price units from
    /// its open price.
    pub fn at_distance(distance: DecimalNumber) -> Self {
        StopLossDetails {
            price: None,
            distance: Some(distance),
            time_in_force: None,
            gtd_time: None,
            client_extensions: None,
            guaranteed: None,
            trigger_mode: None,
        }
    }
}

impl TrailingStopLossDetails {
    /// Trailing stop-loss details trailing at `distance` price units.
    pub fn at_distance(distance: DecimalNumber) -> Self {
        TrailingStopLossDetails {
            distance: Some(distance),
            time_in_force: None,
            gtd_time: None,
            client_extensions: None,
        }
    }
}
