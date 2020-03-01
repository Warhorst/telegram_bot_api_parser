use crate::code_generator::code_generator::CodeGenerator;
use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
use crate::code_generator::template::template::Template;
use crate::code_generator::template::template_code_generation_error::TemplateCodeGenerationError;
use std::fs::read_to_string;
use crate::code_generator::template::template_file::TemplateFile;
use crate::code_generator::template::strategy::file_strategy::FileStrategy;
use std::convert::TryFrom;
use crate::code_generator::target_files_map::TargetFilesMap;

/// Generates code fom a given JSON-template.
pub struct TemplateCodeGenerator {
    template: Template
}

impl TemplateCodeGenerator {
    pub fn new(template: Template) -> Self  {
        TemplateCodeGenerator { template }
    }

    fn resolve_once(&self, template_file: &TemplateFile, api: &TelegramBotApiRaw) {

    }

    fn resolve_for_each_dto(&self, template_file: &TemplateFile, api: &TelegramBotApiRaw) {

    }
}

impl CodeGenerator for TemplateCodeGenerator {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: TelegramBotApiRaw) -> Result<TargetFilesMap, Self::Error> {
        for template_file in self.template.get_template_files() {
            // let content = read_to_string(template_file.get_template_path())?;

            let file_strategy = FileStrategy::try_from(template_file.get_strategy())?;

            match file_strategy {
                FileStrategy::Once => self.resolve_once(template_file, &api),
                FileStrategy::ForEachDTO => self.resolve_for_each_dto(template_file, &api)
            }
        }


        unimplemented!()

        //template analysieren
        // 1. Typ-Entsprechungen entnehmen, für array und optional Templates registrieren
        // 2. Template-Dateien sichten, je nach hinterlegter Strategie auflösen
        // 3. Betrachtete Datei auslesen (als  template registrieren)
    }
}