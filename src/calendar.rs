use serde::{Deserialize};
use crate::client::Client;

#[derive(Debug, Clone, Deserialize)]
pub struct Calendar {
    pub date: String, //Date string in “%Y-%m-%d” format
    pub open: String, //The time the market opens at on this date in “%H:%M” format
    pub close: String, //The time the market closes at on this date in “%H:%M” format
}

impl  Calendar {
    pub fn get(client: &Client) -> Vec<Calendar> {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("calendar");
        let result: Vec<Calendar> = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }
}