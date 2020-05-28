use crate::raw_api::field_type::FieldType;

/// Struct of a DTO with name and type
#[derive(Eq, PartialEq, Debug)]
pub struct RawField {
    pub name: String,
    pub field_type: FieldType,
}

impl RawField {
    pub fn new(name: String, type_value: String, optional: bool) -> Self {
        RawField {
            name,
            field_type: FieldType::from(FieldDescription { value: type_value, optional })
        }
    }
}

/// Struct of a DTO-field-type with its description from the api (type name and if it is optional).
#[derive(Clone)]
pub struct FieldDescription {
    pub value: String,
    pub optional: bool,
}

impl FieldDescription {
    pub fn new(value: String, optional: bool) -> Self {
        FieldDescription {
            value,
            optional
        }
    }
}