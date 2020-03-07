use crate::raw_api::dto_field::DTOField;

pub type DTOName = String;

/// Holds a Telegram-Bot-DTO with its name and all fields.
pub struct BotDTO {
    name: DTOName,
    fields: Vec<DTOField>
}

impl BotDTO {
    pub fn get_name(&self) -> &DTOName {
        &self.name
    }

    pub fn get_fields(&self) -> &Vec<DTOField> {
        &self.fields
    }
}