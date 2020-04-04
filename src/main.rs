use select::document::Document;
use select::predicate::{Name, Predicate};
use std::fs::File;
use crate::api_parser::bot_api_parser::BotApiParser;
use crate::code_generator::template::template_code_generator::TemplateCodeGenerator;

#[macro_use]
extern crate cfg_if;

#[macro_use]
pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;

fn main() {
    let parser = BotApiParser{};
    let raw_api = parser.parse(File::open("html/api.html").unwrap()).unwrap();
}