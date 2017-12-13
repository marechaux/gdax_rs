extern crate rusty_gdax;

use rusty_gdax::RESTConnector;
use rusty_gdax::products::get_product_order_book::{GetProductOrderBook, Level};

fn main() {
    let mut test_connector = RESTConnector::default();
    let order_book = test_connector.request(
        &GetProductOrderBook{
            product_id: String::from("BTC-USD"),
            level: Level::Level2
        }
    );

//    Product::get();
    println!("{:?}", order_book);
}