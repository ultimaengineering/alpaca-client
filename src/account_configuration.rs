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
    pub fn get(client: &Client) -> AccountConfiguration {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("account/configurations");
        let _result: AccountConfiguration = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn update(client: &Client, _account: AccountConfiguration) -> AccountConfiguration {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("account/configurations ");
        let _result: AccountConfiguration = _client.patch(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .json(&serde_json::json!(&_account))
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }
}