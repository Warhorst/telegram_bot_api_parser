use crate::raw_api::type_descriptor::TypeDescriptor;

#[derive(Debug)]
pub struct RawParameter {
    name: String,
    parameter_type: TypeDescriptor,
}