use serde::export::Formatter;
use std::fmt;
use crate::code_generator::template::validation::validator_rule::{ValidatorRule, ValidatorRuleError};
use crate::code_generator::template::template::{Template};
use crate::code_generator::template::validation::optional_template_validator_rule::OptionalTemplateValidatorRule;
use crate::code_generator::template::validation::array_template_validator_rule::ArrayTemplateValidatorRule;

pub type Result = std::result::Result<(), ValidationError>;

/// Validates a Template.
pub trait Validator {
    fn validate_template(&self, template: &dyn Template) -> Result;
}

/// Validates a Template with a given set of rules.
pub struct DefaultValidator {
    rules: Vec<Box<dyn ValidatorRule>>
}

impl DefaultValidator {
    pub fn new() -> Self {
        DefaultValidator {
            rules: vec![
                Box::new(OptionalTemplateValidatorRule {}),
                Box::new(ArrayTemplateValidatorRule {})
            ]
        }
    }
}

impl Validator for DefaultValidator {
    fn validate_template(&self, template: &dyn Template) -> Result {
        let mut errors = Vec::new();
        self.rules
            .iter()
            .map(|rule| rule.validate(template))
            .for_each(|result| if let Err(e) = result {
                errors.push(e)
            });

        if !errors.is_empty() {
            return Err(ValidationError::new(errors));
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ValidationError {
    validation_rule_errors: Vec<ValidatorRuleError>
}

impl ValidationError {
    pub fn new(validation_rule_errors: Vec<ValidatorRuleError>) -> Self {
        ValidationError {
            validation_rule_errors
        }
    }
}

impl std::error::Error for ValidationError {}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Some errors occured while validating the template:")?;

        for error in &self.validation_rule_errors {
            error.fmt(f)?;
        }

        Ok(())
    }
}