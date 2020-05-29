use std::collections::HashSet;

use serde::Serialize;

use crate::code_generator::renderer::Renderer;
use crate::raw_api::raw_dto::RawDto;
use crate::raw_api::raw_field::RawField;
use crate::raw_api::RawApi;
use crate::util::to_snake_case;

pub type ResolvedDtos = Vec<Dto>;

#[derive(Serialize)]
pub struct Api {
    resolved_dtos: ResolvedDtos
}

impl Api {
    pub  fn new<R: Renderer>(api: RawApi, renderer: &R) -> Self {
        let mut template_dtos = Vec::new();

        for dto in api.dtos.iter() {
            template_dtos.push(Dto::new(dto, renderer))
        }

        Api {
            resolved_dtos: template_dtos
        }
    }

    pub fn get_dtos(&self) -> &ResolvedDtos {
        &self.resolved_dtos
    }
}

#[derive(Serialize)]
pub struct Dto {
    name: DtoName,
    fields: Vec<Field>,
    used_dto_names: HashSet<DtoName>,
}

impl Dto {
    pub fn new<R: Renderer>(dto: &RawDto, renderer: &R) -> Self {
        let name = DtoName::new(&dto.name);
        let mut fields = Vec::new();
        let mut used_dto_names = HashSet::new();

        for field in dto.fields.iter() {
            fields.push(Field::new(field, &name, renderer));

            if let Some(dto_name) = field.field_type.get_dto_name() {
                if name.original() != &dto_name {
                    used_dto_names.insert(DtoName::new(&dto_name));
                }
            }
        }

        Dto {
            name,
            fields,
            used_dto_names,
        }
    }
}

/// The name of a Dto and all possible variants
#[derive(Serialize, Eq, PartialEq, Hash)]
pub struct DtoName {
    original: String,
    snake_case: String,
}

impl DtoName {
    pub fn new(dto_name: &String) -> Self {
        let dto_name = dto_name.clone();
        let name_snake_case = to_snake_case(&dto_name);

        DtoName {
            original: dto_name,
            snake_case: name_snake_case
        }
    }

    pub fn original(&self) -> &String {
        &self.original
    }
}

/// Holds all values to resolve a DTOField in a template.
/// It contains:
//      - the name of the field
//      - the complete field-type as String (for example "Option<Update>")
//      - the name of the DTO type, if the type of this field is or wraps (Array, Optional) a DTO. Defaults to empty String (so it can be filtered by handlebars).
//      - the name of the DTO type in snake_case, if the DTO name is set. Defaults to empty String (so it can be filtered by handlebars).
#[derive(Serialize)]
pub struct Field {
    name: String,
    field_type: String,
}

impl Field {
    pub fn new<R: Renderer>(dto_field: &RawField, dto_name: &DtoName, renderer: &R) -> Self {
        let name = renderer.render_rename(dto_field.name.clone(), dto_name);
        let field_type = renderer.render_field_type(&dto_field.field_type);

        Field {
            name,
            field_type,
        }
    }
}