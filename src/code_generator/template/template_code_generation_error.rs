use crate::code_generator::template::strategy::file_strategy::NoValidFileStrategyError;
use serde::export::Formatter;
use serde::export::fmt::Error;
use core::fmt;
use crate::code_generator::target_files_map::SameFilenameError;

#[derive(Debug)]
pub enum TemplateCodeGenerationError {
    NoValidFileStrategyError(NoValidFileStrategyError),
    IoError(std::io::Error),
    SameFilenameError(SameFilenameError)
}

impl std::error::Error for TemplateCodeGenerationError {}

impl std::fmt::Display for TemplateCodeGenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TemplateCodeGenerationError::NoValidFileStrategyError(e) => e.fmt(f),
            TemplateCodeGenerationError::IoError(e) => e.fmt(f),
            TemplateCodeGenerationError::SameFilenameError(e) => e.fmt(f)
        }
    }
}

impl From<NoValidFileStrategyError> for TemplateCodeGenerationError {
    fn from(e: NoValidFileStrategyError) -> Self {
        TemplateCodeGenerationError::NoValidFileStrategyError(e)
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