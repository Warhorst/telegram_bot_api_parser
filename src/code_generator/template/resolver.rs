use std::fmt::Formatter;

use handlebars::{Handlebars, RenderError, TemplateError, TemplateFileError};
use serde::Serialize;

use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::api::{ResolvedDto, ResolvedDtoName, ResolvedDtos};
use crate::code_generator::template::configuration::{Configuration, TemplateFile};
use crate::raw_api::field_type::FieldType;

/// Trait for all objects that can resolve templates.
pub trait Resolver {
    type Error: std::error::Error + std::fmt::Debug;

    fn resolve_for_single_dto(&self, template_file: &TemplateFile, dto: &ResolvedDto) -> Result<TargetFiles, Self::Error>;

    fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &ResolvedDtos) -> Result<TargetFiles, Self::Error>;

    fn resolve_field_type(&self, field_type: &FieldType) -> Result<String, Self::Error>;

    fn resolve_field_rename(&self, field_name: String, field_rename_values: &ResolvedDtoName) -> Result<String, Self::Error>;
}

/// Template resolver that uses the handlebars templating engine.
pub struct HandlebarsResolver<'a> {
    registry: Handlebars<'a>,
    configuration: Configuration
}

impl<'a> Resolver for HandlebarsResolver<'a> {
    type Error = HandlebarsError;

    fn resolve_for_single_dto(&self, template_file: &TemplateFile, dto: &ResolvedDto) -> Result<TargetFiles, Self::Error> {
        self.resolve_template_file(template_file, dto)
    }

    fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &ResolvedDtos) -> Result<TargetFiles, Self::Error> {
        self.resolve_template_file(template_file, dtos)
    }

    fn resolve_field_type(&self, field_type: &FieldType) -> Result<String, Self::Error> {
        match field_type {
            FieldType::Integer => Ok(self.configuration.integer_type.clone()),
            FieldType::String => Ok(self.configuration.string_type.clone()),
            FieldType::Boolean => Ok(self.configuration.boolean_type.clone()),
            FieldType::DTO(dto_name) => Ok(dto_name.clone()),
            FieldType::ArrayOf(array_field_type) => self.get_array_value(self.resolve_field_type(array_field_type)?),
            FieldType::Optional(optional_field_type) => self.get_optional_value(self.resolve_field_type(optional_field_type)?)
        }
    }

    fn resolve_field_rename(&self, field_name: String, field_rename_values: &ResolvedDtoName) -> Result<String, Self::Error> {
        Ok(String::from("foo"))
    }
}

impl<'a> HandlebarsResolver<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";

    pub fn new(configuration: Configuration) -> Result<Self, HandlebarsError> {
        let mut registry = Handlebars::new();

        registry.register_template_string(Self::ARRAY_TEMPLATE, &configuration.array_type)?;
        registry.register_template_string(Self::OPTIONAL_TEMPLATE, &configuration.optional_type)?;

        for template_file in &configuration.template_files {
            let template_path = &template_file.template_path;

            registry.register_template_string(Self::get_file_name_template_name(&template_path).as_str(), &template_file.target_path)?;
            registry.register_template_file(template_path.as_str(), template_path)?;
        }

        Ok(HandlebarsResolver {
            registry,
            configuration,
        })
    }

    /// Creates a TargetFiles object with a single entry, created from a given template that was resolved with the given TemplateDtos.
    /// The given data was either a single TemplateDto or a Vec.
    /// The given TemplateFile is used to load the correct templates.
    fn resolve_template_file<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> Result<TargetFiles, HandlebarsError> {
        let mut result = TargetFiles::new();

        let filename = self.registry.render(Self::get_file_name_template_name(&template_file.template_path).as_str(), dtos)?;
        let content = self.registry.render(&template_file.template_path, dtos)?;

        // Todo add a single value, so no checks are necessary at this spot.
        result.insert(filename, content).unwrap();
        Ok(result)
    }

    /// Returns the name of the template that will be used to resolve the filename of a target file.
    ///
    /// For example: given is a TemplateFile with template_path "struct.txt" and target_path "{{dto.name}}.rs".
    /// The content (template inside) of the file "struct.txt" will be registerd under the name "struct.txt".
    /// The filename template will be registered under the name "struct.txt_name".
    fn get_file_name_template_name(file_name: &String) -> String {
        let mut result = String::from(file_name.as_str());
        result.push_str(Self::FILE_NAME_TEMPLATE_NAME_POSTFIX);
        result
    }

    /// Returns the given String wrapped in the registered optional template.
    fn get_optional_value(&self, value: String) -> Result<String, HandlebarsError> {
        Ok(self.registry.render(Self::OPTIONAL_TEMPLATE, &SingleValueHolder { value })?)
    }

    /// Returns the given String wrapped in the registered array template.
    fn get_array_value(&self, value: String) -> Result<String, HandlebarsError> {
        Ok(self.registry.render(Self::ARRAY_TEMPLATE, &SingleValueHolder { value })?)
    }
}

