//! Account-related models.

use serde::{Deserialize, Serialize};

use super::macros::string_enum;
use super::transaction::Transaction;
use super::{
    AccountId, AccountUnits, CalculatedPositionState, CalculatedTradeState, Currency, DateTime,
    DecimalNumber, DynamicOrderState, InstrumentName, Order, Position, TradeSummary, TransactionId,
};

/// Properties related to an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountProperties {
    /// The Account's identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountId>,

    /// The Account's associated MT4 Account ID. This field will not be present
    /// if the Account is not an MT4 account.
    #[serde(rename = "mt4AccountID", skip_serializing_if = "Option::is_none")]
    pub mt4_account_id: Option<i64>,

    /// The Account's tags
    #[serde(rename = "tags", default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// A summary representation of a client's Account. The AccountSummary does not
/// provide to full specification of pending Orders, open Trades and Positions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountSummary {
    /// The Account's identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountId>,

    /// Client-assigned alias for the Account. Only provided if the Account has
    /// an alias set
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// The home currency of the Account
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The current balance of the Account.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<AccountUnits>,

    /// ID of the user that created the Account.
    #[serde(rename = "createdByUserID", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<i64>,

    /// The date/time when the Account was created.
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime>,

    /// The `guaranteedStopLossOrderMode` field.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,

    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<AccountUnits>,

    /// The total realized profit/loss for the Account since it was last reset
    /// by the client.
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none")]
    pub resettable_pl: Option<AccountUnits>,

    /// The date/time that the Account's resettablePL was last reset.
    #[serde(rename = "resettablePLTime", skip_serializing_if = "Option::is_none")]
    pub resettable_pl_time: Option<DateTime>,

    /// The total amount of financing paid/collected over the lifetime of the
    /// Account.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<AccountUnits>,

    /// The total amount of fees charged over the lifetime of the Account for
    /// the execution of guaranteed Stop Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<AccountUnits>,

    /// Client-provided margin rate override for the Account. The effective
    /// margin rate of the Account is the lesser of this value and the OANDA
    /// margin rate for the Account's division. This value is only provided if a
    /// margin rate override exists for the Account.
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    pub margin_rate: Option<DecimalNumber>,

    /// The date/time when the Account entered a margin call state. Only
    /// provided if the Account is in a margin call.
    #[serde(
        rename = "marginCallEnterTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<DateTime>,

    /// The number of times that the Account's current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,

    /// The date/time of the Account's last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<DateTime>,

    /// The number of Trades currently open in the Account.
    #[serde(rename = "openTradeCount", skip_serializing_if = "Option::is_none")]
    pub open_trade_count: Option<i64>,

    /// The number of Positions currently open in the Account.
    #[serde(rename = "openPositionCount", skip_serializing_if = "Option::is_none")]
    pub open_position_count: Option<i64>,

    /// The number of Orders currently pending in the Account.
    #[serde(rename = "pendingOrderCount", skip_serializing_if = "Option::is_none")]
    pub pending_order_count: Option<i64>,

    /// Flag indicating that the Account has hedging enabled.
    #[serde(rename = "hedgingEnabled", skip_serializing_if = "Option::is_none")]
    pub hedging_enabled: Option<bool>,

    /// The date/time of the last order that was filled for this account.
    #[serde(
        rename = "lastOrderFillTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_order_fill_timestamp: Option<DateTime>,

    /// The total unrealized profit/loss for all Trades currently open in the
    /// Account.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// The net asset value of the Account. Equal to Account balance +
    /// unrealizedPL.
    #[serde(rename = "NAV", skip_serializing_if = "Option::is_none")]
    pub nav: Option<AccountUnits>,

    /// Margin currently used for the Account.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// Margin available for Account currency.
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<AccountUnits>,

    /// The value of the Account's open positions represented in the Account's
    /// home currency.
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none")]
    pub position_value: Option<AccountUnits>,

    /// The Account's margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<AccountUnits>,

    /// The Account's margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV", skip_serializing_if = "Option::is_none")]
    pub margin_closeout_nav: Option<AccountUnits>,

    /// The Account's margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<AccountUnits>,

    /// The Account's margin closeout percentage. When this value is 1.0 or
    /// above the Account is in a margin closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<DecimalNumber>,

    /// The value of the Account's open positions as used for margin closeout
    /// calculations represented in the Account's home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<DecimalNumber>,

    /// The current WithdrawalLimit for the account which will be zero or a
    /// positive value indicating how much can be withdrawn from the account.
    #[serde(rename = "withdrawalLimit", skip_serializing_if = "Option::is_none")]
    pub withdrawal_limit: Option<AccountUnits>,

    /// The Account's margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<AccountUnits>,

    /// The Account's margin call percentage. When this value is 1.0 or above
    /// the Account is in a margin call situation.
    #[serde(rename = "marginCallPercent", skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<DecimalNumber>,

    /// The ID of the last Transaction created for the Account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,

    /// The total amount of dividend adjustment paid over the lifetime of the
    /// Account in the Account's home currency.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The net asset value of the Account inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueNAV", skip_serializing_if = "Option::is_none")]
    pub true_nav: Option<DecimalNumber>,

    /// The total unrealized profit/loss inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,

    /// The date/time of the last dividend adjustment per instrument. This field
    /// is returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(
        rename = "lastDividendAdjustmentTimestamps",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub last_dividend_adjustment_timestamps: Vec<DividendAdjustmentTimestamp>,
}

