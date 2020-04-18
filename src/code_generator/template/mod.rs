use core::fmt;

use handlebars::{RenderError, TemplateError, TemplateFileError};

use crate::code_generator::CodeGenerator;
use crate::code_generator::target_files::SameFilenameError;
use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::configuration::Configuration;
use crate::code_generator::template::resolve_strategy::NoValidResolveStrategyError;
use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::raw_api::RawApi;

mod template_resolver;
mod resolve_strategy;
mod objects;
pub mod configuration;
pub mod configuration_reader;

/// Generates code fom a given JSON-template.
pub struct TemplateCodeGenerator {
    configuration: Configuration
}

impl TemplateCodeGenerator {
    pub fn new(template: Configuration) -> Self {
        TemplateCodeGenerator { configuration: template }
    }
}

impl CodeGenerator for TemplateCodeGenerator {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error> {
        let resolver = TemplateResolver::new(self.configuration.clone())?;
        resolver.resolve(api)
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