use crate::raw_api::field_type::FieldType;

/// Field of a DTO with name and type
pub struct DTOField {
    name: String,
    field_type: FieldType,
    optional: bool
}