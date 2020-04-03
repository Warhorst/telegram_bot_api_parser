use select::document::Document;
use select::predicate::{Name, Predicate};
use std::fs::File;
use crate::api_parser::bot_api_parser::BotApiParser;

#[macro_use]
extern crate cfg_if;

#[macro_use]
pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;

fn main() {
    let real_document = Document::from_read(File::open("html/api.html").unwrap()).unwrap();

    real_document
        .find(Name("h4").or(Name("table")))
        .for_each(|node| println!("{:?}", node));
}