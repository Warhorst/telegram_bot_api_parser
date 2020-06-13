use core::fmt;
use std::collections::hash_set::{IntoIter, Iter};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use serde::export::Formatter;

use crate::code_generator::target_files::SameFilenameError::{Multiple, Single};

pub type InsertResult = Result<(), SameFilenameError>;

pub struct TargetFiles {
    target_files_set: HashSet<TargetFile>
}

impl TargetFiles {
    pub fn new() -> Self {
        TargetFiles {
            target_files_set: HashSet::new()
        }
    }

    pub fn insert(&mut self, target_file: TargetFile) -> InsertResult {
        if self.contains_target_file(&target_file) {
            return Err(SameFilenameError::Single(target_file.file_name));
        }
        self.target_files_set.insert(target_file);
        Ok(())
    }

    pub fn insert_all(&mut self, other: TargetFiles) -> InsertResult {
        let mut multiple_file_names = Vec::new();

        for target_file in other.into_iter() {
            if self.contains_target_file(&target_file) {
                multiple_file_names.push(target_file.file_name)
            } else {
                self.target_files_set.insert(target_file);
            }
        }

        if !multiple_file_names.is_empty() {
            return Err(SameFilenameError::Multiple(multiple_file_names));
        }

        Ok(())
    }

    pub fn contains_target_file(&self, target_file: &TargetFile) -> bool {
        self.target_files_set.contains(target_file)
    }

    pub fn iter(&self) -> Iter<TargetFile> {
        self.target_files_set.iter()
    }

    pub fn into_iter(self) -> IntoIter<TargetFile> {
        self.target_files_set.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.target_files_set.is_empty()
    }

    pub fn len(&self) -> usize {
        self.target_files_set.len()
    }
}

#[derive(Clone)]
pub struct TargetFile {
    pub file_name: String,
    pub content: String,
}

impl Eq for TargetFile {}

impl PartialEq for TargetFile {
    /// Two target files are equal when they have the same file name
    fn eq(&self, other: &Self) -> bool {
        self.file_name.eq(&other.file_name)
    }
}

impl Hash for TargetFile {
    /// Two target files are equal when they have the same file name
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_name.hash(state)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SameFilenameError {
    Single(String),
    Multiple(Vec<String>),
}

impl std::error::Error for SameFilenameError {}

impl std::fmt::Display for SameFilenameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Single(key) => {
                writeln!(f, "Attempted to create two files with same filename \"{}\"", key)
            }
            Multiple(keys) => {
                writeln!(f, "Attempted to create two files with same filename \"{:?}\"", keys)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::code_generator::target_files::{InsertResult, SameFilenameError, TargetFile, TargetFiles};

    const ORIGINAL_FILES_COUNT: usize = 3;

    #[test]
    fn success_insert() {
        let mut target_files = create_test_files();
        let target_file = create_target_file("oof", "oof_value");

        target_files.insert(target_file.clone()).unwrap();

        assert_file_inserted(target_files, target_file)
    }

    #[test]
    fn success_insert_all() {
        let mut target_files = create_test_files();
        let tf_one = create_target_file("oof", "oof_value");
        let tf_two = create_target_file("rab", "rab_value");
        let mut other = TargetFiles::new();
        other.insert(tf_one.clone()).unwrap();
        other.insert(tf_two.clone()).unwrap();

        target_files.insert_all(other).unwrap();

        assert_files_inserted(target_files, vec![tf_one, tf_two])
    }

    #[test]
    fn failure_insert() {
        let mut target_files = create_test_files();
        let existing_file_name = String::from("foo");
        let tf = create_target_file(existing_file_name.clone().as_str(), "foo_value");

        let insert_result = target_files.insert(tf.clone());

        assert_insertion_failed(target_files, insert_result, tf)
    }

    fn assert_insertion_failed(target_files: TargetFiles, insert_result: InsertResult, existing_file: TargetFile) {
        assert_eq!(target_files.len(), ORIGINAL_FILES_COUNT);
        assert!(target_files.contains_target_file(&existing_file));
        assert_eq!(insert_result, Err(SameFilenameError::Single(existing_file.file_name)))
    }

    #[test]
    fn failure_insert_all() {
        let mut target_files = create_test_files();
        let existing_file_name_one = String::from("foo");
        let existing_file_name_two = String::from("bar");

        let mut other = TargetFiles::new();
        other.insert(create_target_file(existing_file_name_one.as_str(), "foo_value")).unwrap();
        other.insert(create_target_file(existing_file_name_two.as_str(), "bar_value")).unwrap();

        let insert_result = target_files.insert_all(other);

        assert_keys_existed(target_files, insert_result, vec![existing_file_name_one, existing_file_name_two])
    }

    fn assert_keys_existed(target_files: TargetFiles, insert_result: InsertResult, keys: Vec<String>) {
        assert_eq!(target_files.len(), ORIGINAL_FILES_COUNT);
        if let Err(SameFilenameError::Multiple(multiple_keys)) = insert_result {
            for key in keys {
                if !multiple_keys.contains(&key) { panic!("Result does not contain expected key!") }
            }
        } else {
            panic!("Result was not an error!")
        }
    }

    fn create_test_files() -> TargetFiles {
        let mut result = TargetFiles::new();
        result.insert(create_target_file("foo", "foo_value")).unwrap();
        result.insert(create_target_file("bar", "bar_value")).unwrap();
        result.insert(create_target_file("baz", "baz_value")).unwrap();

        result
    }

    fn create_target_file(file_name: &str, content: &str) -> TargetFile {
        TargetFile {
            file_name: String::from(file_name),
            content: String::from(content),
        }
    }

    fn assert_files_inserted(target_files: TargetFiles, inserted_files: Vec<TargetFile>) {
        assert_eq!(target_files.len(), ORIGINAL_FILES_COUNT + inserted_files.len());
        for file in inserted_files.iter() {
            assert!(target_files.contains_target_file(file))
        }
    }

    fn assert_file_inserted(target_files: TargetFiles, target_file: TargetFile) {
        assert_files_inserted(target_files, vec![target_file])
    }
}