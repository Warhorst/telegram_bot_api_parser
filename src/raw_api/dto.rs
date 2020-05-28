use crate::raw_api::dto_field::DtoField;

/// Holds a DTO with its name and all fields.
#[derive(Eq,PartialEq ,Debug)]
pub struct Dto {
    pub name: String,
    pub fields: Vec<DtoField>
}

impl Dto {
    pub fn new(name: String, fields: Vec<DtoField>) -> Self {
        Dto {
            name,
            fields
        }
    }
}