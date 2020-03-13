use std::convert::TryFrom;
use std::fs::read_to_string;

use crate::code_generator::code_generator::CodeGenerator;
use crate::code_generator::target_files_map::TargetFilesMap;
use crate::code_generator::template::resolve_strategy::ResolveStrategy;
use crate::code_generator::template::template_code_generation_error::TemplateCodeGenerationError;
use crate::code_generator::template::template_file::TemplateFile;
use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
cfg_if! {
    if #[cfg(test)] {
        use crate::code_generator::template::template::MockTemplate as Template;
    } else {
        use crate::code_generator::template::template::Template;
    }
}

/// Generates code fom a given JSON-template.
pub struct TemplateCodeGenerator {
    template: Template
}

impl TemplateCodeGenerator {
    pub fn new(template: Template) -> Self {
        TemplateCodeGenerator { template }
    }
}

impl CodeGenerator for TemplateCodeGenerator {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: TelegramBotApiRaw) -> Result<TargetFilesMap, Self::Error> {
        let mut result = TargetFilesMap::new();
        let resolver = TemplateResolver::new(&self.template)?;
        let dtos = api.get_bot_dtos();

        for template_file in self.template.get_template_files() {
            let file_strategy = ResolveStrategy::try_from(template_file.get_resolve_strategy())?;

            match file_strategy {
                ResolveStrategy::ForAllDTOs => result.insert_all(resolver.resolve_for_each_dto(template_file, dtos)?)?,
                ResolveStrategy::ForEachDTO => {
                    for dto in dtos {
                        result.insert_all(resolver.resolve_for_single_dto(template_file, dto)?)?
                    }
                }
            };
        }

        Ok(result)
    }
}