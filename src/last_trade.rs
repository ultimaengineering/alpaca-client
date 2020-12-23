use serde::{Deserialize};
use crate::client::Client;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Deserialize)]
pub struct LastStockTrade {
    pub status: String,
    pub symbol: String,
    pub last: Last
}

#[derive(Debug, Clone, Deserialize)]
pub struct Last {
    price: Decimal,
    size: i32,
    exchange: i32,
    cond1: i32,
    cond2: i32,
    cond3: i32,
    cond4: i32,
    timestamp: u128
}

impl Last {
    pub fn get(client: &Client, symbol: String) -> LastStockTrade {
        let _client = reqwest::blocking::Client::new();
        let mut url = "https://data.alpaca.markets/v1/last/stocks/".to_owned();
        url.push_str(&*symbol.to_uppercase());
        let result: LastStockTrade = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }
}