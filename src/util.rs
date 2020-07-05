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

pub fn to_camel_case(input: &String) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if i == 0 && c.is_uppercase() {
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c)
        }
    }

    result
}

pub fn to_capital_camel_case(input: &String) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if i == 0 && c.is_lowercase() {
            result.push(c.to_ascii_uppercase());
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