use std::fmt;

use serde::export::Formatter;

use crate::code_generator::template::template::Template;

pub type Result = std::result::Result<(), ValidatorRuleError>;

pub trait ValidatorRule {
    fn validate(&self, template: &dyn Template) -> Result;
}

#[derive(Debug, Eq, PartialEq)]
pub enum ValidatorRuleError {
    ArrayTemplateIncorrect,
    OptionalTemplateIncorrect
}

impl ValidatorRuleError {
    fn get_message(&self) -> String {
        match self {
            ValidatorRuleError::ArrayTemplateIncorrect => String::from("A template for an array type must contain the key \"value\"! For example Array<{{value}}>"),
            ValidatorRuleError::OptionalTemplateIncorrect => String::from("A template for an optional type must contain the key \"value\"! For example Optional<{{value}}>")
        }
    }
}

impl std::error::Error for ValidatorRuleError {}

impl std::fmt::Display for ValidatorRuleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.get_message())
    }
}