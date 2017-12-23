extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate itertools;
extern crate mockito;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

pub mod products;
mod url;
mod rest_client;

pub use rest_client::RESTClient;
