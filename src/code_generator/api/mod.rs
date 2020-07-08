use serde::Serialize;
use crate::code_generator::api::dto::Dto;
use crate::code_generator::api::method::Method;
use crate::code_generator::renderer::Renderer;
use crate::raw_api::RawApi;

pub mod dto;
pub mod method;
mod field;
mod parameter;

pub type Dtos = Vec<Dto>;
pub type Methods = Vec<Method>;

#[derive(Serialize)]
pub struct Api {
    pub dtos: Dtos,
    pub methods: Methods
}

impl Api {
    pub  fn new<R: Renderer>(raw_api: RawApi, renderer: &R) -> Result<Self, R::Error> {
        let mut dtos = Vec::new();
        let mut methods = Vec::new();

        for raw_dto in raw_api.raw_dtos {
            dtos.push(Dto::new(raw_dto, renderer)?)
        }

        for raw_method in raw_api.raw_methods {
            methods.push(Method::new(raw_method, renderer)?)
        }

        Ok(Api {
            dtos,
            methods
        })
    }
}