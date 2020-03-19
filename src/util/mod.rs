use regex::Regex;

/// Returns a snake_case equivalent of the given String
///
/// All uppercase-chars are changed to lowercase. If the changed char was not the first char (index 0),
/// a underscore prefix is added (_).
/// Non-ascii-chars stay unchanged.
pub fn to_snake_case(input: &String) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if i == 0 && c.is_uppercase() {
            result.push(c.to_ascii_lowercase());
        } else if c.is_uppercase() {
            result.push('_');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c)
        }
    }

    result
}

    /// Checks whether the given String is a template.
    /// It is assumed to be a template if it contains a string surrounded by double braces (Hex 7B and 7D).
    pub fn is_template(input: &String) -> bool {
        let regex = Regex::new(r"^.*\x7b\x7b.*\x7d\x7d.*$").unwrap();
        regex.is_match(input.as_str())
    }

#[cfg(test)]
pub mod tests {
    use crate::util::{to_snake_case, is_template};

    #[test]
    fn success_to_snake_case_single_noun() {
        let input = String::from("Update");

        assert_eq!(to_snake_case(&input), String::from("update"))
    }

    #[test]
    fn success_to_snake_case_multi_noun() {
        let input = String::from("UserUpdateMessage");

        assert_eq!(to_snake_case(&input), String::from("user_update_message"))
    }

    #[test]
    fn success_is_template() {
        let input_expected = vec![
            (String::from("Option<{{value}}>"), true),
            (String::from("Vec<{{value}}>"), true),
            (String::from("Option<{{{value}}}>"), true),
            (String::from("Vec<{{{value}}}>"), true),
            (String::from("{{value}}"), true),
            (String::from("Update"), false),
            (String::from(""), false)
        ];

        input_expected.iter().for_each(|(input, expected)| assert_eq!(*expected, is_template(&input)));
    }
}