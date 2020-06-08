use crate::cli::ApiParserApplication;

pub mod util;
pub mod raw_api;
pub mod api_parser;
pub mod code_generator;
pub mod code_writer;
pub mod cli;

fn main() {
    ApiParserApplication.run();
}