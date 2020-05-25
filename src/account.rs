use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// https://alpaca.markets/docs/api-documentation/api-v2/account/
pub struct Account {
    account_blocked: bool,
    account_number: String,
    buying_power: Decimal,
    cash: Decimal,
    created_at: DateTime<Utc>,
    currency: String,
    daytrade_count: i32,
    daytrading_buying_power: Decimal,
    equity: Decimal,
    id: uuid::Uuid,
    initial_margin: Decimal,
    last_equity: Decimal,
    last_maintenance_margin: Decimal,
    long_market_value: Decimal,
    maintenance_margin: Decimal,
    multiplier: i32,
    pattern_day_trader: bool,
    portfolio_value: Decimal,
    regt_buying_power: Decimal,
    short_market_value: Decimal,
    shorting_enabled: bool,
    sma: i32,
    status: String,
    trade_suspended_by_user: bool,
    trading_blocked: bool,
    transfers_blocked: bool
}