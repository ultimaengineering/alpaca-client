
extern crate alpaca_client;

#[cfg(test)]
mod tests {
    use alpaca_client::client::Client;
    use alpaca_client::client::AccountType::PAPER;
    use alpaca_client::order::{Order};
    use uuid::Uuid;
    use std::env;
    use std::process::exit;
    use alpaca_client::bar::{BarRequest, TimeFrame, Bar};


    #[test]
    fn client_test() {
        get_client().get_account();
    }

    //While not containing an assert, still will fail due to runtime exception if miss-configured.
    #[test]
    fn get_orders_test() {
        let x = get_client().get_all_orders();
        println!("{:?}", &x);
    }

    #[test]
    fn get_order_test() {
        let client = get_client();
        let orders = &client.get_all_orders();
        let old_order = orders.get(0).unwrap();
        let _newly_retrieve_order = &client.get_order(old_order.id.unwrap());
    }

    #[test]
    fn place_order_test() {
        let new_order = Order {
            id: Some(Uuid::new_v4()),
            client_order_id: Uuid::new_v4(),
            created_at: None,
            updated_at:  None,
            submitted_at:  None,
            filled_at:  None,
            expired_at:  None,
            canceled_at:  None,
            failed_at: None,
            replaced_at: None,
            replaces: None,
            asset_id: Default::default(),
            symbol: "AMD".to_string(),
            asset_class: "".to_string(),
            qty: "1".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };

        let placed_order = get_client().place_order(new_order);
        assert_eq!(placed_order.status, "accepted");
    }

    #[test]
    fn cancel_order_test() {
        let new_order = Order {
            id: Some(Uuid::new_v4()),
            client_order_id: Uuid::new_v4(),
            created_at: None,
            updated_at:  None,
            submitted_at:  None,
            filled_at:  None,
            expired_at:  None,
            canceled_at:  None,
            failed_at: None,
            replaced_at: None,
            replaces: None,
            asset_id: Default::default(),
            symbol: "NVDA".to_string(),
            asset_class: "".to_string(),
            qty: "1".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        let _client = get_client();
        let placed_order = &_client.place_order(new_order);
        &_client.cancel_order(placed_order.id.unwrap());
    }

    #[test]
    fn patch_order_test() {
        let mut new_order = Order {
            id: Some(Uuid::new_v4()),
            client_order_id: Uuid::new_v4(),
            created_at: None,
            updated_at:  None,
            submitted_at:  None,
            filled_at:  None,
            expired_at:  None,
            canceled_at:  None,
            failed_at: None,
            replaced_at: None,
            replaces: None,
            asset_id: Default::default(),
            symbol: "AMD".to_string(),
            asset_class: "".to_string(),
            qty: "1".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        let placed_order = get_client().place_order(new_order);
        new_order = Order {
            id: placed_order.id,
            client_order_id: placed_order.client_order_id,
            created_at: None,
            updated_at:  None,
            submitted_at:  None,
            filled_at:  None,
            expired_at:  None,
            canceled_at:  None,
            failed_at: None,
            replaced_at: None,
            replaces: None,
            asset_id: Default::default(),
            symbol: "AMD".to_string(),
            asset_class: "".to_string(),
            qty: "2".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        assert_eq!(placed_order.id, new_order.id);
    }

    #[test]
    fn get_bar() {
        let request = BarRequest {
            time_frame: TimeFrame::OneMinute,
            symbols: "AMD".to_string(),
            limit: 0,
            start: None,
            end: None,
            after: None,
            until: None
        };
        let results = get_client().get_bar(request);
        let keys = results.keys();
        assert_eq!(keys.len(), 1);
        let bars: &Vec<Bar> = results.get("AMD").unwrap();
        let num_bars = bars.iter().len();
        assert_eq!(num_bars, 100);
    }

    #[test]
    fn get_last_trade() {
        let results = get_client().get_last_trade("AMD".to_owned());
        let status = results.status;
        assert_ne!(status, "");
    }

    #[test]
    fn get_last_quote() {
        let results = get_client().get_last_quote("AMD".to_owned());
        let status = results.status;
        assert_ne!(status, "");
    }


    fn get_access_key() -> String {
        return match env::var("alpaca_access_key") {
            Ok(key) => key,
            Err(_e) => exit(-1),
        };
    }

    fn get_secret_key() -> String {
        return match env::var("alpaca_secret_key") {
            Ok(key) => key,
            Err(_e) => exit(-1),
        };
    }

    fn get_client() -> Client {
        return Client::new(
            get_access_key(),
            get_secret_key(),
            PAPER
        );
    }
}