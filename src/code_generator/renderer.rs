use handlebars::{Handlebars, TemplateError, TemplateFileError, RenderError};
use serde::Serialize;

use crate::code_generator::configuration::{Configuration, Rename, TemplateFile};
use crate::code_generator::target_files::TargetFile;
use crate::raw_api::type_descriptor::TypeDescriptor;
use crate::code_generator::names::Names;
use crate::code_generator::api::{Dtos, Methods};
use crate::code_generator::api::dto::Dto;
use crate::code_generator::api::method::Method;
use serde::export::Formatter;

pub trait Renderer {
    type Error: std::error::Error;

    fn from_configuration(configuration: Configuration) -> Result<Self, Self::Error>
    where Self: Sized;

    fn render_for_single_dto(&self, dto: &Dto, template_file: &TemplateFile) -> Result<TargetFile, Self::Error>;

    fn render_for_all_dtos(&self, dto: &Dtos, template_file: &TemplateFile) -> Result<TargetFile, Self::Error>;

    fn render_for_single_method(&self, method: &Method, template_file: &TemplateFile) -> Result<TargetFile, Self::Error>;

    fn render_for_all_methods(&self, method: &Methods, template_file: &TemplateFile) -> Result<TargetFile, Self::Error>;

    fn render_type(&self, field_type: &TypeDescriptor) -> Result<String, Self::Error>;

    fn render_rename(&self, field_name: String, field_rename_values: &Names) -> Result<String, Self::Error>;
}

pub struct RendererImpl<'a> {
    registry: Handlebars<'a>,
    integer_type: String,
    string_type: String,
    boolean_type: String
}

impl<'a> Renderer for RendererImpl<'a> {
    type Error = HandlebarsRenderError;

    fn from_configuration(configuration: Configuration) -> Result<Self, Self::Error> {
        let mut  registry = Handlebars::new();

        Self::register_array_template(&mut registry, configuration.array_type)?;
        Self::register_optional_template(&mut registry, configuration.optional_type)?;

        for template_file in &configuration.template_files {
            Self::register_template_file(&mut registry, template_file)?
        }

        for rename in &configuration.renames {
            Self::register_rename(&mut registry, rename)?
        }

        Ok(RendererImpl {
            registry,
            integer_type: configuration.integer_type,
            string_type: configuration.string_type,
            boolean_type: configuration.boolean_type
        })
    }

    fn render_for_single_dto(&self, dto: &Dto, template_file: &TemplateFile) -> Result<TargetFile, Self::Error> {
        self.render_instance(dto, template_file)
    }

    fn render_for_all_dtos(&self, dtos: &Dtos, template_file: &TemplateFile) -> Result<TargetFile, Self::Error> {
        self.render_instance(dtos, template_file)
    }

    fn render_for_single_method(&self, method: &Method, template_file: &TemplateFile) -> Result<TargetFile, Self::Error> {
        self.render_instance(method, template_file)
    }

    fn render_for_all_methods(&self, methods: &Methods, template_file: &TemplateFile) -> Result<TargetFile, Self::Error> {
        self.render_instance(methods, template_file)
    }

    fn render_type(&self, field_type: &TypeDescriptor) -> Result<String, Self::Error> {
        Ok(match field_type {
            TypeDescriptor::Integer => self.integer_type.clone(),
            TypeDescriptor::String => self.string_type.clone(),
            TypeDescriptor::Boolean => self.boolean_type.clone(),
            TypeDescriptor::DTO(dto_name) => dto_name.clone(),
            TypeDescriptor::ArrayOf(array_field_type) => self.render_array_string(self.render_type(array_field_type)?)?,
            TypeDescriptor::Optional(optional_field_type) => self.render_optional_string(self.render_type(optional_field_type)?)?
        })
    }

    fn render_rename(&self, field_name: String, field_rename_values: &Names) -> Result<String, Self::Error> {
        let template_name = Self::get_rename_template_name(&field_name);

        if self.registry.has_template(template_name.as_str()) {
            let renamed = self.registry.render(template_name.as_str(), field_rename_values)?;
            return Ok(renamed)
        }

        Ok(field_name)
    }
}

