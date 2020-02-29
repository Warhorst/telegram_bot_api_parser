use crate::raw_api::bot_dto::BotDTO;

/// Represents a collection of all extracted values from the HTML-API
pub struct TelegramBotApiRaw {
    bot_dtos: Vec<BotDTO>
}