use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

// https://alpaca.markets/docs/api-documentation/api-v2/account/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_blocked: String,
    pub account_number: String,
    pub buying_power: Decimal,
    pub cash: Decimal,
    pub created_at: DateTime<Utc>,
    pub currency: String,
    pub daytrade_count: i32,
    pub daytrading_buying_power: Decimal,
    pub equity: Decimal,
    pub id: uuid::Uuid,
    pub initial_margin: Decimal,
    pub last_equity: Decimal,
    pub last_maintenance_margin: Decimal,
    pub long_market_value: Decimal,
    pub maintenance_margin: Decimal,
    pub multiplier: String,
    pub pattern_day_trader: String,
    pub portfolio_value: Decimal,
    pub regt_buying_power: Decimal,
    pub short_market_value: Decimal,
    pub shorting_enabled: String,
    pub sma: String,
    pub status: String,
    pub trade_suspended_by_user: String,
    pub trading_blocked: String,
    pub transfers_blocked: String
}