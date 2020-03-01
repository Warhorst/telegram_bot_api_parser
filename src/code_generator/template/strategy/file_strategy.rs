use std::convert::TryFrom;
use std::fmt::{Formatter, Error};
use core::fmt;
use crate::code_generator::template::strategy::file_strategy::FileStrategy::{ForEachDTO, Once};

/// Represents how a template-file should be processed.
pub enum FileStrategy {
    ForEachDTO,
    Once
}

impl TryFrom<&String> for FileStrategy {
    type Error = NoValidFileStrategyError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FOR_EACH_DTO" => Ok(ForEachDTO),
            "ONCE" => Ok(Once),
            _ => Err(NoValidFileStrategyError { value: value.clone() })
        }
    }
}

#[derive(Debug)]
pub struct NoValidFileStrategyError {
    pub value: String
}

impl std::error::Error for NoValidFileStrategyError {}

impl std::fmt::Display for NoValidFileStrategyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The given value {} is not a valid strategy!", self.value)
    }
}