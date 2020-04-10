use crate::raw_api::dto_field::DtoField;

/// Holds a DTO with its name and all fields.
#[derive(Eq,PartialEq ,Debug)]
pub struct Dto {
    name: String,
    fields: Vec<DtoField>
}

impl Dto {
    pub fn new(name: String, fields: Vec<DtoField>) -> Self {
        Dto {
            name,
            fields
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_fields(&self) -> &Vec<DtoField> {
        &self.fields
    }
}