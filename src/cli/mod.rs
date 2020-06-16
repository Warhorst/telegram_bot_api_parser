use std::fs::File;

use crate::api_parser::ApiParser;
use crate::api_parser::scraper::ScraperImpl;
use crate::cli::api_parser_arguments::ApiParserArguments;
use crate::code_generator::{CodeGenerator, CodeGeneratorImpl};
use crate::code_generator::configuration_reader::ConfigurationReader;
use crate::code_generator::renderer::{Renderer, RendererImpl};
use crate::code_writer::CodeWriter;
use crate::api_parser::type_parser::TypeParserImpl;

mod api_parser_arguments;

pub struct ApiParserApplication;

impl ApiParserApplication {
    pub fn run(&self) {
        let arguments = ApiParserArguments::parse_arguments();
        println!("load local: {}", arguments.load_local);

        let reader = ConfigurationReader;
        let configuration = reader.read().unwrap();

        let scraper = ScraperImpl::from_html(File::open("html/api.html").unwrap()).unwrap();
        let type_parser = TypeParserImpl;

        let parser = ApiParser::new(scraper, type_parser);
        let raw_api = parser.parse();

        let generator = CodeGeneratorImpl::new(configuration.clone(), RendererImpl::from_configuration(configuration));
        let target_files = generator.generate(raw_api).unwrap();

        let writer = CodeWriter;
        writer.write(target_files).unwrap();
    }
}

