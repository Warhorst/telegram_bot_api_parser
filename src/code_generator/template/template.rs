use crate::code_generator::template::template_file::TemplateFile;
use mockall::automock;

/// Trait for a template that is extracted from the templates.json.
#[automock]
pub trait Template {
    fn get_integer_type(&self) -> &String;

    fn get_string_type(&self) -> &String;

    fn get_boolean_type(&self) -> &String;

    fn get_array_type(&self) -> &String;

    fn get_optional_type(&self) -> &String;

    fn get_template_files(&self) -> &Vec<TemplateFile>;
}

/// Default implementation, holding all values as fields.
pub struct DefaultTemplate {
    integer_type: String,
    string_type: String,
    boolean_type: String,
    array_type: String,
    optional_type: String,
    template_files: Vec<TemplateFile>
}

impl Template for DefaultTemplate {
    fn get_integer_type(&self) -> &String {
        &self.integer_type
    }

    fn get_string_type(&self) -> &String {
        &self.string_type
    }

    fn get_boolean_type(&self) -> &String {
        &self.boolean_type
    }

    fn get_array_type(&self) -> &String {
        &self.array_type
    }

    fn get_optional_type(&self) -> &String {
        &self.optional_type
    }

    fn get_template_files(&self) -> &Vec<TemplateFile> {
        &self.template_files
    }
}