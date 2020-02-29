use crate::code_generator::template::template_file::TemplateFile;
use crate::code_generator::template::template_listing::TemplateListing;

pub struct Template {
    integer_type: String,
    string_type: String,
    boolean_type: String,
    optional_type: String,
    template_files: Vec<TemplateFile>,
    custom_template_listings: Vec<TemplateListing>
}

impl Template {
    pub fn get_integer_type(&self) -> &String {
        &self.integer_type
    }

    pub fn get_string_type(&self) -> &String {
        &self.string_type
    }

    pub fn get_boolean_type(&self) -> &String {
        &self.boolean_type
    }

    pub fn get_optional_type(&self) -> &String {
        &self.optional_type
    }

    pub fn get_template_files(&self) -> &Vec<TemplateFile> {
        &self.template_files
    }

    pub fn get_custom_template_listings(&self) -> &Vec<TemplateListing> {
        &self.custom_template_listings
    }
}