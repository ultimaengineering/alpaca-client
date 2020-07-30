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
    pub fn get_clock(client: &Client) -> Clock {

    }
}