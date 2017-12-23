extern crate rusty_gdax;

use rusty_gdax::RESTClient;
use rusty_gdax::products::GetTrades;

fn main() {
    let mut test_client = RESTClient::default();
    let product_ticker = test_client.request(&GetTrades::new(String::from("BTC-USD")));

    println!("{:?}", product_ticker);
}
