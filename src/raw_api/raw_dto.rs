use crate::raw_api::raw_field::RawField;

/// Holds a DTO with its name and all fields.
#[derive(Eq,PartialEq ,Debug)]
pub struct RawDto {
    pub name: String,
    pub fields: Vec<RawField>
}

impl RawDto {
    pub fn new(name: String, fields: Vec<RawField>) -> Self {
        RawDto {
            name,
            fields
        }
    }
}