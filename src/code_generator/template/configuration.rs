use serde::Deserialize;

/// A template that is extracted from the configuration.json.
#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    pub integer_type: String,
    pub string_type: String,
    pub boolean_type: String,
    pub array_type: String,
    pub optional_type: String,
    pub renames: Vec<Rename>,
    pub template_files: Vec<TemplateFile>
}

/// Contains a field name that should be renamed (because the name is invalid in the target language for example).
#[derive(Deserialize, Debug, Clone)]
pub struct Rename {
    pub from: String,
    pub to: String
}

/// Contains the data of a template-file and how it should be transformed into generated code
#[derive(Deserialize, Debug, Clone)]
pub struct TemplateFile {
    pub template_path: String,
    pub target_path: String,
    pub resolve_strategy: String,
}