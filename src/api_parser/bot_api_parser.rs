use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;

/// Extracts the raw API from the HTML
pub trait BotApiParser {
    fn parse(&self) -> Result<TelegramBotApiRaw, String>;
}