use std::collections::HashMap;
use std::collections::hash_map::Iter;

/// A map of filenames and their content. It is the result of a code generation.
pub struct TargetFilesMap {
    name_content_map: HashMap<String, String>
}

impl TargetFilesMap {
    pub fn insert(&mut self, target_filename: String, content: String) -> Option<String> {
        self.name_content_map.insert(target_filename, content)
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.name_content_map.iter()
    }
}