use serde::Serialize;

use crate::code_generator::names::Names;
use crate::code_generator::renderer::Renderer;
use crate::raw_api::raw_parameter::RawParameter;

#[derive(Serialize)]
pub struct Parameter {
    name: String,
    parameter_type: String
}

impl Parameter {
    pub fn new<R: Renderer>(raw_parameter: RawParameter, renderer: &R) -> Result<Self, R::Error> {
        let name = match raw_parameter.parameter_type.get_dto_name() {
            Some(dto_name) => {
                let dto_name = Names::new(&dto_name);
                renderer.render_rename(raw_parameter.name, &dto_name)?
            },
            None => raw_parameter.name
        };
        let parameter_type = renderer.render_type(&raw_parameter.parameter_type).unwrap();

        Ok(Parameter {
            name,
            parameter_type
        })
    }
}