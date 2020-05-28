use crate::raw_api::dto::Dto;

pub mod dto;
pub mod dto_field;
pub mod field_type;

pub type Dtos = Vec<Dto>;

/// Represents a collection of all extracted values from the HTML-API
#[derive(Debug)]
pub struct RawApi {
    pub dtos: Dtos
}

impl RawApi {
    pub fn new() -> Self {
        RawApi {
            dtos: Vec::new()
        }
    }
}