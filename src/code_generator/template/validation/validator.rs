use std::fmt;

use serde::export::Formatter;

use crate::code_generator::template::validation::array_template_validator_rule::ArrayTemplateValidatorRule;
use crate::code_generator::template::validation::optional_template_validator_rule::OptionalTemplateValidatorRule;
use crate::code_generator::template::validation::validator_rule::{ValidatorRule, ValidatorRuleError};

mockuse!(crate::code_generator::template::template, Template, MockTemplate);

pub type Result = std::result::Result<(), ValidationError>;

/// Validates a Template with a given set of rules.
pub struct Validator {
    rules: Vec<Box<dyn ValidatorRule>>
}

impl Validator {
    pub fn new() -> Self {
        Validator::new_with_rules(
            vec![
                Box::new(OptionalTemplateValidatorRule {}),
                Box::new(ArrayTemplateValidatorRule {})
            ]
        )
    }

    pub fn new_with_rules(rules: Vec<Box<dyn ValidatorRule>>) -> Self {
        Validator {
            rules
        }
    }

    pub fn validate_template(&self, template: &Template) -> Result {
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

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::code_generator::template::validation::validator_rule::{ValidatorRule, ValidatorRuleError, Result};
    use super::Template;
    use crate::code_generator::template::validation::validator::{ValidationError, Validator};
    use crate::test_support::TestCaseChecker;


    /// Checks whether a Validator returns the expected Validator::Result when validating a
    /// Vec of rules with known outcome.
    #[test]
    fn success() {
        let input_expected_values: Vec<(Vec<Box<dyn ValidatorRule>>, super::Result)> = vec![
            (vec![Box::new(SuccessfulRule {}), Box::new(SuccessfulRule {})], Ok(())),
            (vec![Box::new(SuccessfulRule {}), Box::new(FailingRule {})], Err(ValidationError::new(vec![ValidatorRuleError::ArrayTemplateIncorrect]))),
            (vec![Box::new(FailingRule {}), Box::new(FailingRule {})], Err(ValidationError::new(vec![ValidatorRuleError::ArrayTemplateIncorrect, ValidatorRuleError::ArrayTemplateIncorrect])))
        ];

        let template = Template::new();

        input_expected_values.into_iter().for_each(
            |(input, expected)| assert_eq!(Validator::new_with_rules(input).validate_template(&template), expected)
        )
    }

    struct Checker;
    impl TestCaseChecker for Checker {
        type Input = Vec<Box<dyn ValidatorRule>>;
        type Expected = super::Result;

        fn create_test_cases(&self) -> Vec<(Self::Input, Self::Expected)> {
            vec![
                (vec![Box::new(SuccessfulRule {}), Box::new(SuccessfulRule {})], Ok(())),
                (vec![Box::new(SuccessfulRule {}), Box::new(FailingRule {})], Err(ValidationError::new(vec![ValidatorRuleError::ArrayTemplateIncorrect]))),
                (vec![Box::new(FailingRule {}), Box::new(FailingRule {})], Err(ValidationError::new(vec![ValidatorRuleError::ArrayTemplateIncorrect, ValidatorRuleError::ArrayTemplateIncorrect])))
            ]
        }
    }

    struct SuccessfulRule;
    struct FailingRule;

    impl ValidatorRule for SuccessfulRule {
        fn validate(&self, template: &Template) -> Result {
            Ok(())
        }
    }

    impl ValidatorRule for FailingRule {
        fn validate(&self, template: &Template) -> Result {
            Err(ValidatorRuleError::ArrayTemplateIncorrect)
        }
    }
}