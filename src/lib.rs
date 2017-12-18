extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate itertools;
extern crate mockito;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod products;
mod url;
mod rest_client;

pub use rest_client::RESTClient;
