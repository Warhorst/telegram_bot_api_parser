use std::fs::{create_dir, remove_dir_all, create_dir_all};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::code_generator::target_files::TargetFiles;

pub struct CodeWriter;

impl CodeWriter {
    const BASE_PATH: &'static str = "generated/";

    pub fn write(&self, target_files: TargetFiles) -> Result<(), std::io::Error> {
        self.create_target_directory()?;

        for target_file in target_files.into_iter() {
            let target_path = self.create_path(target_file.file_name.clone());
            let path = Path::new(&target_path);
            let parent_dir_option = path.parent();
            if let Some(parent_dir) = parent_dir_option {
                create_dir_all(parent_dir)?
            }

            let mut file = File::create(target_path)?;
            file.write(target_file.content.as_bytes())?;
        }

        Ok(())
    }

    fn create_target_directory(&self) -> Result<(), std::io::Error> {
        let generated_path = Path::new(Self::BASE_PATH);

        if generated_path.exists() {
            remove_dir_all(&generated_path)?
        }

        create_dir(&generated_path)?;

        Ok(())
    }

    fn create_path(&self, file_name: String) -> String {
        let mut path = String::from(Self::BASE_PATH);
        path.push_str(file_name.as_str());
        path
    }
}