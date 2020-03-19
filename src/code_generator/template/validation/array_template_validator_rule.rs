use crate::code_generator::template::template::{Template};
use crate::code_generator::template::validation::validator_rule::{Result, ValidatorRule, ValidatorRuleError};
use crate::util::is_template;

/// A template-file can contain a template for an array type.
/// If a template is used, the key "value" must be used (for example Vec<{{value}}>).
/// A template is recognized by a word surrounded with double braces {{...}}.
pub struct ArrayTemplateValidatorRule {}

impl ArrayTemplateValidatorRule {
    const EXPECTED_TEMPLATE: &'static str = "{{value}}";
}

impl ValidatorRule for ArrayTemplateValidatorRule {
    fn validate(&self, template: &dyn Template) -> Result {
        let array_template = template.get_array_type();

        match is_template(array_template) {
            false => Ok(()),
            true => match !array_template.contains(ArrayTemplateValidatorRule::EXPECTED_TEMPLATE) {
                false => Ok(()),
                true => Err(ValidatorRuleError::ArrayTemplateIncorrect)
            }
        }
    }
}

///TODO: crate::util::is_template should also be mocked (return true/false depending on test case).
#[cfg(test)]
mod tests {
    use crate::code_generator::template::template::{MockTemplate, Template};
    use crate::code_generator::template::validation::array_template_validator_rule::ArrayTemplateValidatorRule;
    use crate::code_generator::template::validation::validator_rule::{ValidatorRule, ValidatorRuleError};

    #[test]
    fn success() {
        let rule = ArrayTemplateValidatorRule {};
        let result = rule.validate(&*create_valid_template());

        assert_eq!(Ok(()), result)
    }

    #[test]
    fn failure() {
        let rule = ArrayTemplateValidatorRule {};
        let result = rule.validate(&*create_invalid_template());

        assert_eq!(Err(ValidatorRuleError::ArrayTemplateIncorrect), result)
    }

    /// Creates a Template with a valid array template String
    fn create_valid_template() -> Box<dyn Template> {
        let mut template_mock = MockTemplate::new();
        template_mock.expect_get_array_type().return_const(String::from("Array<{{value}}>"));

        Box::new(template_mock)
    }

    /// Creates a Template with an invalid array template String
    fn create_invalid_template() -> Box<dyn Template> {
        let mut template_mock = MockTemplate::new();
        template_mock.expect_get_array_type().return_const(String::from("Array<{{array_value}}>"));

        Box::new(template_mock)
    }
}