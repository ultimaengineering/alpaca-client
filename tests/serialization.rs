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
    fn accounts_serialization() {
        let _data = r#"{
          "account_blocked": false,
          "account_number": "010203ABCD",
          "buying_power": "262113.632",
          "cash": "-23140.2",
          "created_at": "2019-06-12T22:47:07.99658Z",
          "currency": "USD",
          "daytrade_count": 0,
          "daytrading_buying_power": "262113.632",
          "equity": "103820.56",
          "id": "e6fe16f3-64a4-4921-8928-cadf02f92f98",
          "initial_margin": "63480.38",
          "last_equity": "103529.24",
          "last_maintenance_margin": "38000.832",
          "long_market_value": "126960.76",
          "maintenance_margin": "38088.228",
          "multiplier": "4",
          "pattern_day_trader": true,
          "portfolio_value": "103820.56",
          "regt_buying_power": "80680.36",
          "short_market_value": "0",
          "shorting_enabled": true,
          "sma": "0",
          "status": "ACTIVE",
          "trade_suspended_by_user": false,
          "trading_blocked": false,
          "transfers_blocked": false
        }"#;
        let deserialized: account::Account = serde_json::from_str(&_data).unwrap();
        assert_eq!(&deserialized.id, uuid::Uuid::from_str("e6fe16f3-64a4-4921-8928-cadf02f92f98").unwrap().borrow());
        assert_eq!(&deserialized.account_number, "010203ABCD");
        assert_eq!(&deserialized.buying_power, Decimal::from_str("262113.632").unwrap().borrow());
        assert_eq!(&deserialized.account_blocked, &false);
        assert_eq!(&deserialized.cash, Decimal::from_str("-23140.2").unwrap().borrow());
        assert_eq!(&deserialized.currency, "USD");
        assert_eq!(&deserialized.daytrade_count, "0".parse::<i32>().unwrap().borrow());
        assert_eq!(&deserialized.daytrading_buying_power, Decimal::from_str("262113.632").unwrap().borrow());
        assert_eq!(&deserialized.equity, Decimal::from_str("103820.56").unwrap().borrow());
        assert_eq!(&deserialized.initial_margin, Decimal::from_str("63480.38").unwrap().borrow());
        assert_eq!(&deserialized.last_equity, Decimal::from_str("103529.24").unwrap().borrow());
        assert_eq!(&deserialized.last_maintenance_margin, Decimal::from_str("38000.832").unwrap().borrow());
        assert_eq!(&deserialized.long_market_value, Decimal::from_str("126960.76").unwrap().borrow());
        assert_eq!(&deserialized.maintenance_margin, Decimal::from_str("38088.228").unwrap().borrow());
        assert_eq!(&deserialized.multiplier, "4");
        assert_eq!(&deserialized.pattern_day_trader, &true);
        assert_eq!(&deserialized.portfolio_value, Decimal::from_str("103820.56").unwrap().borrow());
        assert_eq!(&deserialized.regt_buying_power, Decimal::from_str("80680.36").unwrap().borrow());
        assert_eq!(&deserialized.short_market_value, Decimal::from_str("0").unwrap().borrow());
        assert_eq!(&deserialized.shorting_enabled, &true);
        assert_eq!(&deserialized.sma, "0");
        assert_eq!(&deserialized.status, "ACTIVE");
        assert_eq!(&deserialized.trade_suspended_by_user, &false);
        assert_eq!(&deserialized.trading_blocked, &false);
        assert_eq!(&deserialized.transfers_blocked, &false);
        print!("{:?}", deserialized);
    }

    #[test]
    fn order_serialization() {
        let _data = r#" {
          "id": "904837e3-3b76-47ec-b432-046db621571b",
          "client_order_id": "904837e3-3b76-47ec-b432-046db621571b",
          "created_at": "2018-10-05T05:48:59Z",
          "updated_at": "2018-10-05T05:48:59Z",
          "submitted_at": "2018-10-05T05:48:59Z",
          "filled_at": "2018-10-05T05:48:59Z",
          "expired_at": "2018-10-05T05:48:59Z",
          "canceled_at": "2018-10-05T05:48:59Z",
          "failed_at": "2018-10-05T05:48:59Z",
          "replaced_at": "2018-10-05T05:48:59Z",
          "replaced_by": "904837e3-3b76-47ec-b432-046db621571b",
          "replaces": null,
          "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
          "symbol": "AAPL",
          "asset_class": "us_equity",
          "qty": "15",
          "filled_qty": "0",
          "type": "market",
          "side": "buy",
          "time_in_force": "day",
          "limit_price": "107.00",
          "stop_price": "106.00",
          "filled_avg_price": "106.00",
          "status": "accepted",
          "extended_hours": false,
          "legs": null
        }
        "#;
        let deserialized: order::Order = serde_json::from_str(&_data).unwrap();
        let _optional_uuid = std::option::Option::Some(uuid::Uuid::from_str("904837e3-3b76-47ec-b432-046db621571b").unwrap().borrow());
        let uuid = uuid::Uuid::from_str("904837e3-3b76-47ec-b432-046db621571b").unwrap();
        assert_eq!(&deserialized.id, &Some(uuid));
        assert_eq!(&deserialized.client_order_id, uuid.borrow());
        assert_eq!(&deserialized.asset_id, uuid.borrow());
        assert_eq!(&deserialized.asset_class, "us_equity");
        assert_eq!(&deserialized.symbol, "AAPL");
        assert_eq!(&deserialized.limit_price, &Some(Decimal::from_str("107").unwrap()));
        assert_eq!(&deserialized.qty, "15");
        assert_eq!(&deserialized.filled_qty, "0");
        assert_eq!(&deserialized.side, "buy");
        assert_eq!(&deserialized.time_in_force, "day");
        assert_eq!(&deserialized.stop_price, &Some(Decimal::from_str("106").unwrap()));
        assert_eq!(&deserialized.filled_avg_price, &Some(Decimal::from_str("106").unwrap()));
        assert_eq!(&deserialized.status, "accepted");
        assert_eq!(&deserialized.extended_hours, &false);
        print!("{:?}", deserialized);
    }

    #[test]
    fn position_serialization() {
        let _data = r#" {
          "asset_id": "904837e3-3b76-47ec-b432-046db621571b",
          "symbol": "AAPL",
          "exchange": "NASDAQ",
          "asset_class": "us_equity",
          "avg_entry_price": "100.0",
          "qty": "5",
          "side": "long",
          "market_value": "600.0",
          "cost_basis": "500.0",
          "unrealized_pl": "100.0",
          "unrealized_plpc": "0.20",
          "unrealized_intraday_pl": "10.0",
          "unrealized_intraday_plpc": "0.0084",
          "current_price": "120.0",
          "lastday_price": "119.0",
          "change_today": "0.0084"
        }"#;
    }

    #[test]
    fn asset_serialization() {
        let _data = r#" {
          "id": "904837e3-3b76-47ec-b432-046db621571b",
          "class": "us_equity",
          "exchange": "NASDAQ",
          "symbol": "AAPL",
          "status": "active",
          "tradable": true,
          "marginable": true,
          "shortable": true,
          "easy_to_borrow": true
        }"#;
    }

    #[test]
    fn calendar_serilaization() {
        let _data = r#"
        {
            "date": "2018-01-03",
            "open": "09:30",
            "close": "16:00"
        }"#;

    }

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
        let mut orders = get_client().get_all_orders();
        let mut num_orders = &orders.iter().count();
        if num_orders.clone() == 0 {
            client_place_order_test();
            orders = get_client().get_all_orders();
            num_orders = &orders.iter().count();
        }
        let old_order = orders.get(0).unwrap();
        let newly_retrieve_order = get_client().get_order(old_order.id.unwrap());
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