impl<'a> RendererImpl<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";
    const RENAME_POSTFIX: &'static str = "_rename";

    fn register_array_template(registry: &mut Handlebars, array_string: String) -> Result<(), HandlebarsRenderError> {
        registry.register_template_string(Self::ARRAY_TEMPLATE, array_string)?;
        Ok(())
    }

    fn register_optional_template(registry: &mut Handlebars, optional_string: String) -> Result<(), HandlebarsRenderError> {
        registry.register_template_string(Self::OPTIONAL_TEMPLATE, optional_string)?;
        Ok(())
    }

    fn register_template_file(registry: &mut Handlebars, template_file: &TemplateFile) -> Result<(), HandlebarsRenderError> {
        let template_path = &template_file.template_path;
        registry.register_template_string(Self::get_file_name_template_name(&template_path).as_str(), &template_file.target_path)?;
        registry.register_template_file(template_path.as_str(), template_path)?;
        Ok(())
    }

    fn register_rename(registry: &mut Handlebars, rename: &Rename) -> Result<(), HandlebarsRenderError> {
        let template_name = Self::get_rename_template_name(&rename.from);
        registry.register_template_string(template_name.as_str(), &rename.to)?;
        Ok(())
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

    fn render_instance<T: Serialize>(&self, instance: &T, template_file: &TemplateFile) -> Result<TargetFile, HandlebarsRenderError> {
        let template_path = &template_file.template_path;
        let file_name = self.registry.render(Self::get_file_name_template_name(&template_path).as_str(), instance)?;
        let content = self.registry.render(&template_path, instance)?;

        Ok(TargetFile {
            file_name,
            content
        })
    }

    fn render_array_string(&self, value: String) -> Result<String, HandlebarsRenderError> {
        let array_string = self.registry.render(Self::ARRAY_TEMPLATE, &SingleValueHolder { value })?;
        Ok(array_string)
    }

    fn render_optional_string(&self, value: String) -> Result<String, HandlebarsRenderError> {
        let optional_string = self.registry.render(Self::OPTIONAL_TEMPLATE, &SingleValueHolder { value })?;
        Ok(optional_string)
    }
}

#[derive(Debug)]
pub enum HandlebarsRenderError {
    TemplateError(TemplateError),
    TemplateFileError(TemplateFileError),
    RenderError(RenderError)
}

impl std::error::Error for HandlebarsRenderError {}

impl std::fmt::Display for HandlebarsRenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HandlebarsRenderError::TemplateError(error) => error.fmt(f),
            HandlebarsRenderError::TemplateFileError(error) => error.fmt(f),
            HandlebarsRenderError::RenderError(error) => error.fmt(f),
        }
    }
}

impl From<TemplateError> for HandlebarsRenderError {
    fn from(error: TemplateError) -> Self {
        HandlebarsRenderError::TemplateError(error)
    }
}

impl From<TemplateFileError> for HandlebarsRenderError {
    fn from(error: TemplateFileError) -> Self {
        HandlebarsRenderError::TemplateFileError(error)
    }
}

impl From<RenderError> for HandlebarsRenderError {
    fn from(error: RenderError) -> Self {
        HandlebarsRenderError::RenderError(error)
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

        let wrapped_value = renderer.render_array_string(value.clone()).unwrap();

        assert_eq!(wrapped_value, format!("Vec<{}>", value))
    }

    #[test]
    fn success_render_optional() {
        let renderer = create_renderer();
        let value = String::from("Foo");

        let wrapped_value = renderer.render_optional_string(value.clone()).unwrap();

        assert_eq!(wrapped_value, format!("Option<{}>", value))
    }

    #[test]
    fn success_render_field_type() {
        let renderer = create_renderer();
        let input_expected = vec![
            (renderer.render_type(&TypeDescriptor::Integer).unwrap(), String::from("u64")),
            (renderer.render_type(&TypeDescriptor::String).unwrap(), String::from("String")),
            (renderer.render_type(&TypeDescriptor::Boolean).unwrap(), String::from("bool")),
            (renderer.render_type(&TypeDescriptor::Optional(Box::new(TypeDescriptor::DTO(String::from("Update"))))).unwrap(), String::from("Option<Update>")),
            (renderer.render_type(&TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("Update"))))).unwrap(), String::from("Vec<Update>")),
            (renderer.render_type(&TypeDescriptor::Optional(Box::new(TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("Update"))))))).unwrap(), String::from("Option<Vec<Update>>"))
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

        RendererImpl::from_configuration(configuration).unwrap()
    }
}