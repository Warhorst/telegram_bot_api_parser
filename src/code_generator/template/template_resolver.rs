use handlebars::{Handlebars, TemplateError};
use serde::Serialize;
cfg_if! {
    if #[cfg(test)] {
        use crate::code_generator::template::template::MockTemplate as Template;
    } else {
        use crate::code_generator::template::template::Template;
    }
}

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

    /// Creates a new template-resolver from a given template.
    /// Types of integer, string and boolean are not expected to contain any templates.
    /// Anything else is registered in a handlebars template registry.
    pub fn new(template: Template) -> Result<Self, TemplateError> {
        let mut handlebars = Handlebars::new();

        handlebars.register_template_string(TemplateResolver::ARRAY_TEMPLATE, template.get_array_type())?;
        handlebars.register_template_string(TemplateResolver::OPTIONAL_TEMPLATE, template.get_optional_type())?;

        Ok(TemplateResolver {
            integer_type: template.get_integer_type().to_owned(),
            string_type: template.get_string_type().to_owned(),
            boolean_type: template.get_boolean_type().to_owned(),
            registry: handlebars,
        })
    }

    pub fn get_optional_value(&self, value: String) -> String {
        self.registry.render(TemplateResolver::OPTIONAL_TEMPLATE, &ValueHolder { value }).unwrap()
    }

    pub fn get_array_value(&self, value: String) -> String {
        self.registry.render(TemplateResolver::ARRAY_TEMPLATE, &ValueHolder { value }).unwrap()
    }
}

#[derive(Serialize)]
/// Wraps a single String so it can be processed by handlebars.
struct ValueHolder {
    pub value: String
}

#[cfg(test)]
mod tests {
    use crate::code_generator::template::template_resolver::TemplateResolver;
    use super::Template;

    #[test]
    fn success_get_optional() {
        assert_eq!(create_resolver().get_optional_value(String::from("Update")), String::from("Option<Update>"))
    }

    #[test]
    fn success_get_array() {
        assert_eq!(create_resolver().get_array_value(String::from("Update")), String::from("Vec<Update>"))
    }

    fn create_resolver() -> TemplateResolver<'static> {
        let mut template_mock = Template::default();
        template_mock.expect_get_integer_type().return_const(String::from("u64"));
        template_mock.expect_get_string_type().return_const(String::from("String"));
        template_mock.expect_get_boolean_type().return_const(String::from("bool"));
        template_mock.expect_get_array_type().return_const(String::from("Vec<{{value}}>"));
        template_mock.expect_get_optional_type().return_const(String::from("Option<{{value}}>"));

        TemplateResolver::new(template_mock).unwrap()
    }
}