string_enum! {
    /// The overall behaviour of the Account regarding guaranteed Stop Loss
    /// Orders.
    pub enum GuaranteedStopLossOrderMode {
        Disabled => "DISABLED",
        Allowed => "ALLOWED",
        Required => "REQUIRED",
    }
}

string_enum! {
    /// The financing mode of an Account
    pub enum AccountFinancingMode {
        NoFinancing => "NO_FINANCING",
        SecondBySecond => "SECOND_BY_SECOND",
        Daily => "DAILY",
        DailyInstrument => "DAILY_INSTRUMENT",
        SecondBySecondInstrument => "SECOND_BY_SECOND_INSTRUMENT",
    }
}

/// The date/time of the last dividend adjustment for a single instrument
/// (entry of `AccountSummary.lastDividendAdjustmentTimestamps`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DividendAdjustmentTimestamp {
    /// The instrument of the dividend adjustment.
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<InstrumentName>,

    /// The date/time of the dividend adjustment.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime>,
}

/// The full details of a client's Account. This includes full open Trade, open
/// Position and pending Order representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Account {
    /// The Account's identifier
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<AccountId>,

    /// Client-assigned alias for the Account. Only provided if the Account has
    /// an alias set
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// The home currency of the Account
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The current balance of the Account.
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<AccountUnits>,

    /// ID of the user that created the Account.
    #[serde(rename = "createdByUserID", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<i64>,

    /// The date/time when the Account was created.
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime>,

    /// The `guaranteedStopLossOrderMode` field.
    #[serde(
        rename = "guaranteedStopLossOrderMode",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_stop_loss_order_mode: Option<GuaranteedStopLossOrderMode>,

    /// The total profit/loss realized over the lifetime of the Account.
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none")]
    pub pl: Option<AccountUnits>,

    /// The total realized profit/loss for the Account since it was last reset
    /// by the client.
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none")]
    pub resettable_pl: Option<AccountUnits>,

    /// The date/time that the Account's resettablePL was last reset.
    #[serde(rename = "resettablePLTime", skip_serializing_if = "Option::is_none")]
    pub resettable_pl_time: Option<DateTime>,

    /// The total amount of financing paid/collected over the lifetime of the
    /// Account.
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<AccountUnits>,

    /// The total amount of commission paid over the lifetime of the Account.
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<AccountUnits>,

    /// The total amount of fees charged over the lifetime of the Account for
    /// the execution of guaranteed Stop Loss Orders.
    #[serde(
        rename = "guaranteedExecutionFees",
        skip_serializing_if = "Option::is_none"
    )]
    pub guaranteed_execution_fees: Option<AccountUnits>,

    /// Client-provided margin rate override for the Account. The effective
    /// margin rate of the Account is the lesser of this value and the OANDA
    /// margin rate for the Account's division. This value is only provided if a
    /// margin rate override exists for the Account.
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    pub margin_rate: Option<DecimalNumber>,

    /// The date/time when the Account entered a margin call state. Only
    /// provided if the Account is in a margin call.
    #[serde(
        rename = "marginCallEnterTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_enter_time: Option<DateTime>,

    /// The number of times that the Account's current margin call was extended.
    #[serde(
        rename = "marginCallExtensionCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_extension_count: Option<i64>,

    /// The date/time of the Account's last margin call extension.
    #[serde(
        rename = "lastMarginCallExtensionTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_margin_call_extension_time: Option<DateTime>,

    /// The number of Trades currently open in the Account.
    #[serde(rename = "openTradeCount", skip_serializing_if = "Option::is_none")]
    pub open_trade_count: Option<i64>,

    /// The number of Positions currently open in the Account.
    #[serde(rename = "openPositionCount", skip_serializing_if = "Option::is_none")]
    pub open_position_count: Option<i64>,

    /// The number of Orders currently pending in the Account.
    #[serde(rename = "pendingOrderCount", skip_serializing_if = "Option::is_none")]
    pub pending_order_count: Option<i64>,

    /// Flag indicating that the Account has hedging enabled.
    #[serde(rename = "hedgingEnabled", skip_serializing_if = "Option::is_none")]
    pub hedging_enabled: Option<bool>,

    /// The date/time of the last order that was filled for this account.
    #[serde(
        rename = "lastOrderFillTimestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_order_fill_timestamp: Option<DateTime>,

    /// The total unrealized profit/loss for all Trades currently open in the
    /// Account.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// The net asset value of the Account. Equal to Account balance +
    /// unrealizedPL.
    #[serde(rename = "NAV", skip_serializing_if = "Option::is_none")]
    pub nav: Option<AccountUnits>,

    /// Margin currently used for the Account.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// Margin available for Account currency.
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<AccountUnits>,

    /// The value of the Account's open positions represented in the Account's
    /// home currency.
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none")]
    pub position_value: Option<AccountUnits>,

    /// The Account's margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<AccountUnits>,

    /// The Account's margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV", skip_serializing_if = "Option::is_none")]
    pub margin_closeout_nav: Option<AccountUnits>,

    /// The Account's margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<AccountUnits>,

    /// The Account's margin closeout percentage. When this value is 1.0 or
    /// above the Account is in a margin closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<DecimalNumber>,

    /// The value of the Account's open positions as used for margin closeout
    /// calculations represented in the Account's home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<DecimalNumber>,

    /// The current WithdrawalLimit for the account which will be zero or a
    /// positive value indicating how much can be withdrawn from the account.
    #[serde(rename = "withdrawalLimit", skip_serializing_if = "Option::is_none")]
    pub withdrawal_limit: Option<AccountUnits>,

    /// The Account's margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<AccountUnits>,

    /// The Account's margin call percentage. When this value is 1.0 or above
    /// the Account is in a margin call situation.
    #[serde(rename = "marginCallPercent", skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<DecimalNumber>,

    /// The ID of the last Transaction created for the Account.
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<TransactionId>,

    /// The details of the Trades currently open in the Account.
    #[serde(rename = "trades", default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<TradeSummary>,

    /// The details all Account Positions.
    #[serde(rename = "positions", default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<Position>,

    /// The details of the Orders currently pending in the Account.
    #[serde(rename = "orders", default, skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<Order>,

    /// The total amount of dividend adjustment paid over the lifetime of the
    /// Account in the Account's home currency.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The net asset value of the Account inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueNAV", skip_serializing_if = "Option::is_none")]
    pub true_nav: Option<DecimalNumber>,

    /// The total unrealized profit/loss inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,

    /// The date/time of the last dividend adjustment per instrument. This field
    /// is returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(
        rename = "lastDividendAdjustmentTimestamps",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub last_dividend_adjustment_timestamps: Vec<DividendAdjustmentTimestamp>,
}

