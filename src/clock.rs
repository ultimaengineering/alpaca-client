use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::client::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    pub timestamp: DateTime<Utc>,
    pub is_open: bool,
    pub next_open: DateTime<Utc>,
    pub next_close: DateTime<Utc>,
}

impl Clock {
    pub fn get(client: &Client) -> Clock {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("clock");
        let _result: Clock = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }
}