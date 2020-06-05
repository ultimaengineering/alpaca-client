use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfiguration {
    pub dtbp_check: String,
    pub no_shorting: bool,
    pub suspend_trade: bool,
    pub trade_confirm_email: String,
}