extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::products::{GetProductOrderBook, Level};

fn main() {
    let mut test_client = RESTClient::default();
    let order_book = test_client.request(&GetProductOrderBook::new(
        String::from("BTC-USD"),
        Level::Level2,
    ));

    println!("{:?}", order_book);
}
