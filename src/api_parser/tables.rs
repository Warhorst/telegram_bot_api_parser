pub type DtoTables = Vec<DtoTable>;

#[derive(Clone, Debug)]
pub struct DtoTable {
    pub name: String,
    pub rows: Vec<DtoRow>,
}

impl DtoTable {
    pub fn new(dto_name: String) -> Self {
        DtoTable {
            name: dto_name,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: DtoRow) {
        self.rows.push(row)
    }
}

#[derive(Clone, Debug)]
pub struct DtoRow {
    pub field_string: String,
    pub type_string: String,
    pub description_string: String,
}

impl DtoRow {
    pub fn new(field_string: String, type_string: String, description_string: String) -> Self {
        DtoRow {
            field_string,
            type_string,
            description_string
        }
    }
}