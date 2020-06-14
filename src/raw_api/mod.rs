use crate::raw_api::raw_dto::RawDto;
use crate::raw_api::raw_method::RawMethod;

pub mod raw_dto;
pub mod raw_field;
pub mod type_descriptor;
pub mod raw_method;
pub mod raw_parameter;

pub type RawDtos = Vec<RawDto>;
pub type RawMethods = Vec<RawMethod>;

/// Represents a collection of all extracted values from the HTML-API
#[derive(Debug)]
pub struct RawApi {
    pub raw_dtos: RawDtos,
    pub raw_methods: RawMethods
}