//! Account-lifecycle and funding transaction subtypes.

use serde::{Deserialize, Serialize};

use crate::models::macros::string_enum;
use crate::models::{
    AccountId, AccountUnits, Currency, DateTime, DecimalNumber, RequestId, TransactionId,
};

/// A ClientConfigureTransaction represents the configuration of an Account by a
/// client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClientConfigureTransaction {
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

    // type is pinned to "CLIENT_CONFIGURE" by the enum wrapper
    /// The client-provided alias for the Account.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// The margin rate override for the Account.
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    pub margin_rate: Option<DecimalNumber>,
}

/// A ClientConfigureRejectTransaction represents the reject of configuration of
/// an Account by a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClientConfigureRejectTransaction {
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

    // type is pinned to "CLIENT_CONFIGURE_REJECT" by the enum wrapper
    /// The client-provided alias for the Account.
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    /// The margin rate override for the Account.
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none")]
    pub margin_rate: Option<DecimalNumber>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

string_enum! {
    /// The reason that a Transaction was rejected.
    pub enum TransactionRejectReason {
        InternalServerError => "INTERNAL_SERVER_ERROR",
        InstrumentPriceUnknown => "INSTRUMENT_PRICE_UNKNOWN",
        AccountNotActive => "ACCOUNT_NOT_ACTIVE",
        AccountLocked => "ACCOUNT_LOCKED",
        AccountOrderCreationLocked => "ACCOUNT_ORDER_CREATION_LOCKED",
        AccountConfigurationLocked => "ACCOUNT_CONFIGURATION_LOCKED",
        AccountDepositLocked => "ACCOUNT_DEPOSIT_LOCKED",
        AccountWithdrawalLocked => "ACCOUNT_WITHDRAWAL_LOCKED",
        AccountOrderCancelLocked => "ACCOUNT_ORDER_CANCEL_LOCKED",
        InstrumentNotTradeable => "INSTRUMENT_NOT_TRADEABLE",
        PendingOrdersAllowedExceeded => "PENDING_ORDERS_ALLOWED_EXCEEDED",
        OrderIdUnspecified => "ORDER_ID_UNSPECIFIED",
        OrderDoesntExist => "ORDER_DOESNT_EXIST",
        OrderIdentifierInconsistency => "ORDER_IDENTIFIER_INCONSISTENCY",
        TradeIdUnspecified => "TRADE_ID_UNSPECIFIED",
        TradeDoesntExist => "TRADE_DOESNT_EXIST",
        TradeIdentifierInconsistency => "TRADE_IDENTIFIER_INCONSISTENCY",
        InsufficientMargin => "INSUFFICIENT_MARGIN",
        InstrumentMissing => "INSTRUMENT_MISSING",
        InstrumentUnknown => "INSTRUMENT_UNKNOWN",
        UnitsMissing => "UNITS_MISSING",
        UnitsInvalid => "UNITS_INVALID",
        UnitsPrecisionExceeded => "UNITS_PRECISION_EXCEEDED",
        UnitsLimitExceeded => "UNITS_LIMIT_EXCEEDED",
        UnitsMimimumNotMet => "UNITS_MIMIMUM_NOT_MET",
        PriceMissing => "PRICE_MISSING",
        PriceInvalid => "PRICE_INVALID",
        PricePrecisionExceeded => "PRICE_PRECISION_EXCEEDED",
        PriceDistanceMissing => "PRICE_DISTANCE_MISSING",
        PriceDistanceInvalid => "PRICE_DISTANCE_INVALID",
        PriceDistancePrecisionExceeded => "PRICE_DISTANCE_PRECISION_EXCEEDED",
        PriceDistanceMaximumExceeded => "PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        PriceDistanceMinimumNotMet => "PRICE_DISTANCE_MINIMUM_NOT_MET",
        TimeInForceMissing => "TIME_IN_FORCE_MISSING",
        TimeInForceInvalid => "TIME_IN_FORCE_INVALID",
        TimeInForceGtdTimestampMissing => "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING",
        TimeInForceGtdTimestampInPast => "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST",
        PriceBoundInvalid => "PRICE_BOUND_INVALID",
        PriceBoundPrecisionExceeded => "PRICE_BOUND_PRECISION_EXCEEDED",
        OrdersOnFillDuplicateClientOrderIds => "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS",
        TradeOnFillClientExtensionsNotSupported => "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED",
        ClientOrderIdInvalid => "CLIENT_ORDER_ID_INVALID",
        ClientOrderIdAlreadyExists => "CLIENT_ORDER_ID_ALREADY_EXISTS",
        ClientOrderTagInvalid => "CLIENT_ORDER_TAG_INVALID",
        ClientOrderCommentInvalid => "CLIENT_ORDER_COMMENT_INVALID",
        ClientTradeIdInvalid => "CLIENT_TRADE_ID_INVALID",
        ClientTradeIdAlreadyExists => "CLIENT_TRADE_ID_ALREADY_EXISTS",
        ClientTradeTagInvalid => "CLIENT_TRADE_TAG_INVALID",
        ClientTradeCommentInvalid => "CLIENT_TRADE_COMMENT_INVALID",
        OrderFillPositionActionMissing => "ORDER_FILL_POSITION_ACTION_MISSING",
        OrderFillPositionActionInvalid => "ORDER_FILL_POSITION_ACTION_INVALID",
        TriggerConditionMissing => "TRIGGER_CONDITION_MISSING",
        TriggerConditionInvalid => "TRIGGER_CONDITION_INVALID",
        OrderPartialFillOptionMissing => "ORDER_PARTIAL_FILL_OPTION_MISSING",
        OrderPartialFillOptionInvalid => "ORDER_PARTIAL_FILL_OPTION_INVALID",
        InvalidReissueImmediatePartialFill => "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL",
        TakeProfitOrderAlreadyExists => "TAKE_PROFIT_ORDER_ALREADY_EXISTS",
        TakeProfitOnFillPriceMissing => "TAKE_PROFIT_ON_FILL_PRICE_MISSING",
        TakeProfitOnFillPriceInvalid => "TAKE_PROFIT_ON_FILL_PRICE_INVALID",
        TakeProfitOnFillPricePrecisionExceeded => "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED",
        TakeProfitOnFillTimeInForceMissing => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING",
        TakeProfitOnFillTimeInForceInvalid => "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID",
        TakeProfitOnFillGtdTimestampMissing => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING",
        TakeProfitOnFillGtdTimestampInPast => "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TakeProfitOnFillClientOrderIdInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID",
        TakeProfitOnFillClientOrderTagInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        TakeProfitOnFillClientOrderCommentInvalid => "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        TakeProfitOnFillTriggerConditionMissing => "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING",
        TakeProfitOnFillTriggerConditionInvalid => "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID",
        StopLossOrderAlreadyExists => "STOP_LOSS_ORDER_ALREADY_EXISTS",
        StopLossOrderGuaranteedRequired => "STOP_LOSS_ORDER_GUARANTEED_REQUIRED",
        StopLossOrderGuaranteedPriceWithinSpread => "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD",
        StopLossOrderGuaranteedNotAllowed => "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED",
        StopLossOrderGuaranteedHaltedCreateViolation => "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION",
        StopLossOrderGuaranteedHaltedTightenViolation => "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION",
        StopLossOrderGuaranteedHedgingNotAllowed => "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED",
        StopLossOrderGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOrderNotCancelable => "STOP_LOSS_ORDER_NOT_CANCELABLE",
        StopLossOrderNotReplaceable => "STOP_LOSS_ORDER_NOT_REPLACEABLE",
        StopLossOrderGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOrderPriceAndDistanceBothSpecified => "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        StopLossOrderPriceAndDistanceBothMissing => "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING",
        StopLossOnFillRequiredForPendingOrder => "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER",
        StopLossOnFillGuaranteedNotAllowed => "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED",
        StopLossOnFillGuaranteedRequired => "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED",
        StopLossOnFillPriceMissing => "STOP_LOSS_ON_FILL_PRICE_MISSING",
        StopLossOnFillPriceInvalid => "STOP_LOSS_ON_FILL_PRICE_INVALID",
        StopLossOnFillPricePrecisionExceeded => "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED",
        StopLossOnFillGuaranteedMinimumDistanceNotMet => "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET",
        StopLossOnFillGuaranteedLevelRestrictionExceeded => "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED",
        StopLossOnFillDistanceInvalid => "STOP_LOSS_ON_FILL_DISTANCE_INVALID",
        StopLossOnFillPriceDistanceMaximumExceeded => "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        StopLossOnFillDistancePrecisionExceeded => "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED",
        StopLossOnFillPriceAndDistanceBothSpecified => "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED",
        StopLossOnFillPriceAndDistanceBothMissing => "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING",
        StopLossOnFillTimeInForceMissing => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        StopLossOnFillTimeInForceInvalid => "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        StopLossOnFillGtdTimestampMissing => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        StopLossOnFillGtdTimestampInPast => "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        StopLossOnFillClientOrderIdInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        StopLossOnFillClientOrderTagInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        StopLossOnFillClientOrderCommentInvalid => "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        StopLossOnFillTriggerConditionMissing => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        StopLossOnFillTriggerConditionInvalid => "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        TrailingStopLossOrderAlreadyExists => "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS",
        TrailingStopLossOnFillPriceDistanceMissing => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING",
        TrailingStopLossOnFillPriceDistanceInvalid => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID",
        TrailingStopLossOnFillPriceDistancePrecisionExceeded => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED",
        TrailingStopLossOnFillPriceDistanceMaximumExceeded => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED",
        TrailingStopLossOnFillPriceDistanceMinimumNotMet => "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET",
        TrailingStopLossOnFillTimeInForceMissing => "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING",
        TrailingStopLossOnFillTimeInForceInvalid => "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID",
        TrailingStopLossOnFillGtdTimestampMissing => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING",
        TrailingStopLossOnFillGtdTimestampInPast => "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST",
        TrailingStopLossOnFillClientOrderIdInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID",
        TrailingStopLossOnFillClientOrderTagInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID",
        TrailingStopLossOnFillClientOrderCommentInvalid => "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID",
        TrailingStopLossOrdersNotSupported => "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED",
        TrailingStopLossOnFillTriggerConditionMissing => "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING",
        TrailingStopLossOnFillTriggerConditionInvalid => "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID",
        CloseTradeTypeMissing => "CLOSE_TRADE_TYPE_MISSING",
        CloseTradePartialUnitsMissing => "CLOSE_TRADE_PARTIAL_UNITS_MISSING",
        CloseTradeUnitsExceedTradeSize => "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE",
        CloseoutPositionDoesntExist => "CLOSEOUT_POSITION_DOESNT_EXIST",
        CloseoutPositionIncompleteSpecification => "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION",
        CloseoutPositionUnitsExceedPositionSize => "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE",
        CloseoutPositionReject => "CLOSEOUT_POSITION_REJECT",
        CloseoutPositionPartialUnitsMissing => "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING",
        MarkupGroupIdInvalid => "MARKUP_GROUP_ID_INVALID",
        PositionAggregationModeInvalid => "POSITION_AGGREGATION_MODE_INVALID",
        AdminConfigureDataMissing => "ADMIN_CONFIGURE_DATA_MISSING",
        MarginRateInvalid => "MARGIN_RATE_INVALID",
        MarginRateWouldTriggerCloseout => "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT",
        AliasInvalid => "ALIAS_INVALID",
        ClientConfigureDataMissing => "CLIENT_CONFIGURE_DATA_MISSING",
        MarginRateWouldTriggerMarginCall => "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL",
        AmountInvalid => "AMOUNT_INVALID",
        InsufficientFunds => "INSUFFICIENT_FUNDS",
        AmountMissing => "AMOUNT_MISSING",
        FundingReasonMissing => "FUNDING_REASON_MISSING",
        ClientExtensionsDataMissing => "CLIENT_EXTENSIONS_DATA_MISSING",
        ReplacingOrderInvalid => "REPLACING_ORDER_INVALID",
        ReplacingTradeIdInvalid => "REPLACING_TRADE_ID_INVALID",
    }
}

