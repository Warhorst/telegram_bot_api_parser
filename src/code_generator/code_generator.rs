use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;

/// Generates code from the extracted api
pub trait CodeGenerator {
    type Error;

    fn generate(&self, api: TelegramBotApiRaw) -> Result<(), Self::Error>;
}