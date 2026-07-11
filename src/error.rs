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

#[cfg(test)]
mod tests {
    use super::*;

    fn api_error(status: u16) -> Error {
        Error::Api {
            status: StatusCode::from_u16(status).unwrap(),
            request_id: Some("req-1".into()),
            body: ApiErrorBody {
                error_message: "boom".into(),
                error_code: Some("CODE".into()),
                reject_reason: None,
                extra: serde_json::Map::new(),
            },
        }
    }

    #[test]
    fn display_formats() {
        assert_eq!(
            api_error(400).to_string(),
            "OANDA API error (HTTP 400 Bad Request): boom"
        );
        let decode = Error::Decode {
            source: serde_json::from_str::<u8>("x").unwrap_err(),
            body: "x".into(),
        };
        assert!(
            decode
                .to_string()
                .starts_with("failed to decode response body")
        );
        assert_eq!(
            Error::Stream("gap".into()).to_string(),
            "stream protocol error: gap"
        );
        assert_eq!(
            Error::Config("bad".into()).to_string(),
            "invalid client configuration: bad"
        );
    }

    #[test]
    fn helpers() {
        assert_eq!(api_error(429).status().map(|s| s.as_u16()), Some(429));
        assert!(api_error(429).is_rate_limited());
        assert!(!api_error(400).is_rate_limited());
        assert_eq!(api_error(400).request_id(), Some("req-1"));
        assert_eq!(Error::Stream("x".into()).status(), None);
        assert_eq!(Error::Config("x".into()).request_id(), None);
    }

    #[test]
    fn details_decodes_extra_fields() {
        #[derive(serde::Deserialize)]
        struct View {
            #[serde(rename = "lastTransactionID")]
            last_transaction_id: String,
        }
        let mut extra = serde_json::Map::new();
        extra.insert("lastTransactionID".into(), serde_json::json!("42"));
        let body = ApiErrorBody {
            error_message: "rejected".into(),
            error_code: None,
            reject_reason: Some("REASON".into()),
            extra,
        };
        let view: View = body.details().unwrap();
        assert_eq!(view.last_transaction_id, "42");
        // A shape mismatch yields None instead of panicking.
        #[derive(serde::Deserialize)]
        struct Wrong {
            #[serde(rename = "lastTransactionID")]
            _n: u64,
        }
        assert!(body.details::<Wrong>().is_none());
    }
}
