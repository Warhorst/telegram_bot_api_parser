use crate::raw_api::type_descriptor::TypeDescriptor;

pub trait TypeParser {
    fn parse_field_type(&self, type_string: String, description_string: String) -> TypeDescriptor;

    fn parse_parameter_type(&self, type_string: String, required_string: String) -> TypeDescriptor;
}

pub struct TypeParserImpl;

impl TypeParserImpl {
    const INTEGER_STR: &'static str = "Integer";
    const STRING_STR: &'static str = "String";
    const BOOLEAN_STR: &'static str = "Boolean";
    const ARRAY_OF_STR: &'static str = "Arrayof";
    const OPTIONAL_STR: &'static str = "Optional";

    fn field_is_optional_by_description(&self, description_string: String) -> bool {
        unimplemented!()
    }

    fn create_base_field_type_from_string(&self, type_string: String) -> TypeDescriptor {
        unimplemented!()
    }

    fn parameter_is_optional_by_required_string(&self, required_string: String) -> bool {
        unimplemented!()
    }

    fn create_base_parameter_type_from_string(&self, type_string: String) -> TypeDescriptor {
        unimplemented!()
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