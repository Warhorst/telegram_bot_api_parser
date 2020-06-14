use crate::raw_api::raw_field::FieldDescription;

#[derive(Eq, PartialEq, Debug)]
pub enum TypeDescriptor {
    Integer,
    String,
    Boolean,
    DTO(String),
    ArrayOf(Box<TypeDescriptor>),
    Optional(Box<TypeDescriptor>)
}

impl TypeDescriptor {
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
            TypeDescriptor::DTO(dto_name) => Some(dto_name.clone()),
            TypeDescriptor::ArrayOf(array_field_type) => array_field_type.get_dto_name(),
            TypeDescriptor::Optional(optional_field_type) => optional_field_type.get_dto_name(),
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

impl From<FieldDescription> for TypeDescriptor {
    /// Create a FieldType from a given FieldDescription (extracted from the API-HTML)
    ///
    /// If the FieldDescription-value contains "Array of", the type is an array encapsulating a FieldType.
    /// If the FieldDescription is optional, the type is an optional encapsulating a FieldType.
    /// Only the whole type can be optional, so Array of Optional for example is not possible.
    fn from(field_description: FieldDescription) -> Self {
        let value = field_description.value;

        if field_description.optional {
            return TypeDescriptor::Optional(Box::new(TypeDescriptor::from(FieldDescription::new(value, false))));
        }

        match value.as_str() {
            TypeDescriptor::INTEGER => TypeDescriptor::Integer,
            TypeDescriptor::STRING => TypeDescriptor::String,
            TypeDescriptor::BOOLEAN => TypeDescriptor::Boolean,
            _ => {
                let trimmed = TypeDescriptor::trim_whitespace(&value);

                if trimmed.starts_with(TypeDescriptor::ARRAY_OF){
                    let result = String::from(&trimmed.as_str()[TypeDescriptor::ARRAY_OF.len()..trimmed.len()]);
                    TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::from(FieldDescription::new(result, false))))
                } else {
                    TypeDescriptor::DTO(value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_api::raw_field::FieldDescription;
    use crate::raw_api::type_descriptor::TypeDescriptor;

    #[test]
    fn success_integer() {
        let input = FieldDescription::new(String::from("Integer"), false);
        let field_type = TypeDescriptor::from(input);

        match field_type {
            TypeDescriptor::Integer => (),
            _ => panic!("Value not parsed to Integer!")
        }
    }

    #[test]
    fn success_string() {
        let input = FieldDescription::new(String::from("String"), false);
        let field_type = TypeDescriptor::from(input);

        match field_type {
            TypeDescriptor::String => (),
            _ => panic!("Value not parsed to String!")
        }
    }

    #[test]
    fn success_dto() {
        let input = FieldDescription::new(String::from("Update"), false);
        let field_type = TypeDescriptor::from(input.clone());

        match field_type {
            TypeDescriptor::DTO(dto_name) => {
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

        let field_type = TypeDescriptor::from(FieldDescription::new(array_of, false));

        match field_type {
            TypeDescriptor::ArrayOf(array_value) => {
                match *array_value {
                    TypeDescriptor::Integer => (),
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

        let field_type = TypeDescriptor::from(FieldDescription::new(array_of, false));

        match field_type {
            TypeDescriptor::ArrayOf(array_value) => {
                match *array_value {
                    TypeDescriptor::DTO(dto_name) => {
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
        let field_type = TypeDescriptor::from(input);

        match field_type {
            TypeDescriptor::Optional(value) => match *value {
                TypeDescriptor::String => (),
                _ => panic!("Value not parsed to String!")
            },
            _ => panic!("Value not parsed to Optional!")
        }
    }

    #[test]
    fn success_optional_of_array() {
        let input = FieldDescription::new(String::from("Array of String"), true);
        let field_type = TypeDescriptor::from(input);

        match field_type {
            TypeDescriptor::Optional(optional_value) => match *optional_value {
                TypeDescriptor::ArrayOf(array_value) => match *array_value {
                    TypeDescriptor::String => (),
                    _ => panic!("Value not parsed to String!")
                },
                _ => panic!("Value not parsed to Array!")
            },
            _ => panic!("Value not parsed to Optional!")
        }
    }

    #[test]
    fn success_get_dto_name_integer() {
        let field_type = TypeDescriptor::Integer;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_string() {
        let field_type = TypeDescriptor::String;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_boolean() {
        let field_type = TypeDescriptor::Boolean;
        assert_eq!(None, field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_some_dto() {
        let dto_name = String::from("Update");
        let field_type = TypeDescriptor::DTO(dto_name.clone());

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_optional_dto() {
        let dto_name = String::from("Update");
        let field_type = TypeDescriptor::Optional(Box::new(TypeDescriptor::DTO(dto_name.clone())));

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }

    #[test]
    fn success_get_dto_name_array_dto() {
        let dto_name = String::from("Update");
        let field_type = TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(dto_name.clone())));

        assert_eq!(Some(dto_name), field_type.get_dto_name())
    }
}