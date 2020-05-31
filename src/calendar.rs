use serde::{Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Calendar {
    pub date: String, //Date string in “%Y-%m-%d” format
    pub open: String, //The time the market opens at on this date in “%H:%M” format
    pub close: String, //The time the market closes at on this date in “%H:%M” format
}