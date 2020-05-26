extern crate alpaca_client;
#[cfg(test)]
mod tests {
    use alpaca_client::account;
    use rust_decimal::prelude::*;
    use std::borrow::Borrow;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_accounts_serialization() {
        let data = r#"{
              "account_blocked": "False",
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
              "pattern_day_trader": "False",
              "portfolio_value": "103820.56",
              "regt_buying_power": "80680.36",
              "short_market_value": "0",
              "shorting_enabled": "True",
              "sma": "0",
              "status": "ACTIVE",
              "trade_suspended_by_user": "False",
              "trading_blocked": "False",
              "transfers_blocked": "False"
        }"#;
        let deserialized: account::Account = serde_json::from_str(&data).unwrap();
        assert_eq!(&deserialized.id, uuid::Uuid::from_str("e6fe16f3-64a4-4921-8928-cadf02f92f98").unwrap().borrow());
        assert_eq!(&deserialized.account_number, "010203ABCD");
        assert_eq!(&deserialized.buying_power, Decimal::from_str("262113.632").unwrap().borrow());
        assert_eq!(&deserialized.account_blocked, "False");
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
        assert_eq!(&deserialized.pattern_day_trader, "False");
        assert_eq!(&deserialized.portfolio_value, Decimal::from_str("103820.56").unwrap().borrow());
        assert_eq!(&deserialized.regt_buying_power, Decimal::from_str("80680.36").unwrap().borrow());
        assert_eq!(&deserialized.short_market_value, Decimal::from_str("0").unwrap().borrow());
        assert_eq!(&deserialized.shorting_enabled, "True");
        assert_eq!(&deserialized.sma, "0");
        assert_eq!(&deserialized.status, "ACTIVE");
        assert_eq!(&deserialized.trade_suspended_by_user, "False");
        assert_eq!(&deserialized.trading_blocked, "False");
        assert_eq!(&deserialized.transfers_blocked, "False");
    }
}