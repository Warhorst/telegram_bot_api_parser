use std::convert::TryFrom;
use std::fmt::Formatter;
use core::fmt;
use crate::code_generator::template::resolve_strategy::ResolveStrategy::{ForAllDTOs, ForEachDTO};

/// Represents how a template-file should be processed.
pub enum ResolveStrategy {
    ForAllDTOs,
    ForEachDTO
}

impl TryFrom<&String> for ResolveStrategy {
    type Error = NoValidResolveStrategyError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FOR_ALL_DTOS" => Ok(ForAllDTOs),
            "FOR_EACH_DTO" => Ok(ForEachDTO),
            _ => Err(NoValidResolveStrategyError { value: value.clone() })
        }
    }
}

#[derive(Debug)]
pub struct NoValidResolveStrategyError {
    pub value: String
}

impl std::error::Error for NoValidResolveStrategyError {}

impl std::fmt::Display for NoValidResolveStrategyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "The given value {} is not a valid strategy!", self.value)
    }
}