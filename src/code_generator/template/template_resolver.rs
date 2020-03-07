use handlebars::{Handlebars, TemplateError};
use serde::Serialize;
use crate::raw_api::bot_dto::BotDTO;
use crate::code_generator::template::strategy::file_strategy::FileStrategy;
use std::convert::TryFrom;
use crate::util::to_snake_case;
use crate::code_generator::template::template_file::TemplateFile;
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

    pub fn resolve_for_each_dto(&self, template_file: &TemplateFile, dtos: &Vec<BotDTO>) {}

    pub fn resolve_for_dto(&self, template_file: &TemplateFile, dto: &BotDTO) {
        let mut tmp_registry = Handlebars::new();
        let listings = template_file.get_template_listings().as_ref().unwrap();

        for listing in listings {
            tmp_registry.register_template_string("curr_item", listing.get_item_template());

            for field in dto.get_fields() {
                // 1. Serialize-Objekt Feld-Name/Typ/TypSnakeCase erstellen
                // 2. String rendern
                // 3. erstellten String an Gesamtstring anheften
                // 4. template-listing mit Gesamtstring auflösen

                // let value_holder = FieldValueHolder::new(field)
            }
        }
    }

    fn get_optional_value(&self, value: String) -> String {
        self.registry.render(TemplateResolver::OPTIONAL_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }

    fn get_array_value(&self, value: String) -> String {
        self.registry.render(TemplateResolver::ARRAY_TEMPLATE, &SingleValueHolder { value }).unwrap()
    }
}

#[derive(Serialize)]
/// Wraps a single String so it can be processed by handlebars.
struct SingleValueHolder {
    pub value: String
}

#[derive(Serialize)]
/// Wraps all values of a field so it can be processed by handlebars.
struct FieldValueHolder {
    name: String,
    field_type: String,
    field_type_snake_case: String,
}

impl FieldValueHolder {
    pub fn new(name: String, field_type: String) -> Self {
        let field_type_snake_case = to_snake_case(&field_type);

        FieldValueHolder {
            name,
            field_type,
            field_type_snake_case
        }
    }
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