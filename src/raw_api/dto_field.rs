use crate::raw_api::field_type::FieldType;

/// Struct of a DTO with name and type
pub struct DTOField {
    name: String,
    dto_field_type: DTOFieldType,
}

impl DTOField {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_dto_field_type(&self) -> &DTOFieldType {
        &self.dto_field_type
    }
}

/// Struct of a DTO-field-type with its description from the api
/// and if it is optional.
#[derive(Clone)]
pub struct DTOFieldType {
    description: String,
    optional: bool,
}

impl DTOFieldType {
    pub fn new(description: String, optional: bool) -> Self {
        DTOFieldType {
            description,
            optional
        }
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn is_optional(&self) -> &bool {
        &self.optional
    }
}