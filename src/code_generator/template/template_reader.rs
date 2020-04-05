use crate::code_generator::template::template::Template;
use std::fs::File;

pub struct TemplateReader;

impl TemplateReader {
    pub fn read(&self) -> Result<Template, String> {
        let template = serde_json::from_reader(File::open("templates/templates.json").unwrap()).unwrap();

        Ok(template)
    }
}