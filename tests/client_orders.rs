
extern crate alpaca;
#[cfg(test)]
mod tests {
    use alpaca::account;
    use alpaca::order;
    use rust_decimal::prelude::*;
    use std::borrow::Borrow;
    use alpaca::client::Client;
    use alpaca::client::AccountType::PAPER;
    use alpaca::order::{Order};
    use chrono::{Utc, TimeZone};
    use uuid::Uuid;
    use reqwest::get;



    #[test]
    fn client_test() {
        get_client().get_account();
    }

    //While not containing an assert, still will fail due to runtime exception if miss-configured.
    #[test]
    fn client_get_orders_test() {
        let x = get_client().get_all_orders();
        println!("{:?}", &x);
    }

    #[test]
    fn client_get_order_test() {
        let client = get_client();
        let mut orders = &client.get_all_orders();
        let mut num_orders = &orders.iter().count();
        let old_order = orders.get(0).unwrap();
        let newly_retrieve_order = &client.get_order(old_order.id.unwrap());
    }

    #[test]
    fn client_place_order_test() {
        let new_order = Order {
            id: Some(Uuid::new_v4()),
            client_order_id: Uuid::new_v4(),
            created_at: None,
            updated_at:  None,
            submitted_at:  None,
            filled_at:  None,
            expired_at:  None,
            canceled_at:  None,
            failed_at: Some(Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829)),
            replaced_at: Some(Utc.ymd(2018, 1, 26).and_hms_micro(18, 30, 9, 453_829)),
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

    fn get_access_key() -> String {
        return "PK0B00349LFLYTD56116".parse().unwrap();
    }

    fn get_secret_key() -> String {
        return "dqKgT3Q4wMytUoyp5SvcdLk1jIm/Hb7tVikK4qzH".parse().unwrap();
    }

    fn get_client() -> Client {
        return Client::new(
            get_access_key(),
            get_secret_key(),
            PAPER
        );
    }
}