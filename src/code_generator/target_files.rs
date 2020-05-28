use core::fmt;
use std::collections::hash_map::{IntoIter, Iter};
use std::collections::HashMap;

use serde::export::Formatter;

use crate::code_generator::target_files::SameFilenameError::{Multiple, Single};

/// Filenames and their content.
pub struct TargetFiles {
    name_content_map: HashMap<String, String>
}

impl TargetFiles {
    pub fn new() -> Self {
        TargetFiles {
            name_content_map: HashMap::new()
        }
    }

    /// Insert a filename and its content to this TargetFiles instance.
    /// If a file with this name already exists, a SameFilenameError is returned.
    pub fn insert(&mut self, target_file: TargetFile) -> Result<(), SameFilenameError> {
        if let Some(_) = self.name_content_map.insert(target_file.file_name.clone(), target_file.content) {
            return Err(SameFilenameError::Single(target_file.file_name))
        }

        Ok(())
    }

    /// Inserts all key-value-pairs of another TargetFiles into this.
    /// The TargetFiles will be consumed.
    pub fn insert_all(&mut self, other: TargetFiles) -> Result<(), SameFilenameError> {
        let mut multiple_file_names = Vec::new();

        for (filename, content) in other.into_iter() {
            if self.name_content_map.contains_key(filename.as_str()) {
                multiple_file_names.push(filename)
            } else {
                self.name_content_map.insert(filename.clone(), content);
            }
        }

        if !multiple_file_names.is_empty() {
            return Err(SameFilenameError::Multiple(multiple_file_names))
        }

        Ok(())
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.name_content_map.iter()
    }

    pub fn into_iter(self) -> IntoIter<String, String> {
        self.name_content_map.into_iter()
    }
}

pub struct TargetFile {
    pub file_name: String,
    pub content: String
}

/// Indicates that a pair with an existing key should be inserted into a TargetFiles.
/// This isn't allowed, because a generated file would be overwritten by another.
#[derive(Debug, Eq, PartialEq)]
pub enum SameFilenameError {
    Single(String),
    Multiple(Vec<String>)
}

impl std::error::Error for SameFilenameError {}

impl std::fmt::Display for SameFilenameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Single(key) => {
                writeln!(f, "Attempted to create two files with same filename \"{}\"", key)
            },
            Multiple(keys) => {
                writeln!(f, "Attempted to create two files with same filename \"{:?}\"", keys)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::code_generator::target_files::{SameFilenameError, TargetFile, TargetFiles};

    #[test]
    fn success_insert() {
        let mut map = create_test_map();
        map.insert(create_target_file("oof", "oof_value")).unwrap()
    }

    #[test]
    fn success_insert_all() {
        let mut map = create_test_map();

        let mut other = TargetFiles::new();
        other.insert(create_target_file("oof", "oof_value")).unwrap();
        other.insert(create_target_file("rab", "rab_value")).unwrap();

        map.insert_all(other).unwrap()
    }

    #[test]
    fn failure_insert() {
        let mut map = create_test_map();
        let existing_key = String::from("foo");

        let insert_result = map.insert(create_target_file(existing_key.clone().as_str(), "value"));

        assert_eq!(insert_result, Err(SameFilenameError::Single(existing_key)))
    }

    #[test]
    fn failure_insert_all() {
        let mut map = create_test_map();
        let existing_key_one = String::from("foo");
        let existing_key_two = String::from("bar");

        let mut other = TargetFiles::new();
        other.insert(create_target_file(existing_key_one.as_str(), "value")).unwrap();
        other.insert(create_target_file(existing_key_two.as_str(), "value")).unwrap();

        let insert_result = map.insert_all(other);

        assert_eq!(insert_result, Err(SameFilenameError::Multiple(vec![existing_key_one, existing_key_two])))
    }

    /// Create a TargetFilesMap with keys foo, bar, baz
    fn create_test_map() -> TargetFiles {
        let mut  result = TargetFiles::new();
        result.insert(create_target_file("oof", "oof_value")).unwrap();
        result.insert(create_target_file("bar", "bar_value")).unwrap();
        result.insert(create_target_file("baz", "baz_value")).unwrap();

        result
    }

    fn create_target_file(file_name: &str, content: &str) -> TargetFile {
        TargetFile {
            file_name: String::from(file_name),
            content: String::from(content)
        }
    }
}