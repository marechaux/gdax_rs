extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::time::GetTime;

fn main() {
    let mut test_client = RESTClient::default();
    let products = test_client.request(&GetTime::new());

    println!("{:?}", products);
}
