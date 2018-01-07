extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::products::Get24hrStats;

fn main() {
    let mut test_client = RESTClient::default();
    let product_ticker = test_client.request(&Get24hrStats::new(String::from("BTC-USD")));

    println!("{:?}", product_ticker);
}
