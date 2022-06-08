extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;
use tokio;

use gdax_rs::RESTClient;
use gdax_rs::products::GetTrades;

#[tokio::main]
async fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default();
    let product_ticker = test_client.send_request(&GetTrades::new(String::from("BTC-USD"))).await.unwrap();

    println!("{:?}", product_ticker);
}
