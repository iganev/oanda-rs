//! Shared helpers for endpoint integration tests.

use oanda_rs::{Client, Environment};
use wiremock::MockServer;

pub const TOKEN: &str = "test-token";
pub const ACCOUNT_ID: &str = "101-004-1234567-001";

/// Starts a wiremock server and a client pointed at it (both REST and
/// stream hosts).
pub async fn mock_client() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let url: reqwest::Url = server.uri().parse().unwrap();
    let client = Client::builder()
        .environment(Environment::Custom {
            rest: url.clone(),
            stream: url,
        })
        .token(TOKEN)
        .build()
        .unwrap();
    (server, client)
}

/// Matchers asserting the headers the SDK must send on every request.
pub fn standard_headers(mock: wiremock::MockBuilder) -> wiremock::MockBuilder {
    use wiremock::matchers::header;
    mock.and(header("Authorization", format!("Bearer {TOKEN}")))
        .and(header("Accept-Datetime-Format", "RFC3339"))
}
