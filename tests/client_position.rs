extern crate alpaca;

#[cfg(test)]
mod tests {
    use std::env;
    use alpaca::client::Client;
    use alpaca::client::AccountType::PAPER;
    use std::process::exit;
    use std::borrow::Borrow;

    #[test]
    pub fn get_open_positions() {

    }

    #[test]
    pub fn get_open_position() {

    }

    #[test]
    pub fn close_all_positions() {

    }

    #[test]
    pub fn close_position() {

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