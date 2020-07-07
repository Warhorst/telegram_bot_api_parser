use std::fs::File;

use crate::api_parser::ApiParser;
use crate::api_parser::scraper::ScraperImpl;
use crate::code_generator::CodeGenerator;
use crate::code_generator::configuration_reader::ConfigurationReader;
use crate::code_generator::renderer::{Renderer, RendererImpl};
use crate::code_writer::CodeWriter;
use crate::api_parser::type_parser::TypeParserImpl;

pub struct ApiParserApplication;

impl ApiParserApplication {
    pub fn run(&self) {
        let configuration = match ConfigurationReader.read() {
            Ok(configuration) => configuration,
            Err(error) => {
                eprintln!("Error while reading the configuration file: {}", error);
                return;
            }
        };

        let api_html = match File::open("html/api.html") {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error while opening the api HTML file: {}", error);
                return;
            }
        };

        let scraper = match ScraperImpl::from_html(api_html) {
            Ok(scraper) => scraper,
            Err(error) => {
                eprintln!("An error occurred while scraping the HTML: {}", error);
                return;
            }
        };
        let parser = ApiParser::new(scraper, TypeParserImpl);
        let raw_api = parser.parse();

        let generator = CodeGenerator::new(configuration.clone(), RendererImpl::from_configuration(configuration).unwrap());
        let target_files = match generator.generate(raw_api) {
            Ok(target_files) => target_files,
            Err(error) => {
                eprintln!("An error occurred while generating the code: {}", error);
                return;
            }
        };

        if let Err(error) = CodeWriter.write(target_files) {
            eprintln!("An error ocurred while writing the code: {}", error)
        }
    }
}

