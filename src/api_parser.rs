use std::fmt;

use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate, Text};
use serde::export::Formatter;

use crate::api_parser::ApiParseError::{DocumentError, ExtractDTOError, NoValidTableError, TableWithoutNameError};
use crate::api_parser::TableContentType::{DTO, Method};
use crate::raw_api::raw_dto::RawDto;
use crate::raw_api::raw_field::RawField;
use crate::raw_api::RawApi;

/// Extracts the raw API from the HTML.
/// The current implementation assumes the following HTML-Scheme:
///
/// Every DTO/method has zero(placeholder like CallbackGame) or one HTML-table, which holds information
/// about its fields/parameters.
/// DTO tables have the header Field/Type/Description.
/// Method tables have the header Parameter/Type/Required/Description
/// Before every table, a single h4 with the name of the DTO/method is located.
/// Between the table and the h4 may be other content like descriptions.
/// h4s aren't exclusively used to introduce DTOs/methods..
pub struct ApiParser;

type ParseResult = Result<RawApi, ApiParseError>;

impl ApiParser {
    const H4: &'static str = "h4";
    const TABLE: &'static str = "table";
    const TABLE_BODY: &'static str = "tbody";
    const TABLE_HEADER: &'static str = "th";
    const TABLE_ROW: &'static str = "tr";
    const TABLE_DATA: &'static str = "td";
    const EMPHASIS: &'static str = "em";
    const ARRAY_OF: &'static str = "Array of ";
    const DTO_TABLE_COLUMNS: usize = 3;
    const METHOD_TABLE_COLUMNS: usize = 4;

    pub fn parse<R: std::io::Read>(&self, api_html: R) -> ParseResult {
        let mut raw_api = RawApi::new();
        let document = Document::from_read(api_html)?;
        let mut current_dto_name = None;

        for node in document.find(Name(ApiParser::H4).or(Name(ApiParser::TABLE))) {
            match node.name().unwrap() {
                Self::H4 => current_dto_name = self.get_node_text(&node),

                Self::TABLE => match (&current_dto_name, self.get_table_content_type(&node)?) {
                    (Some(dto_name), DTO) => {
                        raw_api.dtos.push(self.extract_dto(dto_name, &node)?);
                        current_dto_name = None
                    },

                    (Some(_), Method) => (),

                    (None, _) => return Err(TableWithoutNameError)
                },
                _ => ()
            }
        }

        Ok(raw_api)
    }

    fn extract_dto(&self, dto_name: &String, table_node: &Node) -> Result<RawDto, ApiParseError> {
        let mut fields = Vec::new();
        let table_bodies = table_node.find(Name(Self::TABLE_BODY)).collect::<Vec<Node>>();

        match table_bodies.get(0) {
            Some(table_body) => {
                for table_row in table_body.find(Name(Self::TABLE_ROW)) {
                    fields.push(self.extract_field(&table_row)?)
                }

                Ok(RawDto::new(dto_name.clone(), fields))
            },

            None => Err(ExtractDTOError(String::from("A table does not have a table body.")))
        }
    }

    fn extract_field(&self, table_row: &Node) -> Result<RawField, ApiParseError> {
        let data_nodes: Vec<Node> = table_row.find(Name(Self::TABLE_DATA)).collect();
        let field_node_option = data_nodes.get(0);
        let type_node_option = data_nodes.get(1);
        let description_node_option = data_nodes.get(2);

        match (field_node_option, type_node_option, description_node_option) {
            (Some(field_node), Some(type_node), Some(description_node)) => {
                let field_name = self.get_node_text(field_node).unwrap();
                let type_string = self.get_node_text(type_node).unwrap();
                let optional = description_node.find(Name(Self::EMPHASIS)).count() >= 1;

                Ok(RawField::new(field_name, type_string, optional))
            },

            _ => return Err(ExtractDTOError(String::from("A table row does not contain the expected three table data elements.")))
        }
    }

    fn get_node_text(&self, node: &Node) -> Option<String> {
        let text_nodes: Vec<Node> = node.find(Text).collect();

        match text_nodes.get(0) {
            None => None,

            Some(node) => match node.text().as_str() {
                Self::ARRAY_OF => match text_nodes.get(1) {
                    Some(array_text_node) => {
                        let mut array_type = String::from(node.text());
                        array_type.push_str(array_text_node.text().as_str());
                        Some(array_type)
                    },

                    None => None
                },

                _ => Some(node.text())
            }
        }
    }

    fn get_table_content_type(&self, table_node: &Node) -> Result<TableContentType, ApiParseError> {
        match table_node.find(Name(Self::TABLE_HEADER)).count() {
            Self::DTO_TABLE_COLUMNS => Ok(DTO),
            Self::METHOD_TABLE_COLUMNS => Ok(Method),
            columns => Err(NoValidTableError(columns))
        }
    }
}

