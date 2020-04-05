use crate::raw_api::bot_dto::BotDTO;

/// Represents a collection of all extracted values from the HTML-API
#[derive(Debug)]
pub struct TelegramBotApiRaw {
    bot_dtos: Vec<BotDTO>
}

impl TelegramBotApiRaw {
    pub fn new() -> Self {
        TelegramBotApiRaw {
            bot_dtos: Vec::new()
        }
    }

    /// Adda DTO to the list of DTOs
    pub fn add_dto(&mut self, dto: BotDTO) {
        self.bot_dtos.push(dto)
    }

    pub fn get_bot_dtos(&self) -> &Vec<BotDTO> {
        &self.bot_dtos
    }
}