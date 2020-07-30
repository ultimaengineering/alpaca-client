use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use crate::client::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub asset_id: uuid::Uuid, //Asset ID
    pub symbol: String, //Symbol name of the asset
    pub exchange: String, //Exchange name of the asset
    pub asset_class: String, //Asset class name
    pub avg_entry_price: Decimal, //Average entry price of the position
    pub qty: String, //The number of shares
    pub side: String, //“long”
    pub market_value: Decimal, //Total dollar amount of the position
    pub cost_basis: Decimal, //Total cost basis in dollar
    pub unrealized_pl: Decimal, //Unrealized profit/loss in dollars
    pub unrealized_plpc: Decimal, //Unrealized profit/loss percent (by a factor of 1)
    pub unrealized_intraday_pl: Decimal, //Unrealized profit/loss in dollars for the day
    pub unrealized_intraday_plpc: Decimal, //Unrealized profit/loss percent (by a factor of 1)
    pub current_price: Decimal, //Current asset price per share
    pub lastday_price: Decimal, //Last day’s asset price per share based on the closing value of the last trading day
    pub change_today: Decimal //Percent change from last day price (by a factor of 1)
}

impl Position {

    pub fn get_all(client: &Client) -> Vec<Position> {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("positions");

        let result: Vec<Position> = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return result;
    }

    pub fn get(client: &Client, symbol: String) -> Position {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("positions/");
        url.push_str(symbol.to_string().as_ref());
        let _result: Position = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn close_all(client: &Client) {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("positions");
        _client.delete(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap();
    }

    pub fn close(client: &Client, symbol: String) {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("positions/");
        url.push_str(symbol.to_string().as_ref());
        _client.delete(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap();
    }
}