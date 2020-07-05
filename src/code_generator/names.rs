use serde::Serialize;

use crate::util::{to_snake_case, to_camel_case, to_capital_camel_case};

#[derive(Serialize, Eq, PartialEq, Hash)]
pub struct Names {
    pub snake_case: String,
    pub camel_case: String,
    pub capital_camel_case: String
}

impl Names {
    pub fn new(dto_name: &String) -> Self {
        let snake_case = to_snake_case(&dto_name);
        let camel_case = to_camel_case(&dto_name);
        let capital_camel_case = to_capital_camel_case(&dto_name);

        Names {
            snake_case,
            camel_case,
            capital_camel_case
        }
    }
}