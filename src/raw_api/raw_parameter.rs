use crate::raw_api::type_descriptor::TypeDescriptor;

#[derive(Debug)]
pub struct RawParameter {
    pub name: String,
    pub parameter_type: TypeDescriptor,
}