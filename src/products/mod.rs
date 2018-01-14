//! This module contains all `EndPointRequest` and there response type of GDAX API doc under
//! "Market Data/Product" section (<https://docs.gdax.com/#products>)

mod get_products;
mod get_product_order_book;
mod get_product_ticker;
mod get_trades;
mod get_historic_rates;
mod get_24hr_stats;

pub use self::get_products::{GetProducts, Product};
pub use self::get_product_order_book::{GetProductOrderBook, Level};
pub use self::get_product_ticker::{GetProductTicker, Ticker};
pub use self::get_trades::{GetTrades, Side, Trade};
pub use self::get_historic_rates::{Candle, GetHistoricRates};
pub use self::get_24hr_stats::{Get24hrStats, Stats};
