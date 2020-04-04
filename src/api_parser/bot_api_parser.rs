use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
use select::document::Document;
use select::predicate::{Name, Predicate, Text};
use select::node::Node;
use crate::api_parser::bot_api_parser::TableContentType::{DTO, Method};
use crate::raw_api::bot_dto::BotDTO;
use serde::export::Formatter;
use std::fmt;
use crate::api_parser::bot_api_parser::ApiParseError::{DocumentError, ExtractDTOError, NoValidTableError, TableWithoutNameError};
use crate::raw_api::dto_field::DTOField;

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
pub struct BotApiParser {}

type ParseResult = Result<TelegramBotApiRaw, ApiParseError>;

impl BotApiParser {
    const H4: &'static str = "h4";
    const TABLE: &'static str = "table";
    const TABLE_BODY: &'static str = "tbody";
    const TABLE_HEADER: &'static str = "th";
    const TABLE_ROW: &'static str = "tr";
    const TABLE_DATA: &'static str = "td";
    const EMPHASIS: &'static str = "em";
    const DTO_TABLE_COLUMNS: usize = 3;
    const METHOD_TABLE_COLUMNS: usize = 4;

    /// Parses a given Read(the HTML) to a raw api.
    /// This is implemented by iterating over every node in the HTML. If a h4 is found, followed by a table,
    /// the text of the h4 is converted to a DTOs name and the table content to its fields.
    pub fn parse<R: std::io::Read>(&self, api_html: R) -> ParseResult {
        let mut result = TelegramBotApiRaw::new();
        let document = Document::from_read(api_html)?;
        let mut current_dto_name = None;

        for node in document.find(Name(BotApiParser::H4).or(Name(BotApiParser::TABLE))) {
            match node.name().unwrap() {
                Self::H4 => current_dto_name = self.get_node_text(&node),

                Self::TABLE => match (&current_dto_name, self.get_table_content_type(&node)?) {
                    (Some(dto_name), DTO) => {
                        result.add_dto(self.extract_dto(dto_name, &node)?);
                        current_dto_name = None
                    },

                    (Some(_), Method) => (),

                    (None, _) => return Err(TableWithoutNameError)
                },
                _ => ()
            }
        }

        Ok(result)
    }

    /// Extracts a BotDTO from a given HTML node that contains a table.
    /// This is implemented by taking the table body and iterating over every row, extracting the data from it.
    fn extract_dto(&self, dto_name: &String, table_node: &Node) -> Result<BotDTO, ApiParseError> {
        let mut fields = Vec::new();
        let table_bodies = table_node.find(Name(Self::TABLE_BODY)).collect::<Vec<Node>>();

        match table_bodies.get(0) {
            Some(table_body) => {
                for table_row in table_body.find(Name(Self::TABLE_ROW)) {
                    fields.push(self.extract_field(&table_row)?)
                }

                Ok(BotDTO::new(dto_name.clone(), fields))
            },

            None => Err(ExtractDTOError(String::from("A table does not have a table body.")))
        }
    }

    /// Extracts a DTOField from a given table row node. From this row, three table data nodes are retrieved.
    /// The first node contains the field name, the second its type and the third
    /// the description which indicates if the field is optional.
    ///
    /// The description may contain at least one em-element with the word "Optional", indicating the field is optional.
    fn extract_field(&self, table_row: &Node) -> Result<DTOField, ApiParseError> {
        let data_nodes: Vec<Node> = table_row.find(Name(Self::TABLE_DATA)).collect();
        let field_node_option = data_nodes.get(0);
        let type_node_option = data_nodes.get(1);
        let description_node_option = data_nodes.get(2);

        match (field_node_option, type_node_option, description_node_option) {
            (Some(field_node), Some(type_node), Some(description_node)) => {
                let field_name = self.get_node_text(field_node).unwrap();
                let type_string = self.get_node_text(type_node).unwrap();
                let optional = description_node.find(Name(Self::EMPHASIS)).count() >= 1;

                Ok(DTOField::new(field_name, type_string, optional))
            },

            _ => return Err(ExtractDTOError(String::from("A table row does not contain the expected three table data elements.")))
        }
    }

    /// Returns the text from the first Text-node in the given node.
    /// For example calling this method on <foo><bar>Text</bar></foo> returns "Text".
    fn get_node_text(&self, node: &Node) -> Option<String> {
        let text_nodes: Vec<Node> = node.find(Text).collect();

        match text_nodes.get(0) {
            None => None,
            Some(node) => Some(node.text())
        }
    }

    /// Returns whether the given node holds a table that describes the
    /// fields of a DTO or the parameters of a method.
    fn get_table_content_type(&self, table_node: &Node) -> Result<TableContentType, ApiParseError> {
        match table_node.find(Name(Self::TABLE_HEADER)).count() {
            Self::DTO_TABLE_COLUMNS => Ok(DTO),
            Self::METHOD_TABLE_COLUMNS => Ok(Method),
            columns => Err(NoValidTableError(columns))
        }
    }
}

/// Type of a given tables content.
/// Tables only contain either DTO fields or method parameters.
enum TableContentType {
    DTO,
    Method
}

/// Possible errors of an api parse operation.
///
/// DocumentError - Any error from opening a select::document::Document. Holds the error in a Box (any error can be returned).
/// NoValidTableError - Error that can occur when getting the TableContentType of a node. Holds the amount of table columns.
/// ExtractDTOError - Error that can occur when extracting a DTO from a Node
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
    use select::node::Node;
    use select::document::Document;
    use select::predicate::Name;
    use crate::api_parser::bot_api_parser::BotApiParser;
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

        let field = BotApiParser{}.extract_field(table_row).unwrap();

        assert_eq!(*field.get_name(), String::from(FIELD_NAME));
        assert_eq!(*field.get_field_type(), create_expected_field_type(false));
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

        let field = BotApiParser{}.extract_field(table_row).unwrap();

        assert_eq!(*field.get_name(), String::from(FIELD_NAME));
        assert_eq!(*field.get_field_type(), create_expected_field_type(true));
    }

    /// Tests whether a dto can be extracted from a given html table.
    #[test]
    fn success_extract_dto() {
        let document = Document::from(TABLE_ROW);
        let tables = document.find(Name("table")).collect::<Vec<Node>>();
        let table = tables.get(0).unwrap();
        let dto_name = String::from(DTO_NAME);

        let dto = BotApiParser{}.extract_dto(&dto_name, table).unwrap();

        assert_eq!(*dto.get_name(), dto_name);
        let fields = dto.get_fields();
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn success_parse() {
        let bytes = TABLE_ROW.as_bytes();

        let raw_api = BotApiParser{}.parse(bytes).unwrap();

        let dtos = raw_api.get_bot_dtos();
        assert_eq!(dtos.len(), 1);
        let dto = dtos.get(0).unwrap();
        assert_eq!(*dto.get_name(), String::from(DTO_NAME));
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