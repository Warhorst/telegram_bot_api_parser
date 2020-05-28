use crate::raw_api::raw_dto::RawDto;

pub mod raw_dto;
pub mod raw_field;
pub mod field_type;

pub type RawDtos = Vec<RawDto>;

/// Represents a collection of all extracted values from the HTML-API
#[derive(Debug)]
pub struct RawApi {
    pub dtos: RawDtos
}

impl RawApi {
    pub fn new() -> Self {
        RawApi {
            dtos: Vec::new()
        }
    }
}