#[macro_use] extern crate cfg_if;

#[macro_use] pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;
#[cfg(test)] pub mod test_support;

fn main() {
    println!("wololo")
}