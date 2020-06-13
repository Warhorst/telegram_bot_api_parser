#[derive(Clone, Debug, PartialEq)]
pub enum Table {
    Dto(DtoTable),
    Method(MethodTable),
}

pub type DtoTables = Vec<DtoTable>;
pub type MethodTables = Vec<MethodTable>;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
            description_string,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodTable {
    pub name: String,
    pub rows: Vec<MethodRow>,
}

impl MethodTable {
    pub fn new(method_name: String) -> Self {
        MethodTable {
            name: method_name,
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: MethodRow) {
        self.rows.push(row)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodRow {
    pub parameter_string: String,
    pub type_string: String,
    pub required_string: String,
    pub description_string: String,
}

impl MethodRow {
    pub fn new(parameter_string: String, type_string: String, required_string: String, description_string: String) -> Self {
        MethodRow {
            parameter_string,
            type_string,
            required_string,
            description_string,
        }
    }
}