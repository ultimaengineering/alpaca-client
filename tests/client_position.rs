extern crate alpaca;

#[cfg(test)]
mod tests {
    use std::env;
    use alpaca::client::Client;
    use alpaca::client::AccountType::PAPER;
    use std::process::exit;
    use std::borrow::Borrow;
    use alpaca::order::Order;
    use uuid::Uuid;
    use alpaca::position::Position;

    #[test]
    pub fn get_open_positions() {
        let client = get_client();
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
            qty: "100".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1000".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        &client.place_order(new_order.clone());
        let positions:Vec<Position> = client.get_open_positions();
        let result = positions.into_iter()
            .any(|x| x.symbol.eq(&new_order.borrow().symbol));
        assert!(result, true)
    }

    #[test]
    pub fn get_open_position() {
        let client = get_client();
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
            qty: "100".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1000".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        &client.place_order(new_order.clone());
        let mut positions = Vec::new();
        positions.push(client.get_open_position("AMD".to_string()));
        let result = positions.into_iter()
            .any(|x : Position | x.symbol.eq(&new_order.borrow().symbol));
        assert!(result, true)
    }

    #[test]
    pub fn close_all_positions() {
        let client = get_client();
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
            symbol: "INTC".to_string(),
            asset_class: "".to_string(),
            qty: "100".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1000".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        &client.place_order(new_order.clone());
        let positions:Vec<Position> = client.get_open_positions();
        assert!(positions.len() > 0);
        client.close_all_positions();
        let result = positions.into_iter()
            .any(|x| x.symbol.eq(&new_order.borrow().symbol));
        assert!(result, false);
        assert!(client.get_open_positions().len() == 0)
    }

    #[test]
    pub fn close_position() {
        let client = get_client();
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
            symbol: "INTC".to_string(),
            asset_class: "".to_string(),
            qty: "100".to_string(),
            filled_qty: "".to_string(),
            side: "buy".to_string(),
            order_type: "stop_limit".parse().unwrap(),
            time_in_force: "day".to_string(),
            limit_price: Some("1000".parse().unwrap()),
            stop_price: Some("1".parse().unwrap()),
            filled_avg_price: Default::default(),
            status: "".to_string(),
            extended_hours: false,
            legs: None
        };
        &client.place_order(new_order.clone());
        let positions:Vec<Position> = client.get_open_positions();
        assert!(positions.len() > 0);
        client.close_position("INTC".to_string());
        let result = positions.into_iter()
            .any(|x| x.symbol.eq(&new_order.borrow().symbol));
        assert!(result, false)
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