/// An AccountChanges Object is used to represent the changes to an Account's
/// Orders, Trades and Positions since a specified Account TransactionID in the
/// past.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountChanges {
    /// The Orders created. These Orders may have been filled, cancelled or
    /// triggered in the same period.
    #[serde(
        rename = "ordersCreated",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orders_created: Vec<Order>,

    /// The Orders cancelled.
    #[serde(
        rename = "ordersCancelled",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orders_cancelled: Vec<Order>,

    /// The Orders filled.
    #[serde(
        rename = "ordersFilled",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orders_filled: Vec<Order>,

    /// The Orders triggered.
    #[serde(
        rename = "ordersTriggered",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orders_triggered: Vec<Order>,

    /// The Trades opened.
    #[serde(
        rename = "tradesOpened",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trades_opened: Vec<TradeSummary>,

    /// The Trades reduced.
    #[serde(
        rename = "tradesReduced",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trades_reduced: Vec<TradeSummary>,

    /// The Trades closed.
    #[serde(
        rename = "tradesClosed",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trades_closed: Vec<TradeSummary>,

    /// The Positions changed.
    #[serde(rename = "positions", default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<Position>,

    /// The Transactions that have been generated.
    #[serde(
        rename = "transactions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transactions: Vec<Transaction>,
}