/// Wraps a single String so it can be processed by handlebars.
#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

#[derive(Debug)]
pub enum HandlebarsError {
    TemplateError(TemplateError),
    TemplateFileError(TemplateFileError),
    RenderError(RenderError)
}

impl std::error::Error for HandlebarsError {}

impl std::fmt::Display for HandlebarsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlebarsError::TemplateError(error) => error.fmt(f),
            HandlebarsError::TemplateFileError(error) => std::fmt::Display::fmt(&error, f),
            HandlebarsError::RenderError(error) => error.fmt(f)
        }
    }
}

impl From<TemplateError> for HandlebarsError {
    fn from(e: TemplateError) -> Self {
        HandlebarsError::TemplateError(e)
    }
}

impl From<TemplateFileError> for HandlebarsError {
    fn from(e: TemplateFileError) -> Self {
        HandlebarsError::TemplateFileError(e)
    }
}

impl From<RenderError> for HandlebarsError {
    fn from(e: RenderError) -> Self {
        HandlebarsError::RenderError(e)
    }
}

/// TODO: Test Constructor, resolve_each, resolve_single
#[cfg(test)]
mod tests {
    use crate::code_generator::template::resolver::{HandlebarsResolver, Resolver};
    use crate::code_generator::template::TemplateCodeGenerator;
    use crate::raw_api::field_type::FieldType;

    use super::Configuration;

    #[test]
    fn success_get_field_type() {
        let resolver = create_resolver();
        let input_expected = vec![
            (resolver.resolve_field_type(&FieldType::Integer).unwrap(), String::from("u64")),
            (resolver.resolve_field_type(&FieldType::String).unwrap(), String::from("String")),
            (resolver.resolve_field_type(&FieldType::Boolean).unwrap(), String::from("bool")),
            (resolver.resolve_field_type(&FieldType::Optional(Box::new(FieldType::DTO(String::from("Update"))))).unwrap(), String::from("Option<Update>")),
            (resolver.resolve_field_type(&FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))).unwrap(), String::from("Vec<Update>")),
            (resolver.resolve_field_type(&FieldType::Optional(Box::new(FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))))).unwrap(), String::from("Option<Vec<Update>>"))
        ];

        input_expected.into_iter().for_each(|(input, expected)| assert_eq!(input, expected));
    }

    #[test]
    fn success_file_name_template_name_created() {
        let file_name = String::from("struct.txt");
        let file_name_template_name = HandlebarsResolver::get_file_name_template_name(&file_name);

        assert_eq!(file_name_template_name, String::from("struct.txt_name"))
    }

    fn create_resolver() -> HandlebarsResolver<'static> {
        let configuration = Configuration {
            integer_type: String::from("u64"),
            string_type: String::from("String"),
            boolean_type: String::from("bool"),
            array_type: String::from("Vec<{{{value}}}>"),
            optional_type: String::from("Option<{{{value}}}>"),
            renames: Vec::new(),
            template_files: Vec::new()
        };

        HandlebarsResolver::new(configuration).unwrap()
    }
}