extern crate chrono;
extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;
use chrono::{TimeZone, Utc};

use gdax_rs::RESTClient;
use gdax_rs::products::GetHistoricRates;

#[tokio::main]
async fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default();
    let candles = test_client.send_request(&GetHistoricRates::new(
        String::from("BTC-USD"),
        Utc.ymd(2017, 12, 21).and_hms_micro(10, 10, 10, 10),
        Utc.ymd(2017, 12, 21).and_hms_micro(10, 15, 15, 10),
        60,
    )).await.unwrap();

    println!("{:?}", candles);
}
