use std::str::FromStr;
use std::fmt::Display;

use serde::de;

/// Gdax return the floats values as strings, we need to use the `FromStr` trait to
/// deserialize the string.
///
/// Taken from <https://stackoverflow.com/documentation/rust/1170/serde#t=201708271607008933769/>
pub fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: de::Deserializer<'de>,
{
    let s: String = de::Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
