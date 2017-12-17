extern crate rusty_gdax;

use rusty_gdax::RESTClient;
use rusty_gdax::products::{GetProductOrderBook, Level};

fn main() {
    let mut test_client = RESTClient::default();
    let order_book = test_client.request(
        &GetProductOrderBook::new(
            String::from("BTC-USD"),
            Level::Level2
        )
    );

    println!("{:?}", order_book);
}