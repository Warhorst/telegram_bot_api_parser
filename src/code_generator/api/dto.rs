use std::collections::HashSet;

use serde::Serialize;

use crate::code_generator::api::field::Field;
use crate::code_generator::names::Names;
use crate::code_generator::renderer::Renderer;
use crate::raw_api::raw_dto::RawDto;

#[derive(Serialize)]
pub struct Dto {
    name: Names,
    fields: Vec<Field>,
    used_dto_names: HashSet<Names>,
}

impl Dto {
    pub fn new<R: Renderer>(raw_dto: RawDto, renderer: &R) -> Self {
        let name = Names::new(&raw_dto.name);
        let mut fields = Vec::new();
        let mut used_dto_names = HashSet::new();

        for raw_field in raw_dto.fields {
            if let Some(dto_name) = raw_field.field_type.get_dto_name() {
                if name.camel_case != dto_name {
                    used_dto_names.insert(Names::new(&dto_name));
                }
            }
            fields.push(Field::new(raw_field, &name, renderer));
        }

        Dto {
            name,
            fields,
            used_dto_names,
        }
    }
}