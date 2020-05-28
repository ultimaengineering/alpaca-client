use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: uuid::Uuid,
    pub client_order_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: DateTime<Utc>,
    pub filled_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub canceled_at: DateTime<Utc>,
    pub failed_at: DateTime<Utc>,
    pub replaced_at: DateTime<Utc>,
    pub replaces: Option<uuid::Uuid>,
    pub asset_id: uuid::Uuid,
    pub symbol: String,
    pub asset_class: String,
    pub qty: String,
    pub filled_qty: String,
    pub side: String,
    pub time_in_force: String,
    pub limit_price: Decimal,
    pub stop_price: Decimal,
    pub filled_avg_price: Decimal,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<String>
}