extern crate rusty_gdax;

use rusty_gdax::RESTClient;
use rusty_gdax::time::GetTime;

fn main() {
    let mut test_client = RESTClient::default();
    let products = test_client.request(&GetTime::new());

    println!("{:?}", products);
}
