use serde::{Serialize, Deserialize};
use crate::client::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: uuid::Uuid, //Asset ID.
    pub class: String, //“us_equity”
    pub exchange: String, //AMEX, ARCA, BATS, NYSE, NASDAQ or NYSEARCA
    pub symbol: String, //
    pub status: String, //active or inactive
    pub tradable: bool, //Asset is tradable on Alpaca or not.
    pub marginable: bool, //Asset is marginable or not.
    pub shortable: bool, //Asset is shortable or not.
    pub easy_to_borrow: bool //Asset is easy-to-borrow or not (filtering for easy_to_borrow = True
    // is the best way to check whether the name is currently available to short at Alpaca).
}

impl Asset {

    pub fn get_all(client: &Client) -> Vec<Asset> {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("assets");
        let _result: Vec<Asset> = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }

    pub fn get(client: &Client, symbol: String) -> Asset {
        let _client = reqwest::blocking::Client::new();
        let mut url = client.get_url();
        url.push_str("assets/");
        url.push_str(symbol.to_string().as_ref());
        let _result: Asset = _client.get(&url)
            .header("APCA-API-KEY-ID", &client.auth.access_key)
            .header("APCA-API-SECRET-KEY", &client.auth.secret_key)
            .send()
            .unwrap()
            .json()
            .unwrap();
        return _result;
    }
}