enum TableContentType {
    DTO,
    Method
}

#[derive(Debug)]
pub enum ApiParseError {
    DocumentError(std::io::Error),
    TableWithoutNameError,
    NoValidTableError(usize),
    ExtractDTOError(String)
}

impl std::error::Error for ApiParseError {}

impl std::fmt::Display for ApiParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocumentError(error) => error.fmt(f),
            ApiParseError::TableWithoutNameError => writeln!(f, "The HTML contains a table without a h4 as predecessor."),
            ApiParseError::NoValidTableError(columns) => writeln!(f, "Unable to get the content type of the given HTML table. Columns: {}", columns),
            ApiParseError::ExtractDTOError(message) => writeln!(f, "Could not extract a DTO from the given table node. Reason: {}", message)
        }
    }
}

impl From<std::io::Error> for ApiParseError {
    fn from(error: std::io::Error) -> Self {
        DocumentError(error)
    }
}

#[cfg(test)]
mod tests {
    use select::document::Document;
    use select::node::Node;
    use select::predicate::Name;

    use crate::api_parser::ApiParser;
    use crate::raw_api::field_type::FieldType;

    const DTO_NAME: &'static str = "FooDTO";
    const FIELD_NAME: &'static str = "foo";
    const FIELD_TYPE: &'static str = "Foo";
    const TABLE_ROW: &'static str = r#"
        <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>Headline</h4>
        <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDTO</h4>
        <table>
            <thead>
                <tr>
                    <th>Field</th>
                    <th>Type</th>
                    <th>Description</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>foo</td>
                    <td>Foo</td>
                    <td>foo foo.</td>
                </tr>
            </tbody>
        </table>
    "#;

    const TABLE_ROW_OPTIONAL: &'static str = r#"
        <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>Headline</h4>
        <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDTO</h4>
        <table>
            <thead>
                <tr>
                    <th>Field</th>
                    <th>Type</th>
                    <th>Description</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>foo</td>
                    <td>Foo</td>
                    <td><em>Optional</em>.foo foo.</td>
                </tr>
            </tbody>
        </table>
    "#;

    /// Tests whether a DTOField can be extracted from a given table row node.
    /// The field in the table is declared not optional.
    ///
    /// From the given table the second element with the name "tr" is taken and used for extraction.
    #[test]
    fn success_extract_field() {
        let document = Document::from(TABLE_ROW);
        let rows = document.find(Name("tr")).collect::<Vec<Node>>();
        let table_row = rows.get(1).unwrap();

        let field = ApiParser {}.extract_field(table_row).unwrap();

        assert_eq!(field.name, String::from(FIELD_NAME));
        assert_eq!(field.field_type, create_expected_field_type(false));
    }

    /// Tests whether a DTOField can be extracted from a given table row node.
    /// The field in the table is declared optional.
    ///
    /// From the given table the second element with the name "tr" is taken and used for extraction.
    #[test]
    fn success_extract_optional_field() {
        let document = Document::from(TABLE_ROW_OPTIONAL);
        let rows: Vec<Node> = document.find(Name("tr")).collect();
        let table_row = rows.get(1).unwrap();

        let field = ApiParser {}.extract_field(table_row).unwrap();

        assert_eq!(field.name, String::from(FIELD_NAME));
        assert_eq!(field.field_type, create_expected_field_type(true));
    }

    /// Tests whether a dto can be extracted from a given html table.
    #[test]
    fn success_extract_dto() {
        let document = Document::from(TABLE_ROW);
        let tables = document.find(Name("table")).collect::<Vec<Node>>();
        let table = tables.get(0).unwrap();
        let dto_name = String::from(DTO_NAME);

        let dto = ApiParser {}.extract_dto(&dto_name, table).unwrap();

        assert_eq!(dto.name, dto_name);
        assert_eq!(dto.fields.len(), 1);
    }

    #[test]
    fn success_parse() {
        let bytes = TABLE_ROW.as_bytes();

        let raw_api = ApiParser {}.parse(bytes).unwrap();

        let dtos = raw_api.dtos;
        assert_eq!(dtos.len(), 1);
        let dto = dtos.get(0).unwrap();
        assert_eq!(dto.name, String::from(DTO_NAME));
    }

    /// Creates the expected FieldType of an extracted DTOField.
    /// The type can be optonal or not, based on testcase.
    fn create_expected_field_type(optional: bool) -> FieldType {
        let field_type = FieldType::DTO(String::from(FIELD_TYPE));

        match optional {
            false => field_type,
            true => FieldType::Optional(Box::new(field_type))
        }
    }
}