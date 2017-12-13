extern crate rusty_gdax;

use rusty_gdax::RESTConnector;
use rusty_gdax::products::get_products::{Product, GetProducts};

fn main() {
    let mut test_connector = RESTConnector::default();
    let products = test_connector.request(
        &GetProducts::new()
    );

//    Product::get();
    println!("{:?}", products);
}