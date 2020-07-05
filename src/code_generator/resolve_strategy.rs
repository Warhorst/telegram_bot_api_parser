use core::fmt;
use std::convert::TryFrom;
use std::fmt::Formatter;

use crate::code_generator::resolve_strategy::ResolveStrategy::{ForAllDTOs, ForEachDTO, ForAllMethods, ForEachMethod};

/// Represents how a template-file should be processed.
pub enum ResolveStrategy {
    ForAllDTOs,
    ForEachDTO,
    ForAllMethods,
    ForEachMethod
}

impl TryFrom<&String> for ResolveStrategy {
    type Error = NoValidResolveStrategyError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FOR_ALL_DTOS" => Ok(ForAllDTOs),
            "FOR_EACH_DTO" => Ok(ForEachDTO),
            "FOR_ALL_METHODS" => Ok(ForAllMethods),
            "FOR_EACH_METHOD" => Ok(ForEachMethod),
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