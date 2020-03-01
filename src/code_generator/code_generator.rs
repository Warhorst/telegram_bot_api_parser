use crate::raw_api::telegram_bot_api_raw::TelegramBotApiRaw;
use crate::code_generator::target_files_map::TargetFilesMap;

/// Generates code from the extracted api and stores it in a file-filecontent-map.
pub trait CodeGenerator {
    type Error;

    fn generate(&self, api: TelegramBotApiRaw) -> Result<TargetFilesMap, Self::Error>;
}