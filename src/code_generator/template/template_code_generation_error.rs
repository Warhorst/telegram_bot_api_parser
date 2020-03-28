use core::fmt;
use crate::code_generator::target_files_map::SameFilenameError;
use crate::code_generator::template::resolve_strategy::NoValidResolveStrategyError;
use handlebars::{TemplateError, TemplateFileError, RenderError};

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