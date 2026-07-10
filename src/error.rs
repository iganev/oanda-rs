//! Error types returned by the SDK.

use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// The unified error type returned by every SDK operation.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The HTTP request could not be performed (connection, TLS, timeout, ...).
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// OANDA answered with a non-success HTTP status.
    ///
    /// The parsed error body is available in [`Error::Api::body`]; the raw
    /// payload of order-reject responses can be recovered with
    /// [`ApiErrorBody::details`].
    #[error("OANDA API error (HTTP {status}): {}", body.error_message)]
    Api {
        /// HTTP status code of the response.
        status: StatusCode,
        /// Value of the `RequestID` response header, when present.
        request_id: Option<String>,
        /// Parsed error body.
        body: ApiErrorBody,
    },

    /// A 2xx response body could not be deserialized into the expected type.
    #[error("failed to decode response body: {source}")]
    Decode {
        /// The underlying deserialization error.
        source: serde_json::Error,
        /// The raw response body, kept verbatim for debugging.
        body: String,
    },

    /// A streaming connection violated the expected protocol.
    #[error("stream protocol error: {0}")]
    Stream(String),

    /// The client was configured with invalid values.
    #[error("invalid client configuration: {0}")]
    Config(String),
}

impl Error {
    /// The HTTP status code associated with this error, if any.
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Error::Api { status, .. } => Some(*status),
            Error::Transport(e) => e.status(),
            _ => None,
        }
    }

    /// The `RequestID` header OANDA attached to the failing response, if any.
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Api { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }

    /// Whether this error was caused by OANDA's rate limiting (HTTP 429).
    ///
    /// OANDA allows 120 REST requests per second and 2 new connections per
    /// second per IP address. The client's built-in rate limiter (see
    /// [`ClientBuilder`](crate::ClientBuilder)) keeps you below those limits
    /// by default, so this should only occur when rate limiting was disabled
    /// or other processes share the same IP.
    pub fn is_rate_limited(&self) -> bool {
        self.status() == Some(StatusCode::TOO_MANY_REQUESTS)
    }
}

/// The JSON body OANDA returns for error responses.
///
/// All error responses carry `errorMessage`; some also carry `errorCode`
/// and/or `rejectReason`. Order-related rejections include additional
/// top-level fields (e.g. `orderRejectTransaction`, `relatedTransactionIDs`,
/// `lastTransactionID`), which are preserved in [`ApiErrorBody::extra`] and
/// can be decoded into a typed struct with [`ApiErrorBody::details`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApiErrorBody {
    /// Human-readable description of the error.
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    /// Machine-readable error code, when provided.
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// The reason the request was rejected, when provided.
    #[serde(rename = "rejectReason", skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    /// Any additional top-level fields of the error body.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ApiErrorBody {
    /// Constructs an error body from a plain-text (non-JSON) payload.
    pub(crate) fn from_text(text: String) -> Self {
        ApiErrorBody {
            error_message: text,
            error_code: None,
            reject_reason: None,
            extra: serde_json::Map::new(),
        }
    }

    /// Attempts to decode the full error body into a typed view.
    ///
    /// Useful for order endpoints, whose 400/404 responses carry reject
    /// transactions, e.g.
    /// [`CreateOrderRejectBody`](crate::endpoints::orders::CreateOrderRejectBody).
    pub fn details<T: DeserializeOwned>(&self) -> Option<T> {
        serde_json::to_value(self)
            .ok()
            .and_then(|v| serde_json::from_value(v).ok())
    }
}
