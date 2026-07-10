//! Internal request plumbing: URL building, auth headers, response decoding
//! and error mapping. All endpoint modules funnel through this file, which
//! is also where `tracing` instrumentation lives.

use reqwest::header::HeaderMap;
use reqwest::{Method, RequestBuilder, Response, Url};
use serde::de::DeserializeOwned;

use crate::client::Client;
use crate::error::{ApiErrorBody, Error};

/// The response header carrying OANDA's request identifier.
const REQUEST_ID_HEADER: &str = "RequestID";
const DATETIME_FORMAT_HEADER: &str = "Accept-Datetime-Format";

impl Client {
    /// Builds a REST URL from path segments (each segment is
    /// percent-encoded, so specifiers like `@client-id` are safe).
    pub(crate) fn rest_url(&self, segments: &[&str]) -> Url {
        join_segments(&self.inner.rest_base, segments)
    }

    /// Builds a streaming-host URL from path segments.
    pub(crate) fn stream_url(&self, segments: &[&str]) -> Url {
        join_segments(&self.inner.stream_base, segments)
    }

    /// Starts a request with authentication and datetime-format headers set.
    pub(crate) fn request(&self, method: Method, url: Url) -> RequestBuilder {
        self.inner
            .http
            .request(method, url)
            .bearer_auth(&self.inner.token)
            .header(
                DATETIME_FORMAT_HEADER,
                self.inner.datetime_format.as_header_value(),
            )
    }

    /// Convenience for `request(Method::GET, self.rest_url(segments))`.
    pub(crate) fn get(&self, segments: &[&str]) -> RequestBuilder {
        self.request(Method::GET, self.rest_url(segments))
    }

    /// Convenience for a REST PUT request.
    pub(crate) fn put(&self, segments: &[&str]) -> RequestBuilder {
        self.request(Method::PUT, self.rest_url(segments))
    }

    /// Convenience for a REST POST request.
    pub(crate) fn post(&self, segments: &[&str]) -> RequestBuilder {
        self.request(Method::POST, self.rest_url(segments))
    }

    /// Convenience for a REST PATCH request.
    pub(crate) fn patch(&self, segments: &[&str]) -> RequestBuilder {
        self.request(Method::PATCH, self.rest_url(segments))
    }

    /// Waits for a stream-connection slot (2/s by default).
    pub(crate) async fn acquire_connection_slot(&self) {
        if let Some(limiter) = &self.inner.conn_limiter {
            limiter.acquire().await;
        }
    }

    /// Executes a REST request and decodes the JSON response body.
    pub(crate) async fn execute<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, Error> {
        let (value, _headers) = self.execute_with_headers(request).await?;
        Ok(value)
    }

    /// Executes a REST request, returning the decoded body together with the
    /// response headers (for endpoints that surface `Location`/`Link`).
    pub(crate) async fn execute_with_headers<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<(T, HeaderMap), Error> {
        if let Some(limiter) = &self.inner.rest_limiter {
            limiter.acquire().await;
        }
        let response = request.send().await?;

        #[cfg(feature = "tracing")]
        tracing::debug!(
            status = %response.status(),
            url = %response.url(),
            request_id = header_str(response.headers(), REQUEST_ID_HEADER),
            "oanda response"
        );

        if !response.status().is_success() {
            return Err(error_from_response(response).await);
        }
        let headers = response.headers().clone();
        let body = response.text().await?;
        match serde_json::from_str(&body) {
            Ok(value) => Ok((value, headers)),
            Err(source) => Err(Error::Decode { source, body }),
        }
    }
}

/// Converts a non-success response into [`Error::Api`], keeping the raw body
/// when it is not valid JSON.
pub(crate) async fn error_from_response(response: Response) -> Error {
    let status = response.status();
    let request_id = header_str(response.headers(), REQUEST_ID_HEADER).map(str::to_owned);
    let text = match response.text().await {
        Ok(text) => text,
        Err(e) => return Error::Transport(e),
    };
    let body = serde_json::from_str::<ApiErrorBody>(&text)
        .unwrap_or_else(|_| ApiErrorBody::from_text(text));
    Error::Api {
        status,
        request_id,
        body,
    }
}

pub(crate) fn header_str<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name).and_then(|v| v.to_str().ok())
}

fn join_segments(base: &Url, segments: &[&str]) -> Url {
    let mut url = base.clone();
    {
        let mut path = url
            .path_segments_mut()
            .expect("base URL cannot be a base? validated at construction");
        path.pop_if_empty();
        for segment in segments {
            path.push(segment);
        }
    }
    url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_and_percent_encodes_segments() {
        let base = Url::parse("https://api-fxpractice.oanda.com/v3").unwrap();
        let url = join_segments(&base, &["accounts", "101-004-1-001", "orders", "@my/id"]);
        assert_eq!(
            url.as_str(),
            "https://api-fxpractice.oanda.com/v3/accounts/101-004-1-001/orders/@my%2Fid"
        );
    }

    #[test]
    fn joins_on_base_with_trailing_slash() {
        let base = Url::parse("http://127.0.0.1:9999/").unwrap();
        let url = join_segments(&base, &["accounts"]);
        assert_eq!(url.as_str(), "http://127.0.0.1:9999/accounts");
    }
}
