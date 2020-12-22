use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::client::Client;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub start_time: i32,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
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
struct BarResponse {
    pub t: i32,
    pub o: Decimal,
    pub h: Decimal,
    pub l: Decimal,
    pub c: Decimal,
    pub v: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeFrame {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    Day
}

impl Bar {
    pub fn get_bar(client: &Client, bar_request: BarRequest) -> Bar {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
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

         let _result: BarResponse = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return Bar {
            start_time: _result.t,
            open_price: _result.o,
            high_price: _result.h,
            low_price: _result.l,
            close_price: _result.c,
            volume: _result.v
        };
    }
}