use crate::raw_api::dto_field::DTOField;

/// Holds a Telegram-Bot-DTO with its name and all fields.
pub struct BotDTO {
    name: String,
    fields: Vec<DTOField>
}

impl BotDTO {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_fields(&self) -> &Vec<DTOField> {
        &self.fields
    }
}