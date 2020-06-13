use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate, Text};
use serde::export::Formatter;

use crate::api_parser::ApiParser;
use crate::api_parser::tables::{DtoRow, DtoTable, DtoTables, MethodRow, MethodTable, MethodTables, Table};

pub trait Scraper {
    fn get_dto_tables(&self) -> DtoTables;

    fn get_method_tables(&self) -> MethodTables;
}

pub type ScraperResult = Result<ScraperImpl, ScraperError>;

#[derive(Debug)]
pub struct ScraperImpl {
    tables: Vec<Table>
}

impl ScraperImpl {
    const H4: &'static str = "h4";
    const TABLE: &'static str = "table";
    const TABLE_BODY: &'static str = "tbody";
    const TABLE_HEADER: &'static str = "th";
    const TABLE_ROW: &'static str = "tr";
    const TABLE_DATA: &'static str = "td";
    const DTO_TABLE_COLUMNS: usize = 3;
    const METHOD_TABLE_COLUMNS: usize = 4;

    pub fn from_html<R: std::io::Read>(api_html: R) -> ScraperResult {
        let mut tables = Vec::new();
        let document = Document::from_read(api_html)?;
        let mut current_table_name = None;

        for node in document.find(Self::searched_nodes_predicate()) {
            match node.name().unwrap() {
                Self::H4 => current_table_name = Some(Self::get_node_text(&node)?),
                Self::TABLE => tables.push(Self::extract_table_from_node(&node, &current_table_name)?),
                _ => ()
            }
        }

        Ok(ScraperImpl {
            tables
        })
    }

    fn searched_nodes_predicate() -> impl Predicate {
        Name(ApiParser::H4).or(Name(ApiParser::TABLE))
    }

    fn get_node_text(node: &Node) -> Result<String, ScraperError> {
        let text_nodes: Vec<Node> = node.find(Text).collect();
        let mut text_node_iter = text_nodes.iter();
        let mut node_text = String::new();

        while let Some(text_node) = text_node_iter.next() {
            node_text.push_str(text_node.text().as_str())
        }

        match node_text.is_empty() {
            true => Err(ScraperError::EmptyTextNode),
            false => Ok(node_text)
        }
    }

    fn get_table_content_type(table_node: &Node) -> Result<TableContentType, ScraperError> {
        match table_node.find(Name(Self::TABLE_HEADER)).count() {
            Self::DTO_TABLE_COLUMNS => Ok(TableContentType::DTO),
            Self::METHOD_TABLE_COLUMNS => Ok(TableContentType::Method),
            columns => Err(ScraperError::InvalidTableColumns { columns })
        }
    }

    fn extract_table_from_node(table_node: &Node, current_table_name: &Option<String>) -> Result<Table, ScraperError> {
        let table_content_type = Self::get_table_content_type(table_node)?;
        match (table_content_type, current_table_name) {
            (TableContentType::DTO, Some(table_name)) => Ok(Self::extract_dto_table(table_name.clone(), table_node)?),
            (TableContentType::Method, Some(table_name)) => Ok(Self::extract_method_table(table_name.clone(), table_node)?),
            (_, None) => Err(ScraperError::TableWithoutHeader)
        }
    }

    fn extract_dto_table(dto_name: String, table_node: &Node) -> Result<Table, ScraperError> {
        let mut dto_table = DtoTable::new(dto_name.clone());
        let table_bodies = Self::get_table_body_nodes(table_node);

        match table_bodies.get(0) {
            Some(table_body) => {
                for table_row in table_body.find(Name(Self::TABLE_ROW)) {
                    dto_table.add_row(Self::extract_dto_row(&table_row)?);
                }
            }
            None => return Err(ScraperError::MissingTableBody { table_name: dto_name })
        }

        Ok(Table::Dto(dto_table))
    }

    fn extract_dto_row(table_row: &Node) -> Result<DtoRow, ScraperError> {
        let data_nodes = Self::get_table_data_nodes(table_row);
        let field_node_option = data_nodes.get(0);
        let type_node_option = data_nodes.get(1);
        let description_node_option = data_nodes.get(2);

        match (field_node_option, type_node_option, description_node_option) {
            (Some(field_node), Some(type_node), Some(description_node)) => {
                let field_string = Self::get_node_text(field_node)?;
                let type_string = Self::get_node_text(type_node)?;
                let description_string = Self::get_node_text(description_node)?;
                Ok(DtoRow::new(field_string, type_string, description_string))
            }
            _ => Err(ScraperError::InvalidTableRow)
        }
    }

    fn extract_method_table(method_name: String, table_node: &Node) -> Result<Table, ScraperError> {
        let mut method_table = MethodTable::new(method_name.clone());
        let table_bodies = Self::get_table_body_nodes(table_node);

        match table_bodies.get(0) {
            Some(table_body) => {
                for table_row in table_body.find(Name(Self::TABLE_ROW)) {
                    method_table.add_row(Self::extract_method_row(&table_row)?);
                }
            }
            None => return Err(ScraperError::MissingTableBody { table_name: method_name })
        }

        Ok(Table::Method(method_table))
    }

