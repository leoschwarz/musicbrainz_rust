// Apparently this is recommended when using error-chain.
#![recursion_limit = "1024"]

// TODO: Remove before stable release.
#![allow(dead_code)]

#[macro_use]
extern crate error_chain;
extern crate regex;
extern crate reqwest_mock;
extern crate uuid;
extern crate url;
extern crate xpath_reader;

pub mod errors;
pub use self::errors::*;

pub mod client;
pub mod entities;

mod util;

#[cfg(feature = "rusqlite")]
extern crate rusqlite;
#[cfg(feature = "rusqlite")]
mod rusqlite_support;
