use crate::raw_api::dto_field::DTOField;

/// Holds a Telegram-Bot-DTO with its name and all fields.
#[derive(Eq,PartialEq ,Debug)]
pub struct BotDTO {
    name: String,
    fields: Vec<DTOField>
}

impl BotDTO {
    pub fn new(name: String, fields: Vec<DTOField>) -> Self {
        BotDTO {
            name,
            fields
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_fields(&self) -> &Vec<DTOField> {
        &self.fields
    }
}