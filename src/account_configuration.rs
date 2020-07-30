use serde::{Serialize, Deserialize};
use crate::client::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfiguration {
    pub dtbp_check: String,
    pub no_shorting: bool,
    pub suspend_trade: bool,
    pub trade_confirm_email: String,
}

impl AccountConfiguration {
    pub fn get_account_configurations(client: &Client) -> AccountConfiguration {

    }

    pub fn update_account_configurations(client: &Client, account: AccountConfiguration) -> AccountConfiguration {

    }
}