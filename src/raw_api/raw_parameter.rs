use crate::raw_api::type_descriptor::TypeDescriptor;

pub struct RawParameter {
    name: String,
    parameter_type: TypeDescriptor,
    required: bool
}