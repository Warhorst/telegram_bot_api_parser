use std::fs::File;

use serde::export::Formatter;

use crate::code_generator::configuration::Configuration;

pub struct ConfigurationReader;

impl ConfigurationReader {
    pub fn read(&self) -> Result<Configuration, ConfigurationReadError> {
        let template = serde_json::from_reader(File::open("templates/configuration.json")?)?;
        Ok(template)
    }
}

#[derive(Debug)]
pub enum ConfigurationReadError {
    OpenFileError(std::io::Error),
    DeserializeJsonError(serde_json::Error),
}

impl std::error::Error for ConfigurationReadError {}

impl std::fmt::Display for ConfigurationReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationReadError::OpenFileError(error) => std::io::Error::fmt(error, f),
            ConfigurationReadError::DeserializeJsonError(error) => serde_json::Error::fmt(error, f)
        }
    }
}

impl From<std::io::Error> for ConfigurationReadError {
    fn from(error: std::io::Error) -> Self {
        ConfigurationReadError::OpenFileError(error)
    }
}

impl From<serde_json::Error> for ConfigurationReadError {
    fn from(error: serde_json::Error) -> Self {
        ConfigurationReadError::DeserializeJsonError(error)
    }
}