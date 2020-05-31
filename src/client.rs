use crate::auth::Auth;
use std::fmt::{Debug, Display};
use std::time::Duration;
use std::collections::HashMap;
use std::error::Error;
use reqwest::header::USER_AGENT;

pub struct Client {
    pub auth: Auth,
}


trait AuthError: Debug + Display {
    fn cause(&self) -> Option<&dyn AuthError> { None }
}

    impl Client {
        pub fn new(access: String, secret: String) -> Client {
            let auth = Auth {access_key: access, secret_key: secret};
            let successful_login = Self::login(&auth);

            match successful_login {
                Ok(l) => println!("{:?}", l),
                Err(e) => println!("{:?}", e)
            }

            let client = Client {
                auth
            };
            return client;
        }

        pub fn login(auth: &Auth) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::blocking::Client::new();
            let mut res = client.get("https://paper-api.alpaca.markets/v2/account")
                .header(USER_AGENT, "foo")
                .timeout(Duration::new(5, 0))
                .send()?;
            println!("Status: {}", res.status());
            println!("Headers:\n{:?}", res.headers());
            // copy the response body directly to stdout
            res.copy_to(&mut std::io::stdout())?;
            println!("\n\nDone.");
            Ok(())
        }
    }