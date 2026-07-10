# Endpoint coverage

All 35 operations of the OANDA v20 API (as described by the
[curated OpenAPI spec](../schema)) are implemented.

## Accounts

| Operation | Endpoint | SDK method |
|---|---|---|
| List accounts | `GET /v3/accounts` | `Client::list_accounts` |
| Account details | `GET /v3/accounts/{id}` | `Client::account` |
| Account summary | `GET /v3/accounts/{id}/summary` | `Client::account_summary` |
| Account instruments | `GET /v3/accounts/{id}/instruments` | `Client::account_instruments` |
| Configure account | `PATCH /v3/accounts/{id}/configuration` | `Client::configure_account` |
| Poll account changes | `GET /v3/accounts/{id}/changes` | `Client::account_changes` |

## Instruments

| Operation | Endpoint | SDK method |
|---|---|---|
| Candles | `GET /v3/instruments/{instrument}/candles` | `Client::candles` |
| Order book | `GET /v3/instruments/{instrument}/orderBook` | `Client::instrument_order_book` |
| Position book | `GET /v3/instruments/{instrument}/positionBook` | `Client::instrument_position_book` |
| Account-scoped candles | `GET /v3/accounts/{id}/instruments/{instrument}/candles` | `Client::account_candles` |
| Latest candles | `GET /v3/accounts/{id}/candles/latest` | `Client::latest_candles` |

## Orders

| Operation | Endpoint | SDK method |
|---|---|---|
| Create order | `POST /v3/accounts/{id}/orders` | `Client::create_order` |
| List orders | `GET /v3/accounts/{id}/orders` | `Client::list_orders` |
| Pending orders | `GET /v3/accounts/{id}/pendingOrders` | `Client::list_pending_orders` |
| Get order | `GET /v3/accounts/{id}/orders/{spec}` | `Client::order` |
| Replace order | `PUT /v3/accounts/{id}/orders/{spec}` | `Client::replace_order` |
| Cancel order | `PUT /v3/accounts/{id}/orders/{spec}/cancel` | `Client::cancel_order` |
| Set order client extensions | `PUT /v3/accounts/{id}/orders/{spec}/clientExtensions` | `Client::set_order_client_extensions` |

## Trades

| Operation | Endpoint | SDK method |
|---|---|---|
| List trades | `GET /v3/accounts/{id}/trades` | `Client::list_trades` |
| Open trades | `GET /v3/accounts/{id}/openTrades` | `Client::list_open_trades` |
| Get trade | `GET /v3/accounts/{id}/trades/{spec}` | `Client::trade` |
| Close trade | `PUT /v3/accounts/{id}/trades/{spec}/close` | `Client::close_trade` |
| Set trade client extensions | `PUT /v3/accounts/{id}/trades/{spec}/clientExtensions` | `Client::set_trade_client_extensions` |
| Set dependent orders | `PUT /v3/accounts/{id}/trades/{spec}/orders` | `Client::set_trade_dependent_orders` |

## Positions

| Operation | Endpoint | SDK method |
|---|---|---|
| List positions | `GET /v3/accounts/{id}/positions` | `Client::list_positions` |
| Open positions | `GET /v3/accounts/{id}/openPositions` | `Client::list_open_positions` |
| Get position | `GET /v3/accounts/{id}/positions/{instrument}` | `Client::position` |
| Close position | `PUT /v3/accounts/{id}/positions/{instrument}/close` | `Client::close_position` |

## Pricing

| Operation | Endpoint | SDK method |
|---|---|---|
| Current prices | `GET /v3/accounts/{id}/pricing` | `Client::prices` |
| **Pricing stream** | `GET /v3/accounts/{id}/pricing/stream` (stream host) | `Client::pricing_stream` |

## Transactions

| Operation | Endpoint | SDK method |
|---|---|---|
| List transaction pages | `GET /v3/accounts/{id}/transactions` | `Client::list_transactions` |
| Get transaction | `GET /v3/accounts/{id}/transactions/{txid}` | `Client::transaction` |
| ID range | `GET /v3/accounts/{id}/transactions/idrange` | `Client::transactions_id_range` |
| Since ID | `GET /v3/accounts/{id}/transactions/sinceid` | `Client::transactions_since_id` |
| **Transaction stream** | `GET /v3/accounts/{id}/transactions/stream` (stream host) | `Client::transaction_stream` |

## Models

- `Transaction`: all **36** subtypes as an internally tagged enum, plus an
  `Unknown(serde_json::Value)` fallback so future types never break deserialization.
- `Order`: all **8** subtypes plus `Unknown`.
- `OrderRequest`: typed request enum over the 7 creatable order types with builder
  constructors (the upstream spec leaves the request side untyped).
- Known gap (matching the spec): OANDA's newer `GUARANTEED_STOP_LOSS` order family is
  not modeled yet; such orders surface as `Order::Unknown` / `Transaction::Unknown`
  rather than failing.
