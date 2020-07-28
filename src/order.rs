use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use crate::client::Client;
use uuid::Uuid;


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

impl Order {
    pub fn get_all(client: &Client) -> Vec<Order> {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders");

        let result: Vec<Order> = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }

    pub fn place(client: &Client, _order: Order) -> Order {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders");
        let _result: Order = _client.post(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .json(&serde_json::json!(&_order))
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn get(client: &Client, id: Uuid) -> Order {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders/");
        url.push_str(id.to_string().as_ref());
        let _result: Order = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn replace(client: &Client, _order: Order) -> Order {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders");
        let _result: Order = _client.patch(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .json(&serde_json::json!(&_order))
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn cancel(client: &Client,  id: Uuid) {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders/");
        url.push_str(id.to_string().as_ref());
        _client.delete(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap();
    }

    pub fn cancel_all(client: &Client) {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("orders");
        _client.delete(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap();
    }

}