/// An AccountState Object is used to represent an Account's current price-
/// dependent state. Price-dependent Account state is dependent on OANDA's
/// current Prices, and includes things like unrealized PL, NAV and Trailing
/// Stop Loss Order state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccountChangesState {
    /// The total unrealized profit/loss for all Trades currently open in the
    /// Account.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// The net asset value of the Account. Equal to Account balance +
    /// unrealizedPL.
    #[serde(rename = "NAV", skip_serializing_if = "Option::is_none")]
    pub nav: Option<AccountUnits>,

    /// Margin currently used for the Account.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// Margin available for Account currency.
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<AccountUnits>,

    /// The value of the Account's open positions represented in the Account's
    /// home currency.
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none")]
    pub position_value: Option<AccountUnits>,

    /// The Account's margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<AccountUnits>,

    /// The Account's margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV", skip_serializing_if = "Option::is_none")]
    pub margin_closeout_nav: Option<AccountUnits>,

    /// The Account's margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<AccountUnits>,

    /// The Account's margin closeout percentage. When this value is 1.0 or
    /// above the Account is in a margin closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<DecimalNumber>,

    /// The value of the Account's open positions as used for margin closeout
    /// calculations represented in the Account's home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<DecimalNumber>,

    /// The current WithdrawalLimit for the account which will be zero or a
    /// positive value indicating how much can be withdrawn from the account.
    #[serde(rename = "withdrawalLimit", skip_serializing_if = "Option::is_none")]
    pub withdrawal_limit: Option<AccountUnits>,

    /// The Account's margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<AccountUnits>,

    /// The Account's margin call percentage. When this value is 1.0 or above
    /// the Account is in a margin call situation.
    #[serde(rename = "marginCallPercent", skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<DecimalNumber>,

    /// The price-dependent state of each pending Order in the Account.
    #[serde(rename = "orders", default, skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<DynamicOrderState>,

    /// The price-dependent state for each open Trade in the Account.
    #[serde(rename = "trades", default, skip_serializing_if = "Vec::is_empty")]
    pub trades: Vec<CalculatedTradeState>,

    /// The price-dependent state for each open Position in the Account.
    #[serde(rename = "positions", default, skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<CalculatedPositionState>,

    /// The net asset value of the Account inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueNAV", skip_serializing_if = "Option::is_none")]
    pub true_nav: Option<DecimalNumber>,

    /// The total unrealized profit/loss inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,
}

