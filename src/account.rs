use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

// https://alpaca.markets/docs/api-documentation/api-v2/account/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_blocked: bool, //If true, the account activity by user is prohibited.
    pub account_number: String, //Account number.
    pub buying_power: Decimal, //Current available $ buying power; If multiplier = 4,
    // this is your daytrade buying power which is calculated as (last_equity - (last) maintenance_margin) * 4;
    // If multiplier = 2, buying_power = max(equity – initial_margin,0) * 2;
    // If multiplier = 1, buying_power = cash
    pub cash: Decimal, //Cash balance
    pub created_at: DateTime<Utc>, //Timestamp this account was created at
    pub currency: String, //“USD”
    pub daytrade_count: i32, //The current number of daytrades that have been made in the last 5 trading days (inclusive of today)
    pub daytrading_buying_power: Decimal,
    pub equity: Decimal, //Cash + long_market_value + short_market_value
    pub id: uuid::Uuid, //Account ID.
    pub initial_margin: Decimal,
    pub last_equity: Decimal, //Equity as of previous trading day at 16:00:00 ET
    pub last_maintenance_margin: Decimal,
    pub long_market_value: Decimal, //Real-time MtM value of all long positions held in the account
    pub maintenance_margin: Decimal,
    pub multiplier: String, //Buying power multiplier that represents account margin classification;
    // valid values 1 (standard limited margin account with 1x buying power),
    // 2 (reg T margin account with 2x intraday and overnight buying power;
    // this is the default for all non-PDT accounts with $2,000 or more equity),
    // 4 (PDT account with 4x intraday buying power and 2x reg T overnight buying power)
    pub pattern_day_trader: bool, //Whether or not the account has been flagged as a pattern day trader
    pub portfolio_value: Decimal, //Total value of cash + holding positions (This field is deprecated. It is equivalent to the equity field.)
    pub regt_buying_power: Decimal,
    pub short_market_value: Decimal, //Real-time MtM value of all short positions held in the account
    pub shorting_enabled: bool, //Flag to denote whether or not the account is permitted to short
    pub sma: String, //Value of special memorandum account (will be used at a later date to provide additional buying_power)
    pub status: String, //See Account Status
    pub trade_suspended_by_user: bool, //User setting. If true, the account is not allowed to place orders.
    pub trading_blocked: bool, //If true, the account is not allowed to place orders.
    pub transfers_blocked: bool //If true, the account is not allowed to request money transfers.
}