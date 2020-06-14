use handlebars::Handlebars;
use serde::Serialize;

use crate::code_generator::api::DtoName;
use crate::code_generator::configuration::{Configuration, Rename, TemplateFile};
use crate::code_generator::target_files::TargetFile;
use crate::raw_api::type_descriptor::TypeDescriptor;

pub trait Renderer {
    fn from_configuration(configuration: Configuration) -> Self;

    fn render_field_type(&self, field_type: &TypeDescriptor) -> String;

    fn render_template_file_dtos<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> TargetFile;

    fn render_rename(&self, field_name: String, field_rename_values: &DtoName) -> String;
}

pub struct RendererImpl<'a> {
    registry: Handlebars<'a>,
    integer_type: String,
    string_type: String,
    boolean_type: String
}

impl<'a> Renderer for RendererImpl<'a> {
    fn from_configuration(configuration: Configuration) -> Self {
        let mut  registry = Handlebars::new();

        Self::register_array_template(&mut registry, configuration.array_type);
        Self::register_optional_template(&mut registry, configuration.optional_type);

        for template_file in &configuration.template_files {
            Self::register_template_file(&mut registry, template_file)
        }

        for rename in &configuration.renames {
            Self::register_rename(&mut registry, rename)
        }

        RendererImpl {
            registry,
            integer_type: configuration.integer_type,
            string_type: configuration.string_type,
            boolean_type: configuration.boolean_type
        }
    }

    fn render_field_type(&self, field_type: &TypeDescriptor) -> String {
        match field_type {
            TypeDescriptor::Integer => self.integer_type.clone(),
            TypeDescriptor::String => self.string_type.clone(),
            TypeDescriptor::Boolean => self.boolean_type.clone(),
            TypeDescriptor::DTO(dto_name) => dto_name.clone(),
            TypeDescriptor::ArrayOf(array_field_type) => self.render_array_string(self.render_field_type(array_field_type)),
            TypeDescriptor::Optional(optional_field_type) => self.render_optional_string(self.render_field_type(optional_field_type))
        }
    }

    fn render_template_file_dtos<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> TargetFile {
        let template_path = &template_file.template_path;
        let filename = self.registry.render(Self::get_file_name_template_name(&template_path).as_str(), dtos).unwrap();
        let content = self.registry.render(&template_path, dtos).unwrap();

        TargetFile {
            file_name: filename,
            content
        }
    }

    fn render_rename(&self, field_name: String, field_rename_values: &DtoName) -> String {
        let template_name = Self::get_rename_template_name(&field_name);

        if self.registry.has_template(template_name.as_str()) {
            return self.registry.render(template_name.as_str(), field_rename_values).unwrap()
        }

        field_name
    }
}

impl<'a> RendererImpl<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";
    const RENAME_POSTFIX: &'static str = "_rename";

    fn register_array_template(registry: &mut Handlebars, array_string: String) {
        registry.register_template_string(Self::ARRAY_TEMPLATE, array_string).unwrap()
    }

    fn register_optional_template(registry: &mut Handlebars, optional_string: String) {
        registry.register_template_string(Self::OPTIONAL_TEMPLATE, optional_string).unwrap()
    }

    fn register_template_file(registry: &mut Handlebars, template_file: &TemplateFile) {
        let template_path = &template_file.template_path;
        registry.register_template_string(Self::get_file_name_template_name(&template_path).as_str(), &template_file.target_path).unwrap();
        registry.register_template_file(template_path.as_str(), template_path).unwrap();
    }

    fn register_rename(registry: &mut Handlebars, rename: &Rename) {
        let template_name = Self::get_rename_template_name(&rename.from);
        registry.register_template_string(template_name.as_str(), &rename.to).unwrap();
    }

    fn get_file_name_template_name(path: &String) -> String {
        let mut result = String::from(path.as_str());
        result.push_str(Self::FILE_NAME_TEMPLATE_NAME_POSTFIX);
        result
    }

    fn get_rename_template_name(from: &String) -> String {
        let mut result = String::from(from.as_str());
        result.push_str(Self::RENAME_POSTFIX);
        result
    }

    fn render_array_string(&self, value: String) -> String {
        self.registry.render(Self::ARRAY_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }

    fn render_optional_string(&self, value: String) -> String {
        self.registry.render(Self::OPTIONAL_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }
}

#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

#[cfg(test)]
mod tests {
    use crate::code_generator::configuration::Configuration;
    use crate::code_generator::renderer::{Renderer, RendererImpl};
    use crate::raw_api::type_descriptor::TypeDescriptor;

    #[test]
    fn success_render_array() {
        let renderer = create_renderer();
        let value = String::from("Foo");

        let wrapped_value = renderer.render_array_string(value.clone());

        assert_eq!(wrapped_value, format!("Vec<{}>", value))
    }

    #[test]
    fn success_render_optional() {
        let renderer = create_renderer();
        let value = String::from("Foo");

        let wrapped_value = renderer.render_optional_string(value.clone());

        assert_eq!(wrapped_value, format!("Option<{}>", value))
    }

    #[test]
    fn success_render_field_type() {
        let renderer = create_renderer();
        let input_expected = vec![
            (renderer.render_field_type(&TypeDescriptor::Integer), String::from("u64")),
            (renderer.render_field_type(&TypeDescriptor::String), String::from("String")),
            (renderer.render_field_type(&TypeDescriptor::Boolean), String::from("bool")),
            (renderer.render_field_type(&TypeDescriptor::Optional(Box::new(TypeDescriptor::DTO(String::from("Update"))))), String::from("Option<Update>")),
            (renderer.render_field_type(&TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("Update"))))), String::from("Vec<Update>")),
            (renderer.render_field_type(&TypeDescriptor::Optional(Box::new(TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("Update"))))))), String::from("Option<Vec<Update>>"))
        ];

        input_expected.into_iter().for_each(|(input, expected)| assert_eq!(input, expected));
    }

    fn create_renderer() -> RendererImpl<'static> {
        let configuration = Configuration {
            integer_type: String::from("u64"),
            string_type: String::from("String"),
            boolean_type: String::from("bool"),
            array_type: String::from("Vec<{{{value}}}>"),
            optional_type: String::from("Option<{{{value}}}>"),
            renames: Vec::new(),
            template_files: Vec::new()
        };

        RendererImpl::from_configuration(configuration)
    }
}