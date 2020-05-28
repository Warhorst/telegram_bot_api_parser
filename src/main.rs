use std::fs::File;

use crate::api_parser::ApiParser;
use crate::code_generator::CodeGenerator;
use crate::code_generator::CodeGeneratorImpl;
use crate::code_generator::configuration_reader::ConfigurationReader;
use crate::code_generator::renderer::RendererImpl;
use crate::code_generator::resolver::ResolverImpl;
use crate::code_writer::CodeWriter;

pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;
pub mod code_writer;

fn main() {
    let reader = ConfigurationReader;
    let configuration = reader.read().unwrap();

    let parser = ApiParser;
    let raw_api = parser.parse(File::open("html/api.html").unwrap()).unwrap();

    let resolver: ResolverImpl<RendererImpl> = ResolverImpl::new(configuration.clone());
    let generator = CodeGeneratorImpl::new(configuration.clone(), resolver).unwrap();
    let target_files = generator.generate(raw_api).unwrap();

    let writer = CodeWriter;
    writer.write(target_files).unwrap();
}