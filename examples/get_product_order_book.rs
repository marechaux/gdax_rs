extern crate gdax_rs;
extern crate tokio_core;

// use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::products::{GetProductOrderBook, Level};

#[tokio::main]
async fn main() {
    // let mut core = Core::new().unwrap();
    // let handle = core.handle();

    let mut test_client = RESTClient::default();
    let order_book = test_client.send_request(&GetProductOrderBook::new(
        String::from("BTC-USD"),
        Level::Level2,
    )).await.unwrap();

    println!("{:?}", order_book);
}
