use std::collections::HashMap;
use std::collections::hash_map::{Iter, IntoIter};
use crate::code_generator::target_files::SameFilenameError::{Single, Multiple};
use serde::export::Formatter;
use core::fmt;

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
    pub fn insert(&mut self, target_filename: String, content: String) -> Result<(), SameFilenameError> {
        if let Some(_) = self.name_content_map.insert(target_filename.clone(), content) {
            return Err(SameFilenameError::Single(target_filename))
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
    use crate::code_generator::target_files::{TargetFiles, SameFilenameError};

    #[test]
    fn success_insert() {
        let mut map = create_test_map();
        map.insert(String::from("oof"), String::from("oof_value")).unwrap()
    }

    #[test]
    fn success_insert_all() {
        let mut map = create_test_map();

        let mut other = TargetFiles::new();
        other.insert(String::from("oof"), String::from("oof_value")).unwrap();
        other.insert(String::from("rab"), String::from("rab_value")).unwrap();

        map.insert_all(other).unwrap()
    }

    #[test]
    fn failure_insert() {
        let mut map = create_test_map();
        let existing_key = String::from("foo");

        let insert_result = map.insert(existing_key.clone(), String::from("value"));

        assert_eq!(insert_result, Err(SameFilenameError::Single(existing_key)))
    }

    #[test]
    fn failure_insert_all() {
        let mut map = create_test_map();
        let existing_key_one = String::from("foo");
        let existing_key_two = String::from("bar");

        let mut other = TargetFiles::new();
        other.insert(existing_key_one.clone(), String::from("value")).unwrap();
        other.insert(existing_key_two.clone(), String::from("value")).unwrap();

        let insert_result = map.insert_all(other);

        assert_eq!(insert_result, Err(SameFilenameError::Multiple(vec![existing_key_one, existing_key_two])))
    }

    /// Create a TargetFilesMap with keys foo, bar, baz
    fn create_test_map() -> TargetFiles {
        let mut  result = TargetFiles::new();
        result.insert(String::from("foo"), String::from("foo_value")).unwrap();
        result.insert(String::from("bar"), String::from("bar_value")).unwrap();
        result.insert(String::from("baz"), String::from("baz_value")).unwrap();

        result
    }
}