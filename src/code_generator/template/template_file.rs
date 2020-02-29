/// Contains the data of a template-file and how it should be transformed into generated code
pub struct TemplateFile {
    template_path: String,
    target_path: String,
    strategy: String
}

impl TemplateFile {
    pub fn get_template_path(&self) -> &String {
        &self.template_path
    }

    pub fn get_target_path(&self) -> &String {
        &self.target_path
    }

    pub fn get_strategy(&self) -> &String {
        &self.strategy
    }
}