/// A CreateTransaction represents the creation of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateTransaction {
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

    // type is pinned to "CREATE" by the enum wrapper
    /// The ID of the Division that the Account is in
    #[serde(rename = "divisionID", skip_serializing_if = "Option::is_none")]
    pub division_id: Option<i64>,

    /// The ID of the Site that the Account was created at
    #[serde(rename = "siteID", skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i64>,

    /// The ID of the user that the Account was created for
    #[serde(rename = "accountUserID", skip_serializing_if = "Option::is_none")]
    pub account_user_id: Option<i64>,

    /// The number of the Account within the site/division/user
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<i64>,

    /// The home currency of the Account
    #[serde(rename = "homeCurrency", skip_serializing_if = "Option::is_none")]
    pub home_currency: Option<Currency>,
}

/// A CloseTransaction represents the closing of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CloseTransaction {
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
    // type is pinned to "CLOSE" by the enum wrapper
}

/// A ReopenTransaction represents the re-opening of a closed Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReopenTransaction {
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
    // type is pinned to "REOPEN" by the enum wrapper
}

/// A TransferFundsTransaction represents the transfer of funds in/out of an
/// Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransferFundsTransaction {
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

    // type is pinned to "TRANSFER_FUNDS" by the enum wrapper
    /// The amount to deposit/withdraw from the Account in the Account's home
    /// currency. A positive value indicates a deposit, a negative value
    /// indicates a withdrawal.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<AccountUnits>,

    /// The `fundingReason` field.
    #[serde(rename = "fundingReason", skip_serializing_if = "Option::is_none")]
    pub funding_reason: Option<FundingReason>,

    /// An optional comment that may be attached to a fund transfer for audit
    /// purposes
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// The Account's balance after funds are transferred.
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<AccountUnits>,
}

/// A TransferFundsRejectTransaction represents the rejection of the transfer of
/// funds in/out of an Account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TransferFundsRejectTransaction {
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

    // type is pinned to "TRANSFER_FUNDS_REJECT" by the enum wrapper
    /// The amount to deposit/withdraw from the Account in the Account's home
    /// currency. A positive value indicates a deposit, a negative value
    /// indicates a withdrawal.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<AccountUnits>,

    /// The `fundingReason` field.
    #[serde(rename = "fundingReason", skip_serializing_if = "Option::is_none")]
    pub funding_reason: Option<FundingReason>,

    /// An optional comment that may be attached to a fund transfer for audit
    /// purposes
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// The `rejectReason` field.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<TransactionRejectReason>,
}

string_enum! {
    /// The reason that an Account is being funded.
    pub enum FundingReason {
        ClientFunding => "CLIENT_FUNDING",
        AccountTransfer => "ACCOUNT_TRANSFER",
        DivisionMigration => "DIVISION_MIGRATION",
        SiteMigration => "SITE_MIGRATION",
        Adjustment => "ADJUSTMENT",
    }
}
