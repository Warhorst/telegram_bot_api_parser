use serde::Deserialize;

/// Contains the data of a template-file and how it should be transformed into generated code
#[derive(Deserialize, Debug)]
pub struct TemplateFile {
    template_path: String,
    target_path: String,
    resolve_strategy: String,
}

impl TemplateFile {
    pub fn get_template_path(&self) -> &String {
        &self.template_path
    }

    pub fn get_target_path(&self) -> &String {
        &self.target_path
    }

    pub fn get_resolve_strategy(&self) -> &String {
        &self.resolve_strategy
    }
}