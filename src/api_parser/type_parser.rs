use crate::raw_api::type_descriptor::TypeDescriptor;

pub trait TypeParser {
    fn parse_field_type(&self, type_string: String, description_string: String) -> TypeDescriptor;

    fn parse_parameter_type(&self, type_string: String, required_string: String) -> TypeDescriptor;
}

pub struct TypeParserImpl;

impl TypeParserImpl {
    const INTEGER_STR: &'static str = "Integer";
    const STRING_STR: &'static str = "String";
    const INTEGER_OR_STRING_STR: &'static str = "Integer or String";
    const BOOLEAN_STR: &'static str = "Boolean";
    const ARRAY_OF_STR: &'static str = "Arrayof";
    const OPTIONAL_STR: &'static str = "Optional";

    fn field_is_optional_by_description(&self, description_string: String) -> bool {
        self.trim_whitespace(description_string).starts_with(Self::OPTIONAL_STR)
    }

    fn create_base_field_type_from_string(&self, type_string: String) -> TypeDescriptor {
        match type_string.as_str() {
            Self::INTEGER_STR => TypeDescriptor::Integer,
            Self::STRING_STR => TypeDescriptor::String,
            Self::BOOLEAN_STR => TypeDescriptor::Boolean,
            _ => {
                let trimmed = self.trim_whitespace(type_string.clone());

                if trimmed.starts_with(Self::ARRAY_OF_STR) {
                    let array_type = String::from(&trimmed.as_str()[Self::ARRAY_OF_STR.len()..trimmed.len()]);
                    TypeDescriptor::ArrayOf(Box::new(self.create_base_field_type_from_string(array_type)))
                } else {
                    TypeDescriptor::DTO(trimmed)
                }
            }
        }
    }

    fn parameter_is_optional_by_required_string(&self, required_string: String) -> bool {
        self.trim_whitespace(required_string).starts_with(Self::OPTIONAL_STR)
    }

    /// Create the descriptor from the given type string. Currently, if
    /// the can be either Integer or String, String is preferred.
    fn create_base_parameter_type_from_string(&self, type_string: String) -> TypeDescriptor {
        match type_string.as_str() {
            Self::INTEGER_STR => TypeDescriptor::Integer,
            Self::STRING_STR => TypeDescriptor::String,
            Self::INTEGER_OR_STRING_STR => TypeDescriptor::String,
            Self::BOOLEAN_STR => TypeDescriptor::Boolean,
            _ => TypeDescriptor::DTO(type_string)
        }
    }

    fn trim_whitespace(&self, mut string: String) -> String {
        string.retain(|c| !c.is_whitespace());
        string
    }
}

impl TypeParser for TypeParserImpl {
    fn parse_field_type(&self, type_string: String, description_string: String) -> TypeDescriptor {
        let optional = self.field_is_optional_by_description(description_string);
        let base_type = self.create_base_field_type_from_string(type_string);
        match optional {
            true => TypeDescriptor::Optional(Box::new(base_type)),
            false => base_type
        }
    }

    fn parse_parameter_type(&self, type_string: String, required_string: String) -> TypeDescriptor {
        let optional = self.parameter_is_optional_by_required_string(required_string);
        let base_type = self.create_base_parameter_type_from_string(type_string);
        match optional {
            true => TypeDescriptor::Optional(Box::new(base_type)),
            false => base_type
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api_parser::type_parser::{TypeParser, TypeParserImpl};
    use crate::raw_api::type_descriptor::TypeDescriptor;

    #[test]
    fn success_integer_field() {
        let type_string = String::from("Integer");
        let description = String::from("description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string, description);

        assert_eq!(type_descriptor, TypeDescriptor::Integer)
    }

    #[test]
    fn success_string_field() {
        let type_string = String::from("String");
        let description = String::from("description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string, description);

        assert_eq!(type_descriptor, TypeDescriptor::String)
    }

    #[test]
    fn success_boolean_field() {
        let type_string = String::from("Boolean");
        let description = String::from("description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string, description);

        assert_eq!(type_descriptor, TypeDescriptor::Boolean)
    }

    #[test]
    fn success_dto_field() {
        let type_string = String::from("SomeDTO");
        let description = String::from("description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string.clone(), description);

        assert_eq!(type_descriptor, TypeDescriptor::DTO(type_string))
    }

    #[test]
    fn success_optional_field() {
        let type_string = String::from("SomeDTO");
        let description = String::from("Optional. description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string.clone(), description);

        assert_eq!(type_descriptor, TypeDescriptor::Optional(Box::new(TypeDescriptor::DTO(type_string))))
    }

    #[test]
    fn success_array_field() {
        let type_string = String::from("Array of SomeDTO");
        let description = String::from("description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string, description);

        assert_eq!(type_descriptor, TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("SomeDTO")))))
    }

    #[test]
    fn success_optional_array_field() {
        let type_string = String::from("Array of SomeDTO");
        let description = String::from("Optional. description");

        let type_descriptor = TypeParserImpl.parse_field_type(type_string, description);

        assert_eq!(type_descriptor, TypeDescriptor::Optional(Box::new(TypeDescriptor::ArrayOf(Box::new(TypeDescriptor::DTO(String::from("SomeDTO")))))))
    }

    #[test]
    fn success_integer_parameter() {
        let type_string = String::from("Integer");
        let required_string = String::from("Required");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string, required_string);

        assert_eq!(type_descriptor, TypeDescriptor::Integer)
    }

    #[test]
    fn success_string_parameter() {
        let type_string = String::from("String");
        let required_string = String::from("Required");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string, required_string);

        assert_eq!(type_descriptor, TypeDescriptor::String)
    }

    #[test]
    fn success_integer_or_string_parameter() {
        let type_string = String::from("Integer or String");
        let required_string = String::from("Required");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string, required_string);

        assert_eq!(type_descriptor, TypeDescriptor::String)
    }

    #[test]
    fn success_boolean_parameter() {
        let type_string = String::from("Boolean");
        let required_string = String::from("Required");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string, required_string);

        assert_eq!(type_descriptor, TypeDescriptor::Boolean)
    }

    #[test]
    fn success_dto_parameter() {
        let type_string = String::from("SomeDTO");
        let required_string = String::from("Required");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string.clone(), required_string);

        assert_eq!(type_descriptor, TypeDescriptor::DTO(type_string))
    }

    #[test]
    fn success_optional_parameter() {
        let type_string = String::from("SomeDTO");
        let required_string = String::from("Optional");

        let type_descriptor = TypeParserImpl.parse_parameter_type(type_string.clone(), required_string);

        assert_eq!(type_descriptor, TypeDescriptor::Optional(Box::new(TypeDescriptor::DTO(type_string))))
    }
}