use crate::raw_api::bot_dto::DTOName;
use crate::raw_api::field_type::FieldType::ArrayOf;

pub enum FieldType {
    Integer,
    String,
    DTO(DTOName),
    ArrayOf(Box<FieldType>)
}

impl FieldType {
    const INTEGER: &'static str = "Integer";
    const STRING: &'static str = "String";
    const ARRAY_OF: &'static str = "Arrayof";

    /// Returns a clone of the given String without whitespace
    fn trim_whitespace(string: &String) -> String {
        let mut result = string.clone();
        result.retain(|c| !c.is_whitespace());

        result
    }
}

impl From<String> for FieldType {
    /// Create a FieldType from a given String (extracted from the API-HTML)
    ///
    /// If the String contains "Array of", the type is an array encapsulating a FieldType
    fn from(value: String) -> Self {
        match value.as_str() {
            FieldType::INTEGER => FieldType::Integer,
            FieldType::STRING => FieldType::String,
            _ => {
                let mut trimmed = FieldType::trim_whitespace(&value);

                if trimmed.starts_with(FieldType::ARRAY_OF){
                    let result = String::from(&trimmed.as_str()[FieldType::ARRAY_OF.len()..trimmed.len()]);
                    ArrayOf(Box::new(FieldType::from(result)))
                } else {
                    FieldType::DTO(value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_api::field_type::FieldType;

    #[test]
    fn success_integer() {
        let input = String::from("Integer");
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::Integer => (),
            _ => panic!("Value not parsed to Integer!")
        }
    }

    #[test]
    fn success_string() {
        let input = String::from("String");
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::String => (),
            _ => panic!("Value not parsed to String!")
        }
    }

    #[test]
    fn success_dto() {
        let input = String::from("Update");
        let field_type = FieldType::from(input.clone());

        match field_type {
            FieldType::DTO(dto_name) => {
                assert_eq!(dto_name, input)
            },
            _ => panic!("Value not parsed to DTO!")
        }
    }

    #[test]
    fn success_array_of_non_dto() {
        let mut array_of = String::from("Array of ");
        let value = String::from("Integer");
        array_of.push_str(value.as_str());

        let field_type = FieldType::from(array_of);

        match field_type {
            FieldType::ArrayOf(array_value) => {
                match *array_value {
                    FieldType::Integer => (),
                    _ => panic!("Interior value not parsed to Integer!")
                }
            }
            _ => panic!("Value not parsed to Array!")
        }
    }
    #[test]
    fn success_array_of_dto() {
        let mut array_of = String::from("Array of ");
        let value = String::from("Update");
        array_of.push_str(value.as_str());

        let field_type = FieldType::from(array_of);

        match field_type {
            FieldType::ArrayOf(array_value) => {
                match *array_value {
                    FieldType::DTO(dto_name) => {
                        assert_eq!(dto_name, value)
                    }
                    _ => panic!("Interior value not parsed to DTO!")
                }
            }
            _ => panic!("Value not parsed to Array!")
        }
    }
}