use std::convert::TryFrom;

use handlebars::Handlebars;
use serde::Serialize;

use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::configuration::{Configuration, Rename, TemplateFile};
use crate::code_generator::template::objects::TemplateDto;
use crate::code_generator::template::resolve_strategy::ResolveStrategy;
use crate::code_generator::template::TemplateCodeGenerationError;
use crate::raw_api::{Dtos, RawApi};
use crate::raw_api::field_type::FieldType;

/// Resolves templates provided by the configuration.json.
pub struct TemplateResolver<'a> {
    registry: Handlebars<'a>,
    configuration: Configuration,
}

impl<'a> TemplateResolver<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";

    /// Creates and configures a TemplateResolver with a given Configuration.
    pub fn new(configuration: Configuration) -> Result<Self, TemplateCodeGenerationError> {
        let mut registry = Handlebars::new();

        registry.register_template_string(TemplateResolver::ARRAY_TEMPLATE, &configuration.array_type)?;
        registry.register_template_string(TemplateResolver::OPTIONAL_TEMPLATE, &configuration.optional_type)?;

        for template_file in &configuration.template_files {
            let template_path = &template_file.template_path;

            registry.register_template_string(Self::get_file_name_template_name(&template_path).as_str(), &template_file.target_path)?;
            registry.register_template_file(template_path.as_str(), template_path)?;
        }

        Ok(TemplateResolver {
            registry,
            configuration,
        })
    }

    /// Generates code by iterating over every TemplateFile and applying the choosen ResolveStrategy.
    /// If ForAllDTOs is choosen, the complete list of Dtos is used to resolve the template.
    /// ForEachDTO in comparision resolves and creates a file for each single Dto.
    pub fn resolve(&self, api: RawApi) -> Result<TargetFiles, TemplateCodeGenerationError> {
        let mut result = TargetFiles::new();
        let template_dtos = self.convert_to_template_dtos(api.get_dtos())?;

        for template_file in self.configuration.template_files.iter() {
            let resolve_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match resolve_strategy {
                ResolveStrategy::ForAllDTOs => result.insert_all(self.resolve_with_dtos(template_file, &template_dtos)?)?,
                ResolveStrategy::ForEachDTO => {
                    for dto in template_dtos.iter() {
                        result.insert_all(self.resolve_with_dtos(template_file, dto)?)?
                    }
                }
            };
        }

        Ok(result)
    }

    /// Converts given Dtos to TemplateDtos. These contain more fields to generate the desired code.
    fn convert_to_template_dtos(&self, dtos: &Dtos) -> Result<Vec<TemplateDto>, TemplateCodeGenerationError> {
        let mut result = Vec::new();

        for dto in dtos.into_iter() {
            result.push(TemplateDto::new(dto, &self)?)
        }

        Ok(result)
    }

    /// Creates a TargetFiles object with a single entry, created from a given template that was resolved with the given TemplateDtos.
    /// The given data was either a single TemplateDto or a Vec.
    /// The given TemplateFile is used to load the correct templates.
    fn resolve_with_dtos<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> Result<TargetFiles, TemplateCodeGenerationError> {
        let mut result = TargetFiles::new();

        let filename = self.registry.render(Self::get_file_name_template_name(&template_file.template_path).as_str(), dtos)?;
        let content = self.registry.render(&template_file.template_path, dtos)?;

        result.insert(filename, content)?;
        Ok(result)
    }

    /// Returns the full type of a field as String, converted from its FieldType.
    pub fn get_field_type_string(&self, field_type: &FieldType) -> Result<String, TemplateCodeGenerationError> {
        match field_type {
            FieldType::Integer => Ok(self.configuration.integer_type.clone()),
            FieldType::String => Ok(self.configuration.string_type.clone()),
            FieldType::Boolean => Ok(self.configuration.boolean_type.clone()),
            FieldType::DTO(dto_name) => Ok(dto_name.clone()),
            FieldType::ArrayOf(array_field_type) => self.get_array_value(self.get_field_type_string(array_field_type)?),
            FieldType::Optional(optional_field_type) => self.get_optional_value(self.get_field_type_string(optional_field_type)?)
        }
    }

    pub fn rename(&self, field_name: String) -> String {
        let renames: Vec<&Rename> = self.configuration.renames.iter().filter(|rename| rename.from == field_name).collect();
        let rename_option = renames.first();

        match rename_option {
            Some(rename) => rename.to.clone(),
            None => field_name
        }
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
    fn get_optional_value(&self, value: String) -> Result<String, TemplateCodeGenerationError> {
        Ok(self.registry.render(TemplateResolver::OPTIONAL_TEMPLATE, &SingleValueHolder { value })?)
    }

    /// Returns the given String wrapped in the registered array template.
    fn get_array_value(&self, value: String) -> Result<String, TemplateCodeGenerationError> {
        Ok(self.registry.render(TemplateResolver::ARRAY_TEMPLATE, &SingleValueHolder { value })?)
    }
}

/// Wraps a single String so it can be processed by handlebars.
#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

/// TODO: Test Constructor, resolve_each, resolve_single
#[cfg(test)]
mod tests {
    use crate::code_generator::template::template_resolver::TemplateResolver;
    use crate::raw_api::field_type::FieldType;

    use super::Configuration;

    #[test]
    fn success_get_field_type() {
        let resolver = create_resolver();
        let input_expected = vec![
            (resolver.get_field_type_string(&FieldType::Integer).unwrap(), String::from("u64")),
            (resolver.get_field_type_string(&FieldType::String).unwrap(), String::from("String")),
            (resolver.get_field_type_string(&FieldType::Boolean).unwrap(), String::from("bool")),
            (resolver.get_field_type_string(&FieldType::Optional(Box::new(FieldType::DTO(String::from("Update"))))).unwrap(), String::from("Option<Update>")),
            (resolver.get_field_type_string(&FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))).unwrap(), String::from("Vec<Update>")),
            (resolver.get_field_type_string(&FieldType::Optional(Box::new(FieldType::ArrayOf(Box::new(FieldType::DTO(String::from("Update"))))))).unwrap(), String::from("Option<Vec<Update>>"))
        ];

        input_expected.into_iter().for_each(|(input, expected)| assert_eq!(input, expected));
    }

    #[test]
    fn success_file_name_template_name_created() {
        let file_name = String::from("struct.txt");
        let file_name_template_name = TemplateResolver::get_file_name_template_name(&file_name);

        assert_eq!(file_name_template_name, String::from("struct.txt_name"))
    }

    fn create_resolver() -> TemplateResolver<'static> {
        let template = Configuration {
            integer_type: String::from("u64"),
            string_type: String::from("String"),
            boolean_type: String::from("bool"),
            array_type: String::from("Vec<{{{value}}}>"),
            optional_type: String::from("Option<{{{value}}}>"),
            renames: Vec::new(),
            template_files: Vec::new()
        };

        TemplateResolver::new(template).unwrap()
    }
}