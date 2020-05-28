use serde::Serialize;

use crate::code_generator::api::{Dto, DtoName, ResolvedDtos};
use crate::code_generator::configuration::{Configuration, TemplateFile};
use crate::code_generator::renderer::Renderer;
use crate::code_generator::target_files::TargetFiles;
use crate::raw_api::field_type::FieldType;

/// Trait for all objects that can resolve templates.
pub trait Resolver {
    fn resolve_for_single_dto(&self, template_file: &TemplateFile, dto: &Dto) -> TargetFiles;

    fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &ResolvedDtos) -> TargetFiles;

    fn resolve_field_type(&self, field_type: &FieldType) -> String;

    fn resolve_field_rename(&self, field_name: String, field_rename_values: &DtoName) -> String;
}


pub struct ResolverImpl<E: Renderer> {
    renderer: E,
    integer_type: String,
    string_type: String,
    boolean_type: String
}

impl<E: Renderer> Resolver for ResolverImpl<E> {
    fn resolve_for_single_dto(&self, template_file: &TemplateFile, dto: &Dto) -> TargetFiles {
        self.resolve_template_file(template_file, dto)
    }

    fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &ResolvedDtos) -> TargetFiles {
        self.resolve_template_file(template_file, dtos)
    }

    fn resolve_field_type(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::Integer => self.integer_type.clone(),
            FieldType::String => self.string_type.clone(),
            FieldType::Boolean => self.boolean_type.clone(),
            FieldType::DTO(dto_name) => dto_name.clone(),
            FieldType::ArrayOf(array_field_type) => self.renderer.render_array_string(self.resolve_field_type(array_field_type)),
            FieldType::Optional(optional_field_type) => self.renderer.render_optional_string(self.resolve_field_type(optional_field_type))
        }
    }

    fn resolve_field_rename(&self, field_name: String, _field_rename_values: &DtoName) -> String {
        field_name
    }
}

impl<E: Renderer> ResolverImpl<E> {
    pub fn new(configuration: Configuration) -> Self {
        let mut renderer = E::default();

        renderer.register_array_template(configuration.array_type);
        renderer.register_optional_template(configuration.optional_type);

        for template_file in &configuration.template_files {
            renderer.register_template_file(template_file)
        }

        ResolverImpl {
            renderer,
            integer_type: configuration.integer_type,
            string_type: configuration.string_type,
            boolean_type: configuration.boolean_type
        }
    }

    /// Creates a TargetFiles object with a single entry, created from a given template that was resolved with the given TemplateDtos.
    /// The given data was either a single TemplateDto or a Vec.
    /// The given TemplateFile is used to load the correct templates.
    fn resolve_template_file<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> TargetFiles {
        let mut result = TargetFiles::new();
        let target_file = self.renderer.render_template_file_dtos(template_file, dtos);
        result.insert(target_file).unwrap();
        result
    }
}

/// TODO: Test Constructor, resolve_each, resolve_single
#[cfg(test)]
mod tests {
    use crate::code_generator::renderer::RendererImpl;
    use crate::code_generator::resolver::{Resolver, ResolverImpl};
    use crate::raw_api::field_type::FieldType;

    use super::Configuration;

    #[test]
    fn success_get_field_type() {
        let resolver = create_resolver();
        let input_expected = vec![
            (resolver.resolve_field_type(&FieldType::Integer), String::from("u64")),
            (resolver.resolve_field_type(&FieldType::String), String::from("String")),
            (resolver.resolve_field_type(&FieldType::Boolean), String::from("bool")),
            (resolver.resolve_field_type(&FieldType::Optional(Box::new(FieldType::DTO(String::from("Update"))))), String::from("Option<Update>")),
            (resolver.resolve_field_type(&FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))), String::from("Vec<Update>")),
            (resolver.resolve_field_type(&FieldType::Optional(Box::new(FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))))), String::from("Option<Vec<Update>>"))
        ];

        input_expected.into_iter().for_each(|(input, expected)| assert_eq!(input, expected));
    }

    fn create_resolver() -> ResolverImpl<RendererImpl<'static>> {
        let configuration = Configuration {
            integer_type: String::from("u64"),
            string_type: String::from("String"),
            boolean_type: String::from("bool"),
            array_type: String::from("Vec<{{{value}}}>"),
            optional_type: String::from("Option<{{{value}}}>"),
            renames: Vec::new(),
            template_files: Vec::new()
        };

        ResolverImpl::new(configuration)
    }
}