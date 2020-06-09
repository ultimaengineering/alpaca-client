use crate::auth::Auth;
use std::fmt::{Debug, Display};
use std::time::Duration;
use reqwest::StatusCode;
use reqwest::header::USER_AGENT;
use crate::account::Account;
use crate::client::AccountType::PAPER;
use crate::client::AccountType::LIVE;
use crate::account;
use std::error::Error;
use serde_json::{Map, Value};

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
            let successful_login = Self::login(&auth);

            match successful_login {
                Ok(l) => println!("{:?}", l),
                Err(e) => println!("{:?}", e)
            }

            let client = Client {
                auth,
                account_type
            };
            return client;
        }

        pub fn login(auth: &Auth) -> Result<(), Box<dyn std::error::Error>> {
            let _client = reqwest::blocking::Client::new();
            let mut res = _client.get("account")
                .header(USER_AGENT, "foo")
                .header("APCA-API-KEY-ID", &auth.access_key)
                .header("APCA-API-SECRET-KEY", &auth.secret_key)
                .timeout(Duration::new(5, 0))
                .send()?;
            println!("Status: {}", res.status());
            println!("Headers:\n{:?}", res.headers());
            // copy the response body directly to stdout
            res.copy_to(&mut std::io::stdout())?;
            println!("\n\nDone.");
            Ok(())
        }

        pub fn get_account(&self) {
            let _client = reqwest::blocking::Client::new();
            let mut url = Self::get_url(&self);
            url.push_str("account");
            let result: account::Account = _client.get(&url)
                .header("APCA-API-KEY-ID", &self.auth.access_key)
                .header("APCA-API-SECRET-KEY", &self.auth.secret_key).send().unwrap().json().unwrap();
            println!("{:?}", result);
        }

        pub fn get_url(&self) -> String {
            match &self.account_type {
                PAPER => "https://paper-api.alpaca.markets/v2/".to_owned(),
                LIVE => "https://api.alpaca.markets/v2/".to_owned()
            }
        }
    }