    fn extract_method_row(table_row: &Node) -> Result<MethodRow, ScraperError> {
        let data_nodes = Self::get_table_data_nodes(table_row);
        let parameter_node_option = data_nodes.get(0);
        let type_node_option = data_nodes.get(1);
        let required_node_option = data_nodes.get(2);
        let description_node_option = data_nodes.get(3);

        match (parameter_node_option, type_node_option, required_node_option, description_node_option) {
            (Some(parameter_node), Some(type_node), Some(required_node), Some(description_node)) => {
                let parameter_string = Self::get_node_text(parameter_node)?;
                let type_string = Self::get_node_text(type_node)?;
                let required_string = Self::get_node_text(required_node)?;
                let description_string = Self::get_node_text(description_node)?;
                Ok(MethodRow::new(parameter_string, type_string, required_string, description_string))
            }
            _ => Err(ScraperError::InvalidTableRow)
        }
    }

    fn get_table_body_nodes<'a>(node: &'a Node) -> Vec<Node<'a>> {
        node.find(Name(Self::TABLE_BODY)).collect()
    }

    fn get_table_data_nodes<'a>(node: &'a Node) -> Vec<Node<'a>> {
        node.find(Name(Self::TABLE_DATA)).collect()
    }
}

enum TableContentType {
    DTO,
    Method,
}

impl Scraper for ScraperImpl {
    fn get_dto_tables(&self) -> DtoTables {
        self.tables
            .clone()
            .into_iter()
            .fold(vec![], |mut tables, table| {
                if let Table::Dto(dto_table) = table {
                    tables.push(dto_table)
                }
                tables
            })
    }

    fn get_method_tables(&self) -> MethodTables {
        self.tables
            .clone()
            .into_iter()
            .fold(vec![], |mut tables, table| {
                if let Table::Method(method_table) = table {
                    tables.push(method_table)
                }
                tables
            })
    }
}

#[derive(Debug, PartialEq)]
pub enum ScraperError {
    DocumentError,
    EmptyTextNode,
    TableWithoutHeader,
    InvalidTableColumns { columns: usize },
    InvalidTableRow,
    MissingTableBody { table_name: String },
}

impl std::error::Error for ScraperError {}

impl From<std::io::Error> for ScraperError {
    fn from(_error: std::io::Error) -> Self {
        ScraperError::DocumentError
    }
}

