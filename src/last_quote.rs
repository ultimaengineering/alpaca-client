use serde::{Deserialize};
use crate::client::Client;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Deserialize)]
pub struct LastQuote {
    pub status: String,
    pub symbol: String,
    pub last: Last
}

#[derive(Debug, Clone, Deserialize)]
pub struct Last {
    askprice: Decimal,
    asksize: i32,
    askexchange: i32,
    bidprice: Decimal,
    bidsize: i32,
    bidexchange: i32,
    timestamp: u128
}

impl LastQuote {
    pub fn get(client: &Client, symbol: String) -> LastQuote {
        let _client = reqwest::blocking::Client::new();
        let mut url = "https://data.alpaca.markets/v1/last_quote/stocks/".to_owned();
        url.push_str(&*symbol.to_uppercase());
        let result: LastQuote = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }
}