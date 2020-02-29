use crate::code_generator::code_generator::CodeGenerator;
use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
use crate::code_generator::template::template::Template;
use crate::code_generator::template::template_code_generation_error::TemplateCodeGenerationError;
use std::fs::read_to_string;

pub struct TemplateCodeGenerator {
    template: Template
}

impl TemplateCodeGenerator {
    pub fn new(template: Template) -> Self  {
        TemplateCodeGenerator { template }
    }
}

impl CodeGenerator for TemplateCodeGenerator {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: TelegramBotApiRaw) -> Result<(), Self::Error> {
        for template_file in self.template.get_template_files() {
            let content = read_to_string(template_file.get_template_path())?;
        }

        Ok(())

        //template analysieren
        // 1. Typ-Entsprechungen entnehmen, für array und optional Templates registrieren
        // 2. Template-Dateien sichten, je nach hinterlegter Strategie auflösen
        // 3. Betrachtete Datei auslesen (als  template registrieren)
    }
}