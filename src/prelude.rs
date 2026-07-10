//! Convenience re-exports of the types needed by almost every integration.
//!
//! ```no_run
//! use oanda_rs::prelude::*;
//!
//! let client = Client::new(Environment::Practice, "my-token");
//! ```

pub use crate::client::{Client, ClientBuilder, Environment};
pub use crate::error::{ApiErrorBody, Error};
pub use crate::models::transaction::{Transaction, TransactionStreamItem};
pub use crate::models::{
    AcceptDatetimeFormat, AccountId, AccountUnits, CandleSpecification, CandlestickGranularity,
    ClientExtensions, DateTime, DecimalNumber, InstrumentName, LimitOrderRequest,
    MarketIfTouchedOrderRequest, MarketOrderRequest, Order, OrderId, OrderRequest, OrderSpecifier,
    PriceStreamItem, PriceValue, PricingComponent, StopLossDetails, StopLossOrderRequest,
    StopOrderRequest, TakeProfitDetails, TakeProfitOrderRequest, TradeId, TradeSpecifier,
    TrailingStopLossDetails, TrailingStopLossOrderRequest, TransactionId,
};
pub use crate::streaming::{PricingStream, StreamStats, TransactionStream};
