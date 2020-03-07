use crate::raw_api::bot_dto::DTOName;
use crate::raw_api::field_type::FieldType::{ArrayOf, Optional};
use crate::raw_api::dto_field::DTOFieldType;

pub enum FieldType {
    Integer,
    String,
    DTO(DTOName),
    ArrayOf(Box<FieldType>),
    Optional(Box<FieldType>)
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

impl From<DTOFieldType> for FieldType {
    /// Create a FieldType from a given DTOFieldType (extracted from the API-HTML)
    ///
    /// If the DTOFieldType-description contains "Array of", the type is an array encapsulating a FieldType.
    /// If the DTOFieldType is optional, the type is an optional encapsulating a FieldType.
    /// Only the whole type can be optional, so Array of Optional for example is not possible.
    fn from(dto_field_type: DTOFieldType) -> Self {
        let value = dto_field_type.get_description();

        if *dto_field_type.is_optional() {
            return FieldType::Optional(Box::new(FieldType::from(DTOFieldType::new(value.to_owned(), false))));
        }

        match value.as_str() {
            FieldType::INTEGER => FieldType::Integer,
            FieldType::STRING => FieldType::String,
            _ => {
                let mut trimmed = FieldType::trim_whitespace(&value);

                if trimmed.starts_with(FieldType::ARRAY_OF){
                    let result = String::from(&trimmed.as_str()[FieldType::ARRAY_OF.len()..trimmed.len()]);
                    FieldType::ArrayOf(Box::new(FieldType::from(DTOFieldType::new(result, false))))
                } else {
                    FieldType::DTO(value.to_owned())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_api::field_type::FieldType;
    use crate::raw_api::dto_field::DTOFieldType;

    #[test]
    fn success_integer() {
        let input = DTOFieldType::new(String::from("Integer"), false);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::Integer => (),
            _ => panic!("Value not parsed to Integer!")
        }
    }

    #[test]
    fn success_string() {
        let input = DTOFieldType::new(String::from("String"), false);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::String => (),
            _ => panic!("Value not parsed to String!")
        }
    }

    #[test]
    fn success_dto() {
        let input = DTOFieldType::new(String::from("Update"), false);
        let field_type = FieldType::from(input.clone());

        match field_type {
            FieldType::DTO(dto_name) => {
                assert_eq!(&dto_name, input.get_description())
            },
            _ => panic!("Value not parsed to DTO!")
        }
    }

    #[test]
    fn success_array_of_non_dto() {
        let mut array_of = String::from("Array of ");
        let value = String::from("Integer");
        array_of.push_str(value.as_str());

        let field_type = FieldType::from(DTOFieldType::new(array_of, false));

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

        let field_type = FieldType::from(DTOFieldType::new(array_of, false));

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

    #[test]
    fn success_optional() {
        let input = DTOFieldType::new(String::from("String"), true);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::Optional(value) => match *value {
                FieldType::String => (),
                _ => panic!("Value not parsed to String!")
            },
            _ => panic!("Value not parsed to Optional!")
        }
    }

    #[test]
    fn success_optional_of_array() {
        let input = DTOFieldType::new(String::from("Array of String"), true);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::Optional(optional_value) => match *optional_value {
                FieldType::ArrayOf(array_value) => match *array_value {
                    FieldType::String => (),
                    _ => panic!("Value not parsed to String!")
                },
                _ => panic!("Value not parsed to Array!")
            },
            _ => panic!("Value not parsed to Optional!")
        }
    }
}