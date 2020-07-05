use serde::Serialize;

use crate::code_generator::api::parameter::Parameter;
use crate::code_generator::renderer::Renderer;
use crate::raw_api::raw_method::RawMethod;
use std::collections::HashSet;
use crate::code_generator::names::Names;

#[derive(Serialize)]
pub struct Method {
    name: Names,
    parameters: Vec<Parameter>,
    used_dto_names: HashSet<Names>
}

impl Method {
    pub fn new<R: Renderer>(raw_method: RawMethod, renderer: &R) -> Self {
        let name = Names::new(&raw_method.name);
        let mut parameters = Vec::new();
        let mut used_dto_names = HashSet::new();

        for raw_parameter in raw_method.parameters {
            if let Some(dto_name) = raw_parameter.parameter_type.get_dto_name() {
                used_dto_names.insert(Names::new(&dto_name));
            }

            parameters.push(Parameter::new(raw_parameter, renderer))
        }

        Method {
            name,
            parameters,
            used_dto_names
        }
    }
}