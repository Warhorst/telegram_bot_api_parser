pub mod target_files;
pub mod template;

use crate::raw_api::RawApi;
use crate::code_generator::target_files::TargetFiles;

/// Generates code from the extracted api and stores it in a file-filecontent-map.
pub trait CodeGenerator {
    type Error;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error>;
}