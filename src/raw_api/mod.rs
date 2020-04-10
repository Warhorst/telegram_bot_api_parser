pub mod dto;
pub mod dto_field;
pub mod field_type;

use crate::raw_api::dto::Dto;

/// Represents a collection of all extracted values from the HTML-API
#[derive(Debug)]
pub struct RawApi {
    dtos: Vec<Dto>
}

impl RawApi {
    pub fn new() -> Self {
        RawApi {
            dtos: Vec::new()
        }
    }

    /// Adda DTO to the list of DTOs
    pub fn add_dto(&mut self, dto: Dto) {
        self.dtos.push(dto)
    }

    pub fn get_dtos(&self) -> &Vec<Dto> {
        &self.dtos
    }
}