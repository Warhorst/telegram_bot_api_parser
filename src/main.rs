use std::fs::File;
use crate::api_parser::ApiParser;
use crate::code_generator::template::TemplateCodeGenerator;
use crate::code_generator::template::template_reader::TemplateReader;
use crate::code_generator::CodeGenerator;
use crate::code_writer::CodeWriter;

pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;
pub mod code_writer;

fn main() {
    let reader = TemplateReader;
    let template = reader.read().unwrap();

    let parser = ApiParser;
    let raw_api = parser.parse(File::open("html/api.html").unwrap()).unwrap();

    let generator = TemplateCodeGenerator::new(template);
    let target_files = generator.generate(raw_api).unwrap();

    let writer = CodeWriter;
    writer.write(target_files).unwrap();
}