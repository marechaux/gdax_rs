extern crate gdax_rs;

use gdax_rs::RESTClient;
use gdax_rs::products::GetProducts;

fn main() {
    let mut test_client = RESTClient::default();
    let products = test_client.request(&GetProducts::new());

    println!("{:?}", products);
}
