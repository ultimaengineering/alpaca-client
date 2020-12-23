use crate::auth::Auth;
use std::fmt::{Debug, Display};
use crate::account::Account;
use crate::client::AccountType::PAPER;
use crate::client::AccountType::LIVE;
use crate::order::{Order};
use uuid::Uuid;
use crate::position::Position;
use crate::clock::Clock;
use crate::calendar::Calendar;
use crate::asset::Asset;
use crate::account_configuration::AccountConfiguration;
use crate::bar::Bar;
use crate::bar::BarRequest;
use std::collections::HashMap;


pub enum AccountType {
    PAPER,
    LIVE
}

pub struct Client {
    pub auth: Auth,
    pub account_type: AccountType,
}


trait AuthError: Debug + Display {
    fn cause(&self) -> Option<&dyn AuthError> { None }
}

    impl Client {
        pub fn new(access: String, secret: String, account_type: AccountType) -> Client {
            let auth = Auth {access_key: access, secret_key: secret};
            let client = Client {
                auth,
                account_type
            };
            return client;
        }

        pub fn get_account(&self) -> Account { return Account::get_account(&self); }

        pub fn get_all_orders(&self) -> Vec<Order> { return Order::get_all(&self) }

        pub fn place_order(&self, _order: Order) -> Order {
            return Order::place(&self, _order);
        }

        pub fn get_order(&self, id: Uuid) -> Order { return Order::get(&self, id); }

        pub fn replace_order(&self, _order: Order) -> Order {
            return Order::replace(&self, _order);
        }

        pub fn cancel_order(&self, id: Uuid) {
            return Order::cancel(&self, id);
        }

        pub fn cancel_all_orders(&self) {
            Order::cancel_all(&self);
        }

        pub fn get_open_positions(&self) -> Vec<Position> {
            return Position::get_all(&self);
        }

        pub fn get_open_position(&self, symbol: String) -> Position {
            return Position::get(&self, symbol);
        }

        pub fn close_all_positions(&self) {
            Position::close_all(&self);
        }

        pub fn close_position(&self, symbol: String) {
            Position::close(&self, symbol);
        }

        pub fn get_clock(&self) -> Clock {
            return Clock::get(&self);
        }

        pub fn get_calender(&self) -> Calendar {
            return Calendar::get(&self);
        }

        pub fn get_assets(&self) -> Vec<Asset> {
            return Asset::get_all(&self)
        }

        pub fn get_asset(&self, symbol: String) -> Asset {
            return Asset::get(&self, symbol);
        }

        pub fn get_account_configuration(&self) -> AccountConfiguration {
            return AccountConfiguration::get(&self)
        }

        pub fn update_account_configuration(&self, account: AccountConfiguration) -> AccountConfiguration {
            return AccountConfiguration::update(&self, account);
        }

        pub fn get_bar(&self, bar_request: BarRequest) -> HashMap<String, Vec<Bar>> {
            return Bar::get_bar(&self, bar_request);
        }

        pub fn get_url(&self) -> String {
            match &self.account_type {
                PAPER => "https://paper-api.alpaca.markets/v2/".to_owned(),
                LIVE => "https://api.alpaca.markets/v2/".to_owned()
            }
        }
    }