use crate::raw_api::dto_field::DTOField;

pub type DTOName = String;

/// Holds a Telegram-Bot-DTO with its name and all fields.
pub struct BotDTO {
    name: DTOName,
    fields: Vec<DTOField>
}