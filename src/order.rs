use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};


pub enum Side {
    BUY,
    SELL,
}

pub enum Time {
    DAY,
    GTC,
    OPG,
    CLS,
    IOC,
    FOK,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Option<uuid::Uuid>, //Order id
    pub client_order_id: uuid::Uuid, //Client unique order id
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub filled_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub canceled_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub replaced_at: Option<DateTime<Utc>>,
    pub replaces: Option<uuid::Uuid>,// The order ID that this order replaces
    pub asset_id: uuid::Uuid, // Asset ID
    pub symbol: String, // Asset symbol
    pub asset_class: String, // Asset class
    pub qty: String, // Ordered quantity
    pub filled_qty: String, // Filled quantity
    pub side: String, //Valid values: buy, sell
    #[serde(rename = "type")]
    pub order_type: String,
    pub time_in_force: String, //https://alpaca.markets/docs/trading-on-alpaca/orders/#time-in-force
    pub limit_price: Option<Decimal>, //Limit price
    pub stop_price: Option<Decimal>, //Stop price
    pub filled_avg_price: Option<Decimal>, //Filled average price
    pub status: String, //
    pub extended_hours: bool, //If true, eligible for execution outside regular trading hours.
    pub legs: Option<String> //When querying non-simple order_class orders in a nested style,
    // an array of Order entities associated with this order. Otherwise, null.
}