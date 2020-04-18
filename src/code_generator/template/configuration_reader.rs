use std::fs::File;

use crate::code_generator::template::configuration::Configuration;

pub struct ConfigurationReader;

impl ConfigurationReader {
    pub fn read(&self) -> Result<Configuration, String> {
        let template = serde_json::from_reader(File::open("templates/configuration.json").unwrap()).unwrap();

        Ok(template)
    }
}