/// The dynamically calculated state of a client's Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CalculatedAccountState {
    /// The total unrealized profit/loss for all Trades currently open in the
    /// Account.
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<AccountUnits>,

    /// The net asset value of the Account. Equal to Account balance +
    /// unrealizedPL.
    #[serde(rename = "NAV", skip_serializing_if = "Option::is_none")]
    pub nav: Option<AccountUnits>,

    /// Margin currently used for the Account.
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none")]
    pub margin_used: Option<AccountUnits>,

    /// Margin available for Account currency.
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none")]
    pub margin_available: Option<AccountUnits>,

    /// The value of the Account's open positions represented in the Account's
    /// home currency.
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none")]
    pub position_value: Option<AccountUnits>,

    /// The Account's margin closeout unrealized PL.
    #[serde(
        rename = "marginCloseoutUnrealizedPL",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_unrealized_pl: Option<AccountUnits>,

    /// The Account's margin closeout NAV.
    #[serde(rename = "marginCloseoutNAV", skip_serializing_if = "Option::is_none")]
    pub margin_closeout_nav: Option<AccountUnits>,

    /// The Account's margin closeout margin used.
    #[serde(
        rename = "marginCloseoutMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_margin_used: Option<AccountUnits>,

    /// The Account's margin closeout percentage. When this value is 1.0 or
    /// above the Account is in a margin closeout situation.
    #[serde(
        rename = "marginCloseoutPercent",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_percent: Option<DecimalNumber>,

    /// The value of the Account's open positions as used for margin closeout
    /// calculations represented in the Account's home currency.
    #[serde(
        rename = "marginCloseoutPositionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_closeout_position_value: Option<DecimalNumber>,

    /// The current WithdrawalLimit for the account which will be zero or a
    /// positive value indicating how much can be withdrawn from the account.
    #[serde(rename = "withdrawalLimit", skip_serializing_if = "Option::is_none")]
    pub withdrawal_limit: Option<AccountUnits>,

    /// The Account's margin call margin used.
    #[serde(
        rename = "marginCallMarginUsed",
        skip_serializing_if = "Option::is_none"
    )]
    pub margin_call_margin_used: Option<AccountUnits>,

    /// The Account's margin call percentage. When this value is 1.0 or above
    /// the Account is in a margin call situation.
    #[serde(rename = "marginCallPercent", skip_serializing_if = "Option::is_none")]
    pub margin_call_percent: Option<DecimalNumber>,

    /// The total amount of dividend adjustment paid over the lifetime of the
    /// Account in the Account's home currency.
    #[serde(rename = "dividendAdjustment", skip_serializing_if = "Option::is_none")]
    pub dividend_adjustment: Option<DecimalNumber>,

    /// The net asset value of the Account inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueNAV", skip_serializing_if = "Option::is_none")]
    pub true_nav: Option<DecimalNumber>,

    /// The total unrealized profit/loss inclusive of unsettled amounts. This
    /// field is returned by the live v20 API but is not present in OANDA's
    /// official documentation.
    #[serde(rename = "trueUnrealizedPL", skip_serializing_if = "Option::is_none")]
    pub true_unrealized_pl: Option<DecimalNumber>,

    /// The date/time of the last dividend adjustment per instrument. This field
    /// is returned by the live v20 API but is not present in OANDA's official
    /// documentation.
    #[serde(
        rename = "lastDividendAdjustmentTimestamps",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub last_dividend_adjustment_timestamps: Vec<DividendAdjustmentTimestamp>,
}
