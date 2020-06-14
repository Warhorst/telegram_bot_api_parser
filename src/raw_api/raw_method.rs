use crate::raw_api::raw_parameter::RawParameter;

#[derive(Debug)]
pub struct RawMethod {
    pub name: String,
    pub parameters: Vec<RawParameter>
}