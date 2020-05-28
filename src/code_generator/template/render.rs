use handlebars::Handlebars;
use serde::Serialize;

use crate::code_generator::target_files::TargetFile;
use crate::code_generator::template::configuration::TemplateFile;

/// Renders registered templates.
pub trait Renderer: Default {
    fn register_array_template(&mut self, array_string: String);

    fn register_optional_template(&mut self, object_string: String);

    fn register_template_file(&mut self, template_file: &TemplateFile);

    fn render_template_file_dtos<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> TargetFile;

    fn render_array_string(&self, value: String) -> String;

    fn render_optional_string(&self, value: String) -> String;
}

pub struct RendererImpl<'a> {
    registry: Handlebars<'a>
}

impl<'a> Renderer for RendererImpl<'a> {
    fn register_array_template(&mut self, array_string: String) {
        self.registry.register_template_string(Self::ARRAY_TEMPLATE, array_string).unwrap()
    }

    fn register_optional_template(&mut self, optional_string: String) {
        self.registry.register_template_string(Self::OPTIONAL_TEMPLATE, optional_string).unwrap()
    }

    fn register_template_file(&mut self, template_file: &TemplateFile) {
        let template_path = &template_file.template_path;
        self.registry.register_template_string(self.get_file_name_template_name(&template_path).as_str(), &template_file.target_path).unwrap();
        self.registry.register_template_file(template_path.as_str(), template_path).unwrap();
    }

    fn render_template_file_dtos<T: Serialize>(&self, template_file: &TemplateFile, dtos: &T) -> TargetFile {
        let template_path = &template_file.template_path;
        let filename = self.registry.render(self.get_file_name_template_name(&template_path).as_str(), dtos).unwrap();
        let content = self.registry.render(&template_path, dtos).unwrap();

        TargetFile {
            file_name: filename,
            content
        }
    }

    fn render_array_string(&self, value: String) -> String {
        self.registry.render(Self::ARRAY_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }

    fn render_optional_string(&self, value: String) -> String {
        self.registry.render(Self::OPTIONAL_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }
}

impl<'a> Default for RendererImpl<'a> {
    fn default() -> Self {
        RendererImpl {
            registry: Handlebars::new()
        }
    }
}

impl<'a> RendererImpl<'a> {
    const ARRAY_TEMPLATE: &'static str = "array";
    const OPTIONAL_TEMPLATE: &'static str = "optional";
    const FILE_NAME_TEMPLATE_NAME_POSTFIX: &'static str = "_name";

    fn get_file_name_template_name(&self, path: &String) -> String {
        let mut result = String::from(path.as_str());
        result.push_str(Self::FILE_NAME_TEMPLATE_NAME_POSTFIX);
        result
    }
}

#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

#[cfg(test)]
mod tests {
    use crate::code_generator::template::render::{Renderer, RendererImpl};

    #[test]
    fn success_render_array() {
        let mut renderer = RendererImpl::default();
        let array_template = String::from("Array<{{value}}>");
        let value = String::from("Foo");

        renderer.register_array_template(array_template);
        let wrapped_value = renderer.render_array_string(value.clone());

        assert_eq!(wrapped_value, format!("Array<{}>", value))
    }

    #[test]
    fn success_render_optional() {
        let mut renderer = RendererImpl::default();
        let optional_template = String::from("Optional<{{value}}>");
        let value = String::from("Foo");

        renderer.register_optional_template(optional_template);
        let wrapped_value = renderer.render_optional_string(value.clone());

        assert_eq!(wrapped_value, format!("Optional<{}>", value))
    }
}