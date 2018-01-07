extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::currencies::GetCurrencies;

fn main() {
    let mut test_client = RESTClient::default();
    let products = test_client.request(&GetCurrencies::new());

    println!("{:?}", products);
}
