use handlebars::Handlebars;
use serde::Serialize;

use crate::code_generator::target_files_map::TargetFilesMap;
use crate::code_generator::template::objects::template_bot_dto::TemplateBotDTO;
use crate::code_generator::template::template_code_generation_error::TemplateCodeGenerationError;
use crate::code_generator::template::template_file::TemplateFile;
use crate::code_generator::template::validation::validator::Validator;
use crate::raw_api::bot_dto::BotDTO;
use crate::raw_api::field_type::FieldType;

mockuse!(crate::code_generator::template::template, Template, MockTemplate);

/// Resolves templates provided by the templates.json.
pub struct TemplateResolver<'a> {
    integer_type: String,
    string_type: String,
    boolean_type: String,
    registry: Handlebars<'a>,
}

impl<'a> TemplateResolver<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";

    /// Creates a new template-resolver from a given template.
    /// Types of integer, string and boolean are not expected to contain any templates.
    /// Anything else is registered in a handlebars template registry.
    pub fn new(template: &Template) -> Result<Self, TemplateCodeGenerationError> {
        Validator::new().validate_template(template)?;

        let mut registry = Handlebars::new();

        registry.register_template_string(TemplateResolver::ARRAY_TEMPLATE, template.get_array_type())?;
        registry.register_template_string(TemplateResolver::OPTIONAL_TEMPLATE, template.get_optional_type())?;

        for template_file in template.get_template_files() {
            let template_path = template_file.get_template_path();

            registry.register_template_string(TemplateResolver::get_file_name_template_name(&template_path).as_str(), template_file.get_target_path())?;
            registry.register_template_file(template_path.as_str(), template_path)?;
        }

        Ok(TemplateResolver {
            integer_type: template.get_integer_type().to_owned(),
            string_type: template.get_string_type().to_owned(),
            boolean_type: template.get_boolean_type().to_owned(),
            registry,
        })
    }

    /// Resolves the template of the given file with a Vec of all DTOs.
    pub fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &Vec<BotDTO>) -> Result<TargetFilesMap, TemplateCodeGenerationError> {
        let mut result = TargetFilesMap::new();
        let mut template_dtos = Vec::new();

        for dto in dtos.iter() {
            template_dtos.push(TemplateBotDTO::new(dto, &self)?)
        }

        let filename = self.registry.render(TemplateResolver::get_file_name_template_name(template_file.get_template_path()).as_str(), &template_dtos)?;
        let content = self.registry.render(template_file.get_template_path(), &template_dtos)?;

        result.insert(filename, content)?;
        Ok(result)
    }

    /// Resolves the template of the given file with a single DTO.
    pub fn resolve_for_single_dto(&self, template_file: &TemplateFile, dto: &BotDTO) -> Result<TargetFilesMap, TemplateCodeGenerationError> {
        let mut result = TargetFilesMap::new();
        let template_dto = TemplateBotDTO::new(dto, self)?;

        let filename = self.registry.render(TemplateResolver::get_file_name_template_name(template_file.get_template_path()).as_str(), &template_dto)?;
        let content = self.registry.render(template_file.get_template_path(), &template_dto)?;

        result.insert(filename, content)?;
        Ok(result)
    }

    /// Returns the full type of a field as String, converted from its FieldType.
    pub fn get_field_type_string(&self, field_type: &FieldType) -> Result<String, TemplateCodeGenerationError> {
        match field_type {
            FieldType::Integer => Ok(self.integer_type.clone()),
            FieldType::String => Ok(self.string_type.clone()),
            FieldType::Boolean => Ok(self.boolean_type.clone()),
            FieldType::DTO(dto_name) => Ok(dto_name.clone()),
            FieldType::ArrayOf(array_field_type) => self.get_array_value(self.get_field_type_string(array_field_type)?),
            FieldType::Optional(optional_field_type) => self.get_optional_value(self.get_field_type_string(optional_field_type)?)
        }
    }

    /// Returns the name of the template that will be used to resolve the filename of a target file.
    ///
    /// For example: given is a TemplateFile with template_path "struct.txt" and target_path "{{dto.name}}.rs".
    /// The content (template inside) of the file "struct.txt" will be registerd under the name "struct.txt".
    /// The filename template will be registered under the name "struct.txt_name".
    fn get_file_name_template_name(file_name: &String) -> String {
        let mut result = String::from(file_name.as_str());
        result.push_str(TemplateResolver::FILE_NAME_TEMPLATE_NAME_POSTFIX);

        result
    }

    fn get_optional_value(&self, value: String) -> Result<String, TemplateCodeGenerationError> {
        Ok(self.registry.render(TemplateResolver::OPTIONAL_TEMPLATE, &SingleValueHolder { value })?)
    }

    fn get_array_value(&self, value: String) -> Result<String, TemplateCodeGenerationError> {
        Ok(self.registry.render(TemplateResolver::ARRAY_TEMPLATE, &SingleValueHolder { value })?)
    }
}

#[derive(Serialize)]
/// Wraps a single String so it can be processed by handlebars.
struct SingleValueHolder {
    pub value: String
}

/// TODO: Test Constructor, resolve_each, resolve_single
#[cfg(test)]
mod tests {
    use crate::code_generator::template::template_resolver::TemplateResolver;
    use crate::raw_api::field_type::FieldType;

    use super::Template;

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
        let mut template_mock = Template::new();
        template_mock.expect_get_integer_type().return_const(String::from("u64"));
        template_mock.expect_get_string_type().return_const(String::from("String"));
        template_mock.expect_get_boolean_type().return_const(String::from("bool"));
        template_mock.expect_get_array_type().return_const(String::from("Vec<{{{value}}}>"));
        template_mock.expect_get_optional_type().return_const(String::from("Option<{{{value}}}>"));
        template_mock.expect_get_template_files().return_const(Vec::new());

        TemplateResolver::new(&template_mock).unwrap()
    }
}