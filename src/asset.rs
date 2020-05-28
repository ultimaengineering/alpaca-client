
pub struct asset {
    pub id: UUID, //Asset ID.
    pub class: String, //“us_equity”
    pub exchange: String, //AMEX, ARCA, BATS, NYSE, NASDAQ or NYSEARCA
    pub symbol: String, //
    pub status: String, //active or inactive
    pub tradable: bool, //Asset is tradable on Alpaca or not.
    pub marginable: bool, //Asset is marginable or not.
    pub shortable: bool, //Asset is shortable or not.
    pub easy_to_borrow: bool //Asset is easy-to-borrow or not (filtering for easy_to_borrow = True
    // is the best way to check whether the name is currently available to short at Alpaca).
}