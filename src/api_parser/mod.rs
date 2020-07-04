use crate::api_parser::scraper::Scraper;
use crate::api_parser::tables::{DtoTables, MethodTables, DtoTable, DtoRow, MethodTable, MethodRow};
use crate::raw_api::{RawApi, RawDtos, RawMethods};
use crate::raw_api::raw_dto::RawDto;
use crate::raw_api::raw_field::RawField;
use crate::raw_api::raw_method::RawMethod;
use crate::raw_api::raw_parameter::RawParameter;
use crate::api_parser::type_parser::TypeParser;

pub mod scraper;
pub mod type_parser;
mod tables;

pub struct ApiParser<S: Scraper, T: TypeParser> {
    scraper: S,
    type_parser: T,
}

impl<S: Scraper, T: TypeParser> ApiParser<S, T> {
    pub fn new(scraper: S, type_parser: T) -> Self {
        ApiParser {
            scraper,
            type_parser
        }
    }

    pub fn parse(&self) -> RawApi {
        let raw_dtos = self.parse_dto_tables(self.scraper.get_dto_tables());
        let raw_methods = self.parse_method_tables(self.scraper.get_method_tables());
        RawApi {
            raw_dtos,
            raw_methods,
        }
    }

    fn parse_dto_tables(&self, dto_tables: DtoTables) -> RawDtos {
        let mut raw_dtos = Vec::new();
        for table in dto_tables {
            raw_dtos.push(self.parse_table_to_dto(table))
        }
        raw_dtos
    }

    fn parse_table_to_dto(&self, table: DtoTable) -> RawDto {
        let mut fields = Vec::new();
        for row in table.rows {
            fields.push(self.parse_row_to_field(row))
        }
        RawDto {
            name: table.name,
            fields,
        }
    }

    fn parse_row_to_field(&self, row: DtoRow) -> RawField {
        let name = row.field_string;
        let field_type = self.type_parser.parse_field_type(row.type_string, row.description_string);
        RawField {
            name,
            field_type
        }
    }

    fn parse_method_tables(&self, method_tables: MethodTables) -> RawMethods {
        let mut raw_methods = Vec::new();
        for table in method_tables {
            raw_methods.push(self.parse_table_to_method(table))
        }
        raw_methods
    }

    fn parse_table_to_method(&self, table: MethodTable) -> RawMethod {
        let mut parameters = Vec::new();
        for row in table.rows {
            parameters.push(self.parse_row_to_parameter(row))
        }
        RawMethod {
            name: table.name,
            parameters,
        }
    }

    fn parse_row_to_parameter(&self, row: MethodRow) -> RawParameter {
        let name = row.parameter_string;
        let parameter_type = self.type_parser.parse_parameter_type(row.type_string, row.required_string);
        RawParameter {
            name,
            parameter_type
        }
    }
}