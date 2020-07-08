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
    pub fn new<R: Renderer>(raw_field: RawField, dto_name: &Names, renderer: &R) -> Result<Self, R::Error> {
        let name = renderer.render_rename(raw_field.name.clone(), dto_name).unwrap();
        let field_type = renderer.render_type(&raw_field.field_type).unwrap();

        Ok(Field {
            name,
            field_type,
        })
    }
}