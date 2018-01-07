extern crate chrono;
extern crate gdax_rs;

use chrono::{TimeZone, Utc};

use gdax_rs::RESTClient;
use gdax_rs::products::GetHistoricRates;

fn main() {
    let mut test_client = RESTClient::default();
    let candles = test_client.request(&GetHistoricRates::new(
        String::from("BTC-USD"),
        Utc.ymd(2017, 12, 21).and_hms_micro(10, 10, 10, 10),
        Utc.ymd(2017, 12, 21).and_hms_micro(10, 15, 15, 10),
        1,
    ));

    println!("{:?}", candles);
}
