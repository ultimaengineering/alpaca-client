extern crate alpaca_client;

#[cfg(test)]
mod tests {
    use std::env;
    use alpaca_client::client::Client;
    use alpaca_client::client::AccountType::PAPER;
    use std::process::exit;

    #[test]
    pub fn get_account_configuration() {
        let client = get_client();
        let ac = client.get_account_configuration();
        assert_eq!(ac.trade_confirm_email, "all");
        assert_eq!(ac.dtbp_check, "entry");
        assert_eq!(ac.no_shorting, false);
        assert_eq!(ac.suspend_trade, false);
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