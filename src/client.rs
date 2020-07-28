use crate::auth::Auth;
use std::fmt::{Debug, Display};
use crate::account::Account;
use crate::client::AccountType::PAPER;
use crate::client::AccountType::LIVE;
use crate::order::{Order};
use uuid::Uuid;

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

        pub fn cancel_all_ordres(&self) {
            Order::cancel_all(&self);
        }


        pub fn get_url(&self) -> String {
            match &self.account_type {
                PAPER => "https://paper-api.alpaca.markets/v2/".to_owned(),
                LIVE => "https://api.alpaca.markets/v2/".to_owned()
            }
        }
    }