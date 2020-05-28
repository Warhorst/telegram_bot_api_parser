use crate::raw_api::raw_field::FieldDescription;

#[derive(Eq, PartialEq, Debug)]
pub enum FieldType {
    Integer,
    String,
    Boolean,
    DTO(String),
    ArrayOf(Box<FieldType>),
    Optional(Box<FieldType>)
}

impl FieldType {
    const INTEGER: &'static str = "Integer";
    const STRING: &'static str = "String";
    const BOOLEAN: &'static str = "Boolean";
    const ARRAY_OF: &'static str = "Arrayof";

    /// Returns the DTOName of this FieldType.
    ///
    /// If this FieldType is Integer, String or Boolean, None is returned.
    /// If this FieldType is wrapped in an Array or Optional, the DTOName of the wrapped value will be returned.
    pub fn get_dto_name(&self) -> Option<String> {
        match self {
            FieldType::DTO(dto_name) => Some(dto_name.clone()),
            FieldType::ArrayOf(array_field_type) => array_field_type.get_dto_name(),
            FieldType::Optional(optional_field_type) => optional_field_type.get_dto_name(),
            _ => None
        }
    }

    /// Returns a clone of the given String without whitespace
    fn trim_whitespace(string: &String) -> String {
        let mut result = string.clone();
        result.retain(|c| !c.is_whitespace());

        result
    }
}

impl From<FieldDescription> for FieldType {
    /// Create a FieldType from a given FieldDescription (extracted from the API-HTML)
    ///
    /// If the FieldDescription-value contains "Array of", the type is an array encapsulating a FieldType.
    /// If the FieldDescription is optional, the type is an optional encapsulating a FieldType.
    /// Only the whole type can be optional, so Array of Optional for example is not possible.
    fn from(field_description: FieldDescription) -> Self {
        let value = field_description.value;

        if field_description.optional {
            return FieldType::Optional(Box::new(FieldType::from(FieldDescription::new(value, false))));
        }

        match value.as_str() {
            FieldType::INTEGER => FieldType::Integer,
            FieldType::STRING => FieldType::String,
            FieldType::BOOLEAN => FieldType::Boolean,
            _ => {
                let trimmed = FieldType::trim_whitespace(&value);

                if trimmed.starts_with(FieldType::ARRAY_OF){
                    let result = String::from(&trimmed.as_str()[FieldType::ARRAY_OF.len()..trimmed.len()]);
                    FieldType::ArrayOf(Box::new(FieldType::from(FieldDescription::new(result, false))))
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
    use crate::raw_api::raw_field::FieldDescription;

    #[test]
    fn success_integer() {
        let input = FieldDescription::new(String::from("Integer"), false);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::Integer => (),
            _ => panic!("Value not parsed to Integer!")
        }
    }

    #[test]
    fn success_string() {
        let input = FieldDescription::new(String::from("String"), false);
        let field_type = FieldType::from(input);

        match field_type {
            FieldType::String => (),
            _ => panic!("Value not parsed to String!")
        }
    }

    #[test]
    fn success_dto() {
        let input = FieldDescription::new(String::from("Update"), false);
        let field_type = FieldType::from(input.clone());

        match field_type {
            FieldType::DTO(dto_name) => {
                assert_eq!(dto_name, input.value)
            },
            _ => panic!("Value not parsed to DTO!")
        }
    }

    #[test]
    fn success_array_of_non_dto() {
        let mut array_of = String::from("Array of ");
        let value = String::from("Integer");
        array_of.push_str(value.as_str());

        let field_type = FieldType::from(FieldDescription::new(array_of, false));

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

        let field_type = FieldType::from(FieldDescription::new(array_of, false));

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
        let input = FieldDescription::new(String::from("String"), true);
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
        let input = FieldDescription::new(String::from("Array of String"), true);
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

    #[test]
    fn success_get_dto_name_integer() {
        let field_type = FieldType::Integer;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_string() {
        let field_type = FieldType::String;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_boolean() {
        let field_type = FieldType::Boolean;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_some_dto() {
        let dto_name = String::from("Update");
        let field_type = FieldType::DTO(dto_name.clone());

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_optional_dto() {
        let dto_name = String::from("Update");
        let field_type = FieldType::Optional(Box::new(FieldType::DTO(dto_name.clone())));

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_array_dto() {
        let dto_name = String::from("Update");
        let field_type = FieldType::ArrayOf(Box::new(FieldType::DTO(dto_name.clone())));

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }
}