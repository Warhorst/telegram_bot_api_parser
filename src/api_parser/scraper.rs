use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate, Text};

use crate::api_parser::ApiParser;
use crate::api_parser::tables::{DtoRow, DtoTable, DtoTables};
use crate::raw_api::RawApi;

pub trait Scraper {
    fn get_dto_tables(&self) -> DtoTables;
}

pub struct ScraperImpl {
    dto_tables: DtoTables
}

impl ScraperImpl {
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

    pub fn new<R: std::io::Read>(api_html: R) -> Self {
        let mut dto_tables = Vec::new();
        let document = Document::from_read(api_html).unwrap();
        let mut current_table_name = None;

        for node in document.find(Self::searched_nodes_predicate()) {
            match node.name().unwrap() {
                Self::H4 => current_table_name = Self::get_node_text(&node),

                Self::TABLE => match (&current_table_name, Self::get_table_content_type(&node)) {
                    (Some(table_name), TableContentType::DTO) => {
                        dto_tables.push(Self::extract_dto_table(table_name.clone(), &node));
                        current_table_name = None
                    }

                    (Some(_), TableContentType::Method) => (),

                    (None, _) => ()
                },
                _ => ()
            }
        }

        ScraperImpl {
            dto_tables
        }
    }

    fn searched_nodes_predicate() -> impl Predicate {
        Name(ApiParser::H4).or(Name(ApiParser::TABLE))
    }

    fn get_node_text(node: &Node) -> Option<String> {
        let text_nodes: Vec<Node> = node.find(Text).collect();
        let mut text_node_iter = text_nodes.iter();
        let mut node_text = String::new();

        while let Some(text_node) = text_node_iter.next() {
            node_text.push_str(text_node.text().as_str())
        }

        match node_text.is_empty() {
            true => None,
            false => Some(node_text)
        }
    }

    fn extract_dto_table(dto_name: String, table_node: &Node) -> DtoTable {
        let mut dto_table = DtoTable::new(dto_name);
        let table_bodies = table_node.find(Name(Self::TABLE_BODY)).collect::<Vec<Node>>();

        match table_bodies.get(0) {
            Some(table_body) => {
                for table_row in table_body.find(Name(Self::TABLE_ROW)) {
                    dto_table.add_row(Self::extract_dto_row(&table_row).unwrap());
                }
            }
            None => panic!()
        }

        dto_table
    }

    fn extract_dto_row(table_row: &Node) -> Option<DtoRow> {
        let data_nodes: Vec<Node> = table_row.find(Name(Self::TABLE_DATA)).collect();
        let field_node_option = data_nodes.get(0);
        let type_node_option = data_nodes.get(1);
        let description_node_option = data_nodes.get(2);

        match (field_node_option, type_node_option, description_node_option) {
            (Some(field_node), Some(type_node), Some(description_node)) => {
                let field_string = Self::get_node_text(field_node).unwrap();
                let type_string = Self::get_node_text(type_node).unwrap();
                let description_string = Self::get_node_text(description_node).unwrap();
                Some(DtoRow::new(field_string, type_string, description_string))
            }
            _ => None
        }
    }

    fn get_table_content_type(table_node: &Node) -> TableContentType {
        match table_node.find(Name(Self::TABLE_HEADER)).count() {
            Self::DTO_TABLE_COLUMNS => TableContentType::DTO,
            Self::METHOD_TABLE_COLUMNS => TableContentType::Method,
            columns => panic!()
        }
    }
}

enum TableContentType {
    DTO,
    Method,
}

impl Scraper for ScraperImpl {
    fn get_dto_tables(&self) -> DtoTables {
        self.dto_tables.clone()
    }
}

#[cfg(test)]
mod tests {
    use select::document::Document;
    use select::node::Node;
    use select::predicate::{Any, Name};

    use crate::api_parser::scraper::{Scraper, ScraperImpl};

    const TABLE_HTML: &'static str = r#"
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
                    <td>foos</td>
                    <td>Array of <a href=#messageentity>Foo</a></td>
                    <td><em>Optional.</em> foo bar baz.</td>
                </tr>
            </tbody>
        </table>
    "#;

    #[test]
    fn success_extract_table() {
        let s = ScraperImpl::new(TABLE_HTML.as_bytes());
        println!("{:?}", s.get_dto_tables())
    }
}