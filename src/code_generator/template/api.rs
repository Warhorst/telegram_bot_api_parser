use std::collections::HashSet;

use serde::Serialize;

use crate::code_generator::template::resolver::Resolver;
use crate::raw_api::dto::Dto;
use crate::raw_api::dto_field::DtoField;
use crate::raw_api::RawApi;
use crate::util::to_snake_case;

pub type ResolvedDtos = Vec<ResolvedDto>;

#[derive(Serialize)]
pub struct ResolvedApi {
    resolved_dtos: ResolvedDtos
}

impl ResolvedApi {
    pub  fn new<R: Resolver>(api: RawApi, resolver: &R) -> Self {
        let mut template_dtos = Vec::new();

        for dto in api.get_dtos().iter() {
            template_dtos.push(ResolvedDto::new(dto, resolver))
        }

        ResolvedApi {
            resolved_dtos: template_dtos
        }
    }

    pub fn get_dtos(&self) -> &ResolvedDtos {
        &self.resolved_dtos
    }
}

#[derive(Serialize)]
pub struct ResolvedDto {
    name: ResolvedDtoName,
    fields: Vec<ResolvedDtoField>,
    used_dto_names: HashSet<ResolvedDtoName>,
}

impl ResolvedDto {
    pub fn new<R: Resolver>(dto: &Dto, resolver: &R) -> Self {
        let name = ResolvedDtoName::new(&dto.get_name());
        let mut fields = Vec::new();
        let mut used_dto_names = HashSet::new();

        for field in dto.get_fields().iter() {
            fields.push(ResolvedDtoField::new(field, &name, resolver));

            if let Some(dto_name) = field.get_field_type().get_dto_name() {
                if name.original() != &dto_name {
                    used_dto_names.insert(ResolvedDtoName::new(&dto_name));
                }
            }
        }

        ResolvedDto {
            name,
            fields,
            used_dto_names,
        }
    }
}

/// The name of a Dto and all possible variants
#[derive(Serialize, Eq, PartialEq, Hash)]
pub struct ResolvedDtoName {
    original: String,
    snake_case: String,
}

impl ResolvedDtoName {
    pub fn new(dto_name: &String) -> Self {
        let dto_name = dto_name.clone();
        let name_snake_case = to_snake_case(&dto_name);

        ResolvedDtoName {
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
pub struct ResolvedDtoField {
    name: String,
    field_type: String,
}

impl ResolvedDtoField {
    pub fn new<R: Resolver>(dto_field: &DtoField, dto_name: &ResolvedDtoName, resolver: &R) -> Self {
        let name = resolver.resolve_field_rename(dto_field.get_name().clone(), dto_name);
        let field_type = resolver.resolve_field_type(dto_field.get_field_type());

        ResolvedDtoField {
            name,
            field_type,
        }
    }
}