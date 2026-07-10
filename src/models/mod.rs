//! Data models mirroring the OANDA v20 component schemas.
//!
//! Every type derives `Debug`, `Clone`, `Serialize` and `Deserialize` and
//! (de)serializes exactly as the API encodes it on the wire. Response
//! structs and enums are `#[non_exhaustive]`, and string enums keep unknown
//! wire values in an `Other(String)` variant, so additions on OANDA's side
//! never become breaking changes.

pub(crate) mod macros;
pub(crate) mod serde_util;

mod account;
mod instrument;
mod instrument_name;
mod order;
mod position;
mod pricing;
mod primitives;
mod trade;
pub mod transaction;

pub use account::*;
pub use instrument::*;
pub use instrument_name::InstrumentName;
pub use order::*;
pub use position::*;
pub use pricing::*;
pub use primitives::{
    AcceptDatetimeFormat, AccountId, AccountUnits, ClientComment, ClientId, ClientTag, Currency,
    DateTime, DecimalNumber, OrderId, OrderSpecifier, PriceValue, RequestId, TradeId,
    TradeSpecifier, TransactionId,
};
pub use trade::*;
