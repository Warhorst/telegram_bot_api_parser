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

#[cfg(test)]
pub mod tests {
    use crate::util::to_snake_case;

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
}