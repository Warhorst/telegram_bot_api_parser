use crate::code_generator::target_files::TargetFiles;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs::{remove_dir_all, create_dir};

/// Creates all files that were generated by a CodeGenerator.
pub struct CodeWriter {}

impl CodeWriter {
    const BASE_PATH: &'static str = "generated/";

    /// Creates all files in the given TargetFiles object.
    pub fn write(&self, target_files: TargetFiles) -> Result<(), std::io::Error> {
        self.create_target_directory()?;

        for (filename, content) in target_files.into_iter() {
            let mut file = File::create(self.create_path(filename))?;
            file.write(content.as_bytes())?;
        }

        Ok(())
    }

    /// Creates the target directory for generated files.
    /// If the directory already exists, it will be removed and recreated.
    fn create_target_directory(&self) -> Result<(), std::io::Error> {
        let generated_path = Path::new(Self::BASE_PATH);

        if generated_path.exists() {
            remove_dir_all(&generated_path)?
        }

        create_dir(&generated_path)?;

        Ok(())
    }

    /// Creates a path from the base path and the given filename.
    fn create_path(&self, file_name: String) -> String {
        let mut path = String::from(Self::BASE_PATH);
        path.push_str(file_name.as_str());

        path
    }
}