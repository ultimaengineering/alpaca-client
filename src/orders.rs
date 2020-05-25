use chrono::{DateTime, Utc};
use rust_decimal::Decimal;


struct Order {
    id: uuid::Uuid,
    client_order_id: uuid::Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    submitted_at: DateTime<Utc>,
    filled_at: DateTime<Utc>,
    expired_at: DateTime<Utc>,
    canceled_at: DateTime<Utc>,
    failed_at: DateTime<Utc>,
    replaced_at: DateTime<Utc>,
    replaces: uuid::Uuid,
    asset_id: uuid::Uuid,
    symbol: String,
    asset_class: String,
    qty: i32,
    filled_qty: i32,
    side: String,
    time_in_force: String,
    limit_price: Decimal,
    stop_price: Decimal,
    filled_avg_price: Decimal,
    status: String,
    extended_hours: bool,
    legs: String
}