use serde::Serialize;

use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::code_generator::template::TemplateCodeGenerationError;
use crate::raw_api::dto::Dto;
use crate::raw_api::dto_field::DtoField;
use crate::util::to_snake_case;

#[derive(Serialize)]
pub struct TemplateDto {
    name: String,
    name_snake_case: String,
    fields: Vec<TemplateDtoField>,
}

impl TemplateDto {
    pub fn new(dto: &Dto, template_resolver: &TemplateResolver) -> Result<Self, TemplateCodeGenerationError> {
        let name = dto.get_name().clone();
        let name_snake_case = to_snake_case(&name);
        let mut fields = Vec::new();

        for field in dto.get_fields().iter() {
            fields.push(TemplateDtoField::new(field, template_resolver)?)
        }

        Ok(TemplateDto {
            name,
            name_snake_case,
            fields,
        })
    }
}

/// Holds all values to resolve a DTOField in a template.
/// It contains:
//      - the name of the field
//      - the complete field-type as String (for example "Option<Update>")
//      - the name of the DTO type, if the type of this field is or wraps (Array, Optional) a DTO. Defaults to empty String (so it can be filtered by handlebars).
//      - the name of the DTO type in snake_case, if the DTO name is set. Defaults to empty String (so it can be filtered by handlebars).
#[derive(Serialize)]
pub struct TemplateDtoField {
    name: String,
    field_type: String,
    dto_type: String,
    dto_type_snake_case: String,
}

impl TemplateDtoField {
    pub fn new(dto_field: &DtoField, template_resovler: &TemplateResolver) -> Result<Self, TemplateCodeGenerationError> {
        let name = dto_field.get_name().clone();
        let field_type = dto_field.get_field_type();
        let field_type_string = template_resovler.get_field_type_string(field_type)?;
        let dto_name = field_type.get_dto_name();

        let (dto_type_snake_case, dto_type) = match dto_name {
            Some(value) => (to_snake_case(&value), value),
            None => (String::new(), String::new())
        };

        Ok(TemplateDtoField {
            name,
            field_type: field_type_string,
            dto_type,
            dto_type_snake_case,
        })
    }
}