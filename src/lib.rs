// Apparently this is recommended when using error-chain.
#![recursion_limit = "1024"]
// TODO: Remove before stable release.
#![allow(dead_code)]
// TODO: Remove again later.
#![allow(unused)]

extern crate isolang;
extern crate regex;
extern crate reqwest_mock;
extern crate uuid;
extern crate url;
extern crate xpath_reader;

mod error;
pub use self::error::Error;

pub mod client;
pub mod entities;
pub mod search;

mod util;

#[cfg(feature = "rusqlite")]
extern crate rusqlite;
#[cfg(feature = "rusqlite")]
mod rusqlite_support;
