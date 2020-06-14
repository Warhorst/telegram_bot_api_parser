use std::fmt;

use serde::export::Formatter;

use crate::api_parser::scraper::Scraper;
use crate::api_parser::tables::{DtoTables, MethodTables};
use crate::raw_api::{RawApi, RawDtos, RawMethods};
use crate::raw_api::raw_dto::RawDto;
use crate::raw_api::raw_field::RawField;

pub mod scraper;
mod tables;

pub struct ApiParser;

type ParseResult = Result<RawApi, ApiParseError>;

impl ApiParser {
    pub fn parse<S: Scraper>(&self, scraper: S) -> ParseResult {
        let raw_dtos = self.parse_dto_tables(scraper.get_dto_tables());
        let raw_methods = self.parse_method_tables(scraper.get_method_tables());
        Ok(RawApi {
            raw_dtos,
            raw_methods
        })
    }

    fn parse_dto_tables(&self, dto_tables: DtoTables) -> RawDtos {
        unimplemented!()
    }

    fn parse_method_tables(&self, method_tables: MethodTables) -> RawMethods {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum ApiParseError {}

impl std::error::Error for ApiParseError {}

impl std::fmt::Display for ApiParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}