impl std::fmt::Display for ScraperError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScraperError::DocumentError => writeln!(f, "The HTML could not be read!"),
            ScraperError::EmptyTextNode => writeln!(f, "A node did not contain any text!"),
            ScraperError::TableWithoutHeader => writeln!(f, "A table does not have a h4 header! This may indicate a changed html format."),
            ScraperError::InvalidTableColumns { columns } => writeln!(f, "A table has a invalid amount of columns: {}. This may indicate a changed html format.", columns),
            ScraperError::InvalidTableRow => writeln!(f, "A table has an invalid row."),
            ScraperError::MissingTableBody { table_name } => writeln!(f, "The table {} has no body!", table_name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api_parser::scraper::{Scraper, ScraperError, ScraperImpl};
    use crate::api_parser::tables::{DtoRow, DtoTable, DtoTables, MethodRow, MethodTable, MethodTables};

    const TABLE_HTML: &'static str = r#"
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDto</h4>
    <p>A nice Dto.</p>
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
                <td>foos</td>
                <td>Array of <a href=#messageentity>Foo</a></td>
                <td><em>Optional.</em> foo bar baz.</td>
            </tr>
            <tr>
                <td>bar</td>
                <td>Bar</a></td>
                <td>Bar bar bar.</td>
            </tr>
        </tbody>
    </table>
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>BarDto</h4>
    <p>Just a header.</p>
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>BazDto</h4>
    <p>Another nice Dto.</p>
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
                <td>baz</td>
                <td>Baz</a></td>
                <td>Baz baz baz.</td>
            </tr>
        </tbody>
    </table>
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooMethod</h4>
    <p>A nice method.</p>
    <table>
        <thead>
            <tr>
                <th>Parameter</th>
                <th>Type</th>
                <th>Required</th>
                <th>Description</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>foo</td>
                <td>Foo</td>
                <td>Optional</td>
                <td>A foo parameter.</td>
            </tr>
        </tbody>
    </table>
    "#;

    const INVALID_TABLE_COLUMNS: &'static str = r#"
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDto</h4>
    <table>
        <thead>
            <tr>
                <th>Field</th>
                <th>Type and Description</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>foos</td>
                <td>Array of <a href=#messageentity>Foo. Optional.</em> foo bar baz.</a></td>
            </tr>
        </tbody>
    </table>
    "#;

    const INVALID_TABLE_ROW: &'static str = r#"
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDto</h4>
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
                <td>foos</td>
            </tr>
        </tbody>
    </table>
    "#;

    const INVALID_TABLE_MISSING_BODY: &'static str = r#"
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDto</h4>
    <table>
        <thead>
            <tr>
                <th>Field</th>
                <th>Type</th>
                <th>Description</th>
            </tr>
        </thead>
    </table>
    "#;

    const INVALID_HTML_EMPTY_TEXT_NODE: &'static str = r#"
    <h4><a class="anchor" name="update"><i class="anchor-icon"></i></a>FooDto</h4>
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
                <td>foos</td>
                <td></td>
                <td><em>Optional.</em> foo bar baz.</td>
            </tr>
        </tbody>
    </table>
    "#;

    const INVALID_TABLE_NO_HEADER: &'static str = r#"
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
                <td>foos</td>
                <td>Array of <a href=#messageentity>Foo</a></td>
                <td><em>Optional.</em> foo bar baz.</td>
            </tr>
            <tr>
                <td>bar</td>
                <td>Bar</a></td>
                <td>Bar bar bar.</td>
            </tr>
        </tbody>
    </table>
    "#;

    /// The html string used for this test contains
    /// - two Dtos, FooDto and BazDto
    /// - three h4 headers (so one that does not introduce a Dto)
    /// - additional elements between header and table
    /// Some table cells also contain additional elements like <em>.
    #[test]
    fn success_extract_dto_tables() {
        let scraper = ScraperImpl::from_html(TABLE_HTML.as_bytes()).unwrap();

        let dto_tables = scraper.get_dto_tables();
        let method_tables = scraper.get_method_tables();

        assert_eq!(dto_tables, create_expected_dto_tables());
        assert_eq!(method_tables, create_expected_method_tables())
    }

    fn create_expected_dto_tables() -> DtoTables {
        let mut tables = Vec::new();
        tables.push(create_first_dto_table());
        tables.push(create_second_dto_table());
        tables
    }

    fn create_first_dto_table() -> DtoTable {
        let mut table = DtoTable::new(String::from("FooDto"));
        table.add_row(DtoRow::new(String::from("foos"), String::from("Array of Foo"), String::from("Optional. foo bar baz.")));
        table.add_row(DtoRow::new(String::from("bar"), String::from("Bar"), String::from("Bar bar bar.")));
        table
    }

    fn create_second_dto_table() -> DtoTable {
        let mut table = DtoTable::new(String::from("BazDto"));
        table.add_row(DtoRow::new(String::from("baz"), String::from("Baz"), String::from("Baz baz baz.")));
        table
    }

    fn create_expected_method_tables() -> MethodTables {
        let mut tables = Vec::new();
        tables.push(create_method_table());
        tables
    }

    fn create_method_table() -> MethodTable {
        let mut table = MethodTable::new(String::from("FooMethod"));
        table.add_row(MethodRow::new(String::from("foo"), String::from("Foo"), String::from("Optional"), String::from("A foo parameter.")));
        table
    }

    #[test]
    fn failure_invalid_table_columns() {
        let scraper_result = ScraperImpl::from_html(INVALID_TABLE_COLUMNS.as_bytes());

        match scraper_result {
            Err(err) => assert_eq!(err, ScraperError::InvalidTableColumns { columns: 2 }),
            Ok(_) => panic!("Result was not an error!")
        }
    }

    #[test]
    fn failure_invalid_table_row() {
        let scraper_result = ScraperImpl::from_html(INVALID_TABLE_ROW.as_bytes());

        match scraper_result {
            Err(err) => assert_eq!(err, ScraperError::InvalidTableRow),
            Ok(_) => panic!("Result was not an error!")
        }
    }

    #[test]
    fn failure_missing_table_body() {
        let scraper_result = ScraperImpl::from_html(INVALID_TABLE_MISSING_BODY.as_bytes());

        match scraper_result {
            Err(err) => assert_eq!(err, ScraperError::MissingTableBody { table_name: String::from("FooDto") }),
            Ok(_) => panic!("Result was not an error!")
        }
    }

    #[test]
    fn failure_empty_text_node() {
        let scraper_result = ScraperImpl::from_html(INVALID_HTML_EMPTY_TEXT_NODE.as_bytes());

        match scraper_result {
            Err(err) => assert_eq!(err, ScraperError::EmptyTextNode),
            Ok(_) => panic!("Result was not an error!")
        }
    }

    #[test]
    fn failure_invalid_table_missing_header() {
        let scraper_result = ScraperImpl::from_html(INVALID_TABLE_NO_HEADER.as_bytes());

        match scraper_result {
            Err(err) => assert_eq!(err, ScraperError::TableWithoutHeader),
            Ok(_) => panic!("Result was not an error!")
        }
    }
}