//! Transaction models: the `Transaction` discriminated union (36 subtypes)
//! and its supporting enums.

mod account_tx;
mod fill_cancel_tx;
mod misc_tx;
mod order_tx;

pub use account_tx::*;
pub use fill_cancel_tx::*;
pub use misc_tx::*;
pub use order_tx::*;

use serde::{Deserialize, Serialize};

use crate::models::{AccountId, DateTime, RequestId, TransactionId};

/// Internal helper running the same expression against the inner value of
/// every known [`Transaction`] variant.
macro_rules! for_each_transaction {
    ($tx:expr, $inner:ident => $body:expr, $unknown:expr) => {
        match $tx {
            Transaction::Create($inner) => $body,
            Transaction::Close($inner) => $body,
            Transaction::Reopen($inner) => $body,
            Transaction::ClientConfigure($inner) => $body,
            Transaction::ClientConfigureReject($inner) => $body,
            Transaction::TransferFunds($inner) => $body,
            Transaction::TransferFundsReject($inner) => $body,
            Transaction::MarketOrder($inner) => $body,
            Transaction::MarketOrderReject($inner) => $body,
            Transaction::FixedPriceOrder($inner) => $body,
            Transaction::LimitOrder($inner) => $body,
            Transaction::LimitOrderReject($inner) => $body,
            Transaction::StopOrder($inner) => $body,
            Transaction::StopOrderReject($inner) => $body,
            Transaction::MarketIfTouchedOrder($inner) => $body,
            Transaction::MarketIfTouchedOrderReject($inner) => $body,
            Transaction::TakeProfitOrder($inner) => $body,
            Transaction::TakeProfitOrderReject($inner) => $body,
            Transaction::StopLossOrder($inner) => $body,
            Transaction::StopLossOrderReject($inner) => $body,
            Transaction::TrailingStopLossOrder($inner) => $body,
            Transaction::TrailingStopLossOrderReject($inner) => $body,
            Transaction::OrderFill($inner) => $body,
            Transaction::OrderCancel($inner) => $body,
            Transaction::OrderCancelReject($inner) => $body,
            Transaction::OrderClientExtensionsModify($inner) => $body,
            Transaction::OrderClientExtensionsModifyReject($inner) => $body,
            Transaction::TradeClientExtensionsModify($inner) => $body,
            Transaction::TradeClientExtensionsModifyReject($inner) => $body,
            Transaction::MarginCallEnter($inner) => $body,
            Transaction::MarginCallExtend($inner) => $body,
            Transaction::MarginCallExit($inner) => $body,
            Transaction::DelayedTradeClosure($inner) => $body,
            Transaction::DailyFinancing($inner) => $body,
            Transaction::ResetResettablePL($inner) => $body,
            Transaction::DividendAdjustment($inner) => $body,
            Transaction::Unknown(_) => $unknown,
        }
    };
}

/// A single change to an account's state, discriminated by its `type`
/// field.
///
/// Transactions of a type unknown to this SDK deserialize into
/// [`Transaction::Unknown`] with the raw JSON preserved, so new
/// transaction types on OANDA's side never break deserialization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)] // variants mirror the wire format; boxing would hurt ergonomics
pub enum Transaction {
    /// A `CREATE` transaction.
    #[serde(rename = "CREATE")]
    Create(CreateTransaction),
    /// A `CLOSE` transaction.
    #[serde(rename = "CLOSE")]
    Close(CloseTransaction),
    /// A `REOPEN` transaction.
    #[serde(rename = "REOPEN")]
    Reopen(ReopenTransaction),
    /// A `CLIENT_CONFIGURE` transaction.
    #[serde(rename = "CLIENT_CONFIGURE")]
    ClientConfigure(ClientConfigureTransaction),
    /// A `CLIENT_CONFIGURE_REJECT` transaction.
    #[serde(rename = "CLIENT_CONFIGURE_REJECT")]
    ClientConfigureReject(ClientConfigureRejectTransaction),
    /// A `TRANSFER_FUNDS` transaction.
    #[serde(rename = "TRANSFER_FUNDS")]
    TransferFunds(TransferFundsTransaction),
    /// A `TRANSFER_FUNDS_REJECT` transaction.
    #[serde(rename = "TRANSFER_FUNDS_REJECT")]
    TransferFundsReject(TransferFundsRejectTransaction),
    /// A `MARKET_ORDER` transaction.
    #[serde(rename = "MARKET_ORDER")]
    MarketOrder(MarketOrderTransaction),
    /// A `MARKET_ORDER_REJECT` transaction.
    #[serde(rename = "MARKET_ORDER_REJECT")]
    MarketOrderReject(MarketOrderRejectTransaction),
    /// A `FIXED_PRICE_ORDER` transaction.
    #[serde(rename = "FIXED_PRICE_ORDER")]
    FixedPriceOrder(FixedPriceOrderTransaction),
    /// A `LIMIT_ORDER` transaction.
    #[serde(rename = "LIMIT_ORDER")]
    LimitOrder(LimitOrderTransaction),
    /// A `LIMIT_ORDER_REJECT` transaction.
    #[serde(rename = "LIMIT_ORDER_REJECT")]
    LimitOrderReject(LimitOrderRejectTransaction),
    /// A `STOP_ORDER` transaction.
    #[serde(rename = "STOP_ORDER")]
    StopOrder(StopOrderTransaction),
    /// A `STOP_ORDER_REJECT` transaction.
    #[serde(rename = "STOP_ORDER_REJECT")]
    StopOrderReject(StopOrderRejectTransaction),
    /// A `MARKET_IF_TOUCHED_ORDER` transaction.
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER")]
    MarketIfTouchedOrder(MarketIfTouchedOrderTransaction),
    /// A `MARKET_IF_TOUCHED_ORDER_REJECT` transaction.
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER_REJECT")]
    MarketIfTouchedOrderReject(MarketIfTouchedOrderRejectTransaction),
    /// A `TAKE_PROFIT_ORDER` transaction.
    #[serde(rename = "TAKE_PROFIT_ORDER")]
    TakeProfitOrder(TakeProfitOrderTransaction),
    /// A `TAKE_PROFIT_ORDER_REJECT` transaction.
    #[serde(rename = "TAKE_PROFIT_ORDER_REJECT")]
    TakeProfitOrderReject(TakeProfitOrderRejectTransaction),
    /// A `STOP_LOSS_ORDER` transaction.
    #[serde(rename = "STOP_LOSS_ORDER")]
    StopLossOrder(StopLossOrderTransaction),
    /// A `STOP_LOSS_ORDER_REJECT` transaction.
    #[serde(rename = "STOP_LOSS_ORDER_REJECT")]
    StopLossOrderReject(StopLossOrderRejectTransaction),
    /// A `TRAILING_STOP_LOSS_ORDER` transaction.
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER")]
    TrailingStopLossOrder(TrailingStopLossOrderTransaction),
    /// A `TRAILING_STOP_LOSS_ORDER_REJECT` transaction.
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER_REJECT")]
    TrailingStopLossOrderReject(TrailingStopLossOrderRejectTransaction),
    /// A `ORDER_FILL` transaction.
    #[serde(rename = "ORDER_FILL")]
    OrderFill(OrderFillTransaction),
    /// A `ORDER_CANCEL` transaction.
    #[serde(rename = "ORDER_CANCEL")]
    OrderCancel(OrderCancelTransaction),
    /// A `ORDER_CANCEL_REJECT` transaction.
    #[serde(rename = "ORDER_CANCEL_REJECT")]
    OrderCancelReject(OrderCancelRejectTransaction),
    /// A `ORDER_CLIENT_EXTENSIONS_MODIFY` transaction.
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY")]
    OrderClientExtensionsModify(OrderClientExtensionsModifyTransaction),
    /// A `ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT` transaction.
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    OrderClientExtensionsModifyReject(OrderClientExtensionsModifyRejectTransaction),
    /// A `TRADE_CLIENT_EXTENSIONS_MODIFY` transaction.
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY")]
    TradeClientExtensionsModify(TradeClientExtensionsModifyTransaction),
    /// A `TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT` transaction.
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    TradeClientExtensionsModifyReject(TradeClientExtensionsModifyRejectTransaction),
    /// A `MARGIN_CALL_ENTER` transaction.
    #[serde(rename = "MARGIN_CALL_ENTER")]
    MarginCallEnter(MarginCallEnterTransaction),
    /// A `MARGIN_CALL_EXTEND` transaction.
    #[serde(rename = "MARGIN_CALL_EXTEND")]
    MarginCallExtend(MarginCallExtendTransaction),
    /// A `MARGIN_CALL_EXIT` transaction.
    #[serde(rename = "MARGIN_CALL_EXIT")]
    MarginCallExit(MarginCallExitTransaction),
    /// A `DELAYED_TRADE_CLOSURE` transaction.
    #[serde(rename = "DELAYED_TRADE_CLOSURE")]
    DelayedTradeClosure(DelayedTradeClosureTransaction),
    /// A `DAILY_FINANCING` transaction.
    #[serde(rename = "DAILY_FINANCING")]
    DailyFinancing(DailyFinancingTransaction),
    /// A `RESET_RESETTABLE_PL` transaction.
    #[serde(rename = "RESET_RESETTABLE_PL")]
    ResetResettablePL(ResetResettablePLTransaction),
    /// A `DIVIDEND_ADJUSTMENT` transaction.
    #[serde(rename = "DIVIDEND_ADJUSTMENT")]
    DividendAdjustment(DividendAdjustmentTransaction),
    /// A transaction of a type not (yet) known to this SDK; the raw JSON
    /// object is preserved.
    #[serde(untagged)]
    Unknown(serde_json::Value),
}

impl Transaction {
    /// The transaction's `id`, when known.
    pub fn id(&self) -> Option<&TransactionId> {
        for_each_transaction!(self, t => t.id.as_ref(), None)
    }

    /// The transaction's `time`, when known.
    pub fn time(&self) -> Option<&DateTime> {
        for_each_transaction!(self, t => t.time.as_ref(), None)
    }

    /// The transaction's `account_id`, when known.
    pub fn account_id(&self) -> Option<&AccountId> {
        for_each_transaction!(self, t => t.account_id.as_ref(), None)
    }

    /// The transaction's `batch_id`, when known.
    pub fn batch_id(&self) -> Option<&TransactionId> {
        for_each_transaction!(self, t => t.batch_id.as_ref(), None)
    }

    /// The transaction's `request_id`, when known.
    pub fn request_id(&self) -> Option<&RequestId> {
        for_each_transaction!(self, t => t.request_id.as_ref(), None)
    }

    /// The ID of the user that initiated the transaction, when known.
    pub fn user_id(&self) -> Option<i64> {
        for_each_transaction!(self, t => t.user_id, None)
    }

    /// The wire name of the transaction's type (e.g. `ORDER_FILL`), or
    /// the raw `type` value for unknown transactions.
    pub fn type_name(&self) -> Option<&str> {
        match self {
            Transaction::Create(_) => Some("CREATE"),
            Transaction::Close(_) => Some("CLOSE"),
            Transaction::Reopen(_) => Some("REOPEN"),
            Transaction::ClientConfigure(_) => Some("CLIENT_CONFIGURE"),
            Transaction::ClientConfigureReject(_) => Some("CLIENT_CONFIGURE_REJECT"),
            Transaction::TransferFunds(_) => Some("TRANSFER_FUNDS"),
            Transaction::TransferFundsReject(_) => Some("TRANSFER_FUNDS_REJECT"),
            Transaction::MarketOrder(_) => Some("MARKET_ORDER"),
            Transaction::MarketOrderReject(_) => Some("MARKET_ORDER_REJECT"),
            Transaction::FixedPriceOrder(_) => Some("FIXED_PRICE_ORDER"),
            Transaction::LimitOrder(_) => Some("LIMIT_ORDER"),
            Transaction::LimitOrderReject(_) => Some("LIMIT_ORDER_REJECT"),
            Transaction::StopOrder(_) => Some("STOP_ORDER"),
            Transaction::StopOrderReject(_) => Some("STOP_ORDER_REJECT"),
            Transaction::MarketIfTouchedOrder(_) => Some("MARKET_IF_TOUCHED_ORDER"),
            Transaction::MarketIfTouchedOrderReject(_) => Some("MARKET_IF_TOUCHED_ORDER_REJECT"),
            Transaction::TakeProfitOrder(_) => Some("TAKE_PROFIT_ORDER"),
            Transaction::TakeProfitOrderReject(_) => Some("TAKE_PROFIT_ORDER_REJECT"),
            Transaction::StopLossOrder(_) => Some("STOP_LOSS_ORDER"),
            Transaction::StopLossOrderReject(_) => Some("STOP_LOSS_ORDER_REJECT"),
            Transaction::TrailingStopLossOrder(_) => Some("TRAILING_STOP_LOSS_ORDER"),
            Transaction::TrailingStopLossOrderReject(_) => Some("TRAILING_STOP_LOSS_ORDER_REJECT"),
            Transaction::OrderFill(_) => Some("ORDER_FILL"),
            Transaction::OrderCancel(_) => Some("ORDER_CANCEL"),
            Transaction::OrderCancelReject(_) => Some("ORDER_CANCEL_REJECT"),
            Transaction::OrderClientExtensionsModify(_) => Some("ORDER_CLIENT_EXTENSIONS_MODIFY"),
            Transaction::OrderClientExtensionsModifyReject(_) => {
                Some("ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT")
            }
            Transaction::TradeClientExtensionsModify(_) => Some("TRADE_CLIENT_EXTENSIONS_MODIFY"),
            Transaction::TradeClientExtensionsModifyReject(_) => {
                Some("TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT")
            }
            Transaction::MarginCallEnter(_) => Some("MARGIN_CALL_ENTER"),
            Transaction::MarginCallExtend(_) => Some("MARGIN_CALL_EXTEND"),
            Transaction::MarginCallExit(_) => Some("MARGIN_CALL_EXIT"),
            Transaction::DelayedTradeClosure(_) => Some("DELAYED_TRADE_CLOSURE"),
            Transaction::DailyFinancing(_) => Some("DAILY_FINANCING"),
            Transaction::ResetResettablePL(_) => Some("RESET_RESETTABLE_PL"),
            Transaction::DividendAdjustment(_) => Some("DIVIDEND_ADJUSTMENT"),
            Transaction::Unknown(value) => value.get("type").and_then(serde_json::Value::as_str),
        }
    }
}
/// A single line of the transaction stream: either a full transaction or a
/// heartbeat (sent every 5 seconds to keep the connection alive).
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)] // variants mirror the wire format; boxing would hurt ergonomics
pub enum TransactionStreamItem {
    /// A transaction applied to the account.
    Transaction(Transaction),
    /// A keep-alive heartbeat (`"type": "HEARTBEAT"`).
    Heartbeat(TransactionHeartbeat),
}

impl serde::Serialize for TransactionStreamItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TransactionStreamItem::Transaction(tx) => tx.serialize(serializer),
            TransactionStreamItem::Heartbeat(heartbeat) => heartbeat.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TransactionStreamItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let is_heartbeat =
            value.get("type").and_then(serde_json::Value::as_str) == Some("HEARTBEAT");
        if is_heartbeat {
            serde_json::from_value(value)
                .map(TransactionStreamItem::Heartbeat)
                .map_err(serde::de::Error::custom)
        } else {
            serde_json::from_value(value)
                .map(TransactionStreamItem::Transaction)
                .map_err(serde::de::Error::custom)
        }
    }
}
