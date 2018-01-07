extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::products::GetTrades;

fn main() {
    let mut test_client = RESTClient::default();
    let product_ticker = test_client.request(&GetTrades::new(String::from("BTC-USD")));

    println!("{:?}", product_ticker);
}
