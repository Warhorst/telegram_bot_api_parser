use std::convert::TryFrom;

use crate::code_generator::code_generator::CodeGenerator;
use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::resolve_strategy::ResolveStrategy;
use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
use core::fmt;
use crate::code_generator::target_files::SameFilenameError;
use crate::code_generator::template::resolve_strategy::NoValidResolveStrategyError;
use handlebars::{TemplateError, TemplateFileError, RenderError};

mockuse!(crate::code_generator::template::template, Template, MockTemplate);

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

    fn generate(&self, api: TelegramBotApiRaw) -> Result<TargetFiles, Self::Error> {
        let mut result = TargetFiles::new();
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

#[derive(Debug)]
pub enum TemplateCodeGenerationError {
    NoValidResolveStrategyError(NoValidResolveStrategyError),
    IoError(std::io::Error),
    SameFilenameError(SameFilenameError),
    HandlebarsTemplateError(TemplateError),
    HandlebarsTemplateFileError(TemplateFileError),
    HandlebarsRenderError(RenderError)
}

impl std::error::Error for TemplateCodeGenerationError {}

impl std::fmt::Display for TemplateCodeGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateCodeGenerationError::NoValidResolveStrategyError(e) => e.fmt(f),
            TemplateCodeGenerationError::IoError(e) => e.fmt(f),
            TemplateCodeGenerationError::SameFilenameError(e) => e.fmt(f),
            TemplateCodeGenerationError::HandlebarsTemplateError(e) => e.fmt(f),
            TemplateCodeGenerationError::HandlebarsTemplateFileError(e) => std::fmt::Display::fmt(&e, f),
            TemplateCodeGenerationError::HandlebarsRenderError(e) => e.fmt(f)
        }
    }
}

impl From<NoValidResolveStrategyError> for TemplateCodeGenerationError {
    fn from(e: NoValidResolveStrategyError) -> Self {
        TemplateCodeGenerationError::NoValidResolveStrategyError(e)
    }
}

impl From<std::io::Error> for TemplateCodeGenerationError {
    fn from(e: std::io::Error) -> Self {
        TemplateCodeGenerationError::IoError(e)
    }
}

impl From<SameFilenameError> for TemplateCodeGenerationError {
    fn from(e: SameFilenameError) -> Self {
        TemplateCodeGenerationError::SameFilenameError(e)
    }
}

impl From<TemplateError> for TemplateCodeGenerationError {
    fn from(e: TemplateError) -> Self {
        TemplateCodeGenerationError::HandlebarsTemplateError(e)
    }
}

impl From<TemplateFileError> for TemplateCodeGenerationError {
    fn from(e: TemplateFileError) -> Self {
        TemplateCodeGenerationError::HandlebarsTemplateFileError(e)
    }
}

impl From<RenderError> for TemplateCodeGenerationError {
    fn from(e: RenderError) -> Self {
        TemplateCodeGenerationError::HandlebarsRenderError(e)
    }
}