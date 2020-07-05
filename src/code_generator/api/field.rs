use serde::Serialize;

use crate::code_generator::names::Names;
use crate::code_generator::renderer::Renderer;
use crate::raw_api::raw_field::RawField;

#[derive(Serialize)]
pub struct Field {
    name: String,
    field_type: String,
}

impl Field {
    pub fn new<R: Renderer>(raw_field: RawField, dto_name: &Names, renderer: &R) -> Self {
        let name = renderer.render_rename(raw_field.name.clone(), dto_name);
        let field_type = renderer.render_type(&raw_field.field_type);

        Field {
            name,
            field_type,
        }
    }
}