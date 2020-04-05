use select::document::Document;
use select::predicate::{Name, Predicate};
use std::fs::File;
use crate::api_parser::bot_api_parser::BotApiParser;
use crate::code_generator::template::template_code_generator::TemplateCodeGenerator;
use crate::code_generator::template::template_reader::TemplateReader;
use crate::code_generator::code_generator::CodeGenerator;
use crate::code_generator::code_writer::code_writer::CodeWriter;

#[macro_use]
extern crate cfg_if;

#[macro_use]
pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;

fn main() {
    let reader = TemplateReader;
    let template = reader.read().unwrap();

    let parser = BotApiParser;
    let raw_api = parser.parse(File::open("html/api.html").unwrap()).unwrap();

    let generator = TemplateCodeGenerator::new(template);
    let target_files = generator.generate(raw_api).unwrap();

    let writer = CodeWriter;
    writer.write(target_files).unwrap();
}