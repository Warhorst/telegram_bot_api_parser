use crate::code_generator::template::template::Template;
use crate::code_generator::template::validation::validator_rule::{Result, ValidatorRule, ValidatorRuleError};
use crate::util::is_template;

/// A template-file can contain a template for an optional type.
/// If a template is used, the key "value" must be used (for example Option<{{value}}>).
/// A template is recognized by a word surrounded with double braces {{...}}.
pub struct OptionalTemplateValidatorRule {}

impl OptionalTemplateValidatorRule {
    const EXPECTED_TEMPLATE: &'static str = "{{value}}";
}

impl ValidatorRule for OptionalTemplateValidatorRule {
    fn validate(&self, template: &dyn Template) -> Result {
        let optional_template = template.get_optional_type();

        match is_template(optional_template) {
            false => Ok(()),
            true => match !optional_template.contains(OptionalTemplateValidatorRule::EXPECTED_TEMPLATE) {
                false => Ok(()),
                true => Err(ValidatorRuleError::OptionalTemplateIncorrect)
            }
        }
    }
}
///TODO: crate::util::is_template should also be mocked (return true/false depending on test case).
#[cfg(test)]
mod tests {
    use crate::code_generator::template::template::{MockTemplate, Template};
    use crate::code_generator::template::validation::validator_rule::{ValidatorRule, ValidatorRuleError};
    use crate::code_generator::template::validation::optional_template_validator_rule::OptionalTemplateValidatorRule;

    #[test]
    fn success() {
        let rule = OptionalTemplateValidatorRule {};
        let result = rule.validate(&*create_valid_template());

        assert_eq!(Ok(()), result)
    }

    #[test]
    fn failure() {
        let rule = OptionalTemplateValidatorRule {};
        let result = rule.validate(&*create_invalid_template());

        assert_eq!(Err(ValidatorRuleError::OptionalTemplateIncorrect), result)
    }

    /// Creates a Template with a valid array template String
    fn create_valid_template() -> Box<dyn Template> {
        let mut template_mock = MockTemplate::new();
        template_mock.expect_get_optional_type().return_const(String::from("Option<{{value}}>"));

        Box::new(template_mock)
    }

    /// Creates a Template with an invalid array template String
    fn create_invalid_template() -> Box<dyn Template> {
        let mut template_mock = MockTemplate::new();
        template_mock.expect_get_optional_type().return_const(String::from("Option<{{array_value}}>"));

        Box::new(template_mock)
    }
}
