#![feature(custom_derive)]

extern crate rustc_serialize;
extern crate hyper;
extern crate serde_json;
extern crate url;
extern crate regex;

pub mod stats;
pub mod queries;
pub mod err;
pub mod scrape;
pub mod parse;
pub mod constants;
pub mod tests;
