extern crate gdax_rs;
extern crate tokio_core;

// use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::time::GetTime;


#[tokio::main]
async fn main() {
    // let mut core = Core::new().unwrap();
    // let handle = core.handle();

    let mut test_client = RESTClient::default();
    let products = test_client.send_request(&GetTime::new()).await.unwrap();

    println!("{:?}", products);
}
