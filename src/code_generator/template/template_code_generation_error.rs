use crate::code_generator::template::strategy::file_strategy::NoValidFileStrategyError;
use serde::export::Formatter;
use serde::export::fmt::Error;
use core::fmt;

#[derive(Debug)]
pub enum TemplateCodeGenerationError {
    NoValidFileStrategyError(NoValidFileStrategyError),
    IoError(std::io::Error)
}

impl std::error::Error for TemplateCodeGenerationError {}

impl std::fmt::Display for TemplateCodeGenerationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TemplateCodeGenerationError::NoValidFileStrategyError(e) => e.fmt(f),
            TemplateCodeGenerationError::IoError(e) => e.fmt(f)
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