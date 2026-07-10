//! Margin-call, financing, dividend, and housekeeping transaction subtypes,
//! plus stream heartbeats and transaction filters.

use serde::{Deserialize, Serialize};

use crate::models::macros::string_enum;
use crate::models::transaction::MarketOrderReason;
use crate::models::{
    AccountFinancingMode, AccountId, AccountUnits, DateTime, DecimalNumber, HomeConversionFactors,
    InstrumentName, OpenTradeDividendAdjustment, PositionFinancing, RequestId, TradeId,
    TransactionId,
};

/// A MarginCallEnterTransaction is created when an Account enters the margin
/// call state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallEnterTransaction {
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
    // type is pinned to "MARGIN_CALL_ENTER" by the enum wrapper
}

/// A MarginCallExtendTransaction is created when the margin call state for an
/// Account has been extended.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallExtendTransaction {
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

    // type is pinned to "MARGIN_CALL_EXTEND" by the enum wrapper
    /// The number of the extensions to the Account's current margin call that
    /// have been applied. This value will be set to 1 for the first
    /// MarginCallExtend Transaction
    #[serde(rename = "extensionNumber", skip_serializing_if = "Option::is_none")]
    pub extension_number: Option<i64>,
}

/// A MarginCallExitnterTransaction is created when an Account leaves the margin
/// call state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MarginCallExitTransaction {
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

    // type is pinned to "MARGIN_CALL_EXIT" by the enum wrapper
    /// The reason the Margin Call was exited. This field is returned by the
    /// live v20 API but is not present in OANDA's official documentation.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// A DelayedTradeClosure Transaction is created administratively to indicate
/// open trades that should have been closed but weren't because the open
/// trades' instruments were untradeable at the time. Open trades listed in this
/// transaction will be closed once their respective instruments become
/// tradeable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DelayedTradeClosureTransaction {
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

    // type is pinned to "DELAYED_TRADE_CLOSURE" by the enum wrapper
    /// The `reason` field.
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<MarketOrderReason>,

    /// List of Trade ID's identifying the open trades that will be closed when
    /// their respective instruments become tradeable
    #[serde(rename = "tradeIDs", skip_serializing_if = "Option::is_none")]
    pub trade_ids: Option<TradeId>,
}

/// A DailyFinancingTransaction represents the daily payment/collection of
/// financing for an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DailyFinancingTransaction {
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

    // type is pinned to "DAILY_FINANCING" by the enum wrapper
    /// The amount of financing paid/collected for the Account.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The Account's balance after daily financing.
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<AccountUnits>,

    /// The `accountFinancingMode` field.
    #[serde(
        rename = "accountFinancingMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub account_financing_mode: Option<AccountFinancingMode>,

    /// The financing paid/collected for each Position in the Account.
    #[serde(
        rename = "positionFinancings",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub position_financings: Vec<PositionFinancing>,

    /// The total cost of currency conversions for the financing, in the
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

/// A DividendAdjustment Transaction is created administratively to pay or
/// collect dividend adjustment amounts to or from an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DividendAdjustmentTransaction {
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

    // type is pinned to "DIVIDEND_ADJUSTMENT" by the enum wrapper
    /// The name of the instrument for the dividendAdjustment transaction.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The total dividend adjustment amount paid or collected in the Account's
    /// home currency for the Account as a result of applying the
    /// DividendAdjustment Transaction.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The total dividend adjustment amount paid or collected in the
    /// Instrument's quote currency.
    #[serde(
        rename = "quoteDividendAdjustment",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_dividend_adjustment: Option<DecimalNumber>,

    /// The `homeConversionFactors` field.
    #[serde(
        rename = "homeConversionFactors",
        skip_serializing_if = "Option::is_none"
    )]
    pub home_conversion_factors: Option<HomeConversionFactors>,

    /// The Account balance after applying the DividendAdjustment Transaction.
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<DecimalNumber>,

    /// The Open Trades being adjusted.
    #[serde(
        rename = "openTradeDividendAdjustments",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub open_trade_dividend_adjustments: Vec<OpenTradeDividendAdjustment>,

    /// The cost of converting the dividend adjustment to the Account's home
    /// currency. This field is returned by the live v20 API but is not present
    /// in OANDA's official documentation.
    #[serde(rename = "homeConversionCost", skip_serializing_if = "Option::is_none")]
    pub home_conversion_cost: Option<DecimalNumber>,
}

/// A ResetResettablePLTransaction represents the resetting of the Account's
/// resettable PL counters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ResetResettablePLTransaction {
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
    // type is pinned to "RESET_RESETTABLE_PL" by the enum wrapper
}

/// A LiquidityRegenerationSchedule indicates how liquidity that is used when
/// filling an Order for an instrument is regenerated following the fill. A
/// liquidity regeneration schedule will be in effect until the timestamp of its
/// final step, but may be replaced by a schedule created for an Order of the
/// same instrument that is filled while it is still in effect.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiquidityRegenerationSchedule {
    /// The steps in the Liquidity Regeneration Schedule
    #[serde(rename = "steps", default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<LiquidityRegenerationScheduleStep>,
}

/// A liquidity regeneration schedule Step indicates the amount of bid and ask
/// liquidity that is used by the Account at a certain time. These amounts will
/// only change at the timestamp of the following step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LiquidityRegenerationScheduleStep {
    /// The timestamp of the schedule step.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime>,

    /// The amount of bid liquidity used at this step in the schedule.
    #[serde(rename = "bidLiquidityUsed", skip_serializing_if = "Option::is_none")]
    pub bid_liquidity_used: Option<DecimalNumber>,

    /// The amount of ask liquidity used at this step in the schedule.
    #[serde(rename = "askLiquidityUsed", skip_serializing_if = "Option::is_none")]
    pub ask_liquidity_used: Option<DecimalNumber>,
}

string_enum! {
    /// The possible types of a Transaction
    pub enum TransactionType {
        Create => "CREATE",
        Close => "CLOSE",
        Reopen => "REOPEN",
        ClientConfigure => "CLIENT_CONFIGURE",
        ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
        TransferFunds => "TRANSFER_FUNDS",
        TransferFundsReject => "TRANSFER_FUNDS_REJECT",
        MarketOrder => "MARKET_ORDER",
        MarketOrderReject => "MARKET_ORDER_REJECT",
        FixedPriceOrder => "FIXED_PRICE_ORDER",
        LimitOrder => "LIMIT_ORDER",
        LimitOrderReject => "LIMIT_ORDER_REJECT",
        StopOrder => "STOP_ORDER",
        StopOrderReject => "STOP_ORDER_REJECT",
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
        StopLossOrder => "STOP_LOSS_ORDER",
        StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
        OrderFill => "ORDER_FILL",
        OrderCancel => "ORDER_CANCEL",
        OrderCancelReject => "ORDER_CANCEL_REJECT",
        OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
        OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
        TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        MarginCallEnter => "MARGIN_CALL_ENTER",
        MarginCallExtend => "MARGIN_CALL_EXTEND",
        MarginCallExit => "MARGIN_CALL_EXIT",
        DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
        DailyFinancing => "DAILY_FINANCING",
        ResetResettablePl => "RESET_RESETTABLE_PL",
        DividendAdjustment => "DIVIDEND_ADJUSTMENT",
    }
}

string_enum! {
    /// A filter that can be used when fetching Transactions
    pub enum TransactionFilter {
        Order => "ORDER",
        Funding => "FUNDING",
        Admin => "ADMIN",
        Create => "CREATE",
        Close => "CLOSE",
        Reopen => "REOPEN",
        ClientConfigure => "CLIENT_CONFIGURE",
        ClientConfigureReject => "CLIENT_CONFIGURE_REJECT",
        TransferFunds => "TRANSFER_FUNDS",
        TransferFundsReject => "TRANSFER_FUNDS_REJECT",
        MarketOrder => "MARKET_ORDER",
        MarketOrderReject => "MARKET_ORDER_REJECT",
        LimitOrder => "LIMIT_ORDER",
        LimitOrderReject => "LIMIT_ORDER_REJECT",
        StopOrder => "STOP_ORDER",
        StopOrderReject => "STOP_ORDER_REJECT",
        MarketIfTouchedOrder => "MARKET_IF_TOUCHED_ORDER",
        MarketIfTouchedOrderReject => "MARKET_IF_TOUCHED_ORDER_REJECT",
        TakeProfitOrder => "TAKE_PROFIT_ORDER",
        TakeProfitOrderReject => "TAKE_PROFIT_ORDER_REJECT",
        StopLossOrder => "STOP_LOSS_ORDER",
        StopLossOrderReject => "STOP_LOSS_ORDER_REJECT",
        TrailingStopLossOrder => "TRAILING_STOP_LOSS_ORDER",
        TrailingStopLossOrderReject => "TRAILING_STOP_LOSS_ORDER_REJECT",
        OneCancelsAllOrder => "ONE_CANCELS_ALL_ORDER",
        OneCancelsAllOrderReject => "ONE_CANCELS_ALL_ORDER_REJECT",
        OneCancelsAllOrderTriggered => "ONE_CANCELS_ALL_ORDER_TRIGGERED",
        OrderFill => "ORDER_FILL",
        OrderCancel => "ORDER_CANCEL",
        OrderCancelReject => "ORDER_CANCEL_REJECT",
        OrderClientExtensionsModify => "ORDER_CLIENT_EXTENSIONS_MODIFY",
        OrderClientExtensionsModifyReject => "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT",
        TradeClientExtensionsModify => "TRADE_CLIENT_EXTENSIONS_MODIFY",
        TradeClientExtensionsModifyReject => "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT",
        MarginCallEnter => "MARGIN_CALL_ENTER",
        MarginCallExtend => "MARGIN_CALL_EXTEND",
        MarginCallExit => "MARGIN_CALL_EXIT",
        DelayedTradeClosure => "DELAYED_TRADE_CLOSURE",
        DailyFinancing => "DAILY_FINANCING",
        ResetResettablePl => "RESET_RESETTABLE_PL",
    }
}

/// A TransactionHeartbeat object is injected into the Transaction stream to
/// ensure that the HTTP connection remains active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransactionHeartbeat {
    /// The type discriminator, always `HEARTBEAT`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The ID of the most recent Transaction created for the Account
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,

    /// The date/time when the TransactionHeartbeat was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
}

/// A TransactionHeartbeat object is injected into the Transaction stream to
/// ensure that the HTTP connection remains active.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MT4TransactionHeartbeat {
    /// The string "HEARTBEAT"
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// The date/time when the TransactionHeartbeat was created.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime>,
}
