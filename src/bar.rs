use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::client::Client;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    #[serde(alias = "t")]
    pub start_time: i32,
    #[serde(alias = "o")]
    pub open_price: Decimal,
    #[serde(alias = "h")]
    pub high_price: Decimal,
    #[serde(alias = "l")]
    pub low_price: Decimal,
    #[serde(alias = "c")]
    pub close_price: Decimal,
    #[serde(alias = "v")]
    pub volume: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarRequest {
    pub time_frame: TimeFrame,
    pub symbols: String,
    pub limit: i32,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
    pub after: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeFrame {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    Day
}

impl Bar {
    pub fn get_bar(client: &Client, bar_request: BarRequest) -> HashMap<String, Vec<Bar>> {
        let _client = reqwest::blocking::Client::new();
        let mut url = "https://data.alpaca.markets/v1/".to_owned();
        url.push_str("bars/");
        url.push_str(match bar_request.time_frame {
            TimeFrame::OneMinute => "1Min",
            TimeFrame::FiveMinute => "5Min",
            TimeFrame::FifteenMinute => "15Min",
            TimeFrame::Day => "1D"
        });
        url.push_str(&*("?symbols=".to_owned() + &*bar_request.symbols));
        match bar_request.start {
            Some(e) => url.push_str(&*("?".to_string() + &*e.to_string())),
            _ => {}
        }
        match bar_request.end {
            Some(e) => url.push_str(&*("?".to_string() + &*e.to_string())),
            _ => {}
        }
        match bar_request.after {
            Some(e) => url.push_str(&*("?".to_string() + &*e.to_string())),
            _ => {}
        }
        match bar_request.until {
            Some(e) => url.push_str(&*("?".to_string() + &*e.to_string())),
            _ => {}
        }
        let result: HashMap<String, Vec<Bar>> = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }
}