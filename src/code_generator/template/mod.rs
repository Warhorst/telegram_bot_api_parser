use core::fmt;
use std::convert::TryFrom;

use handlebars::{RenderError, TemplateError, TemplateFileError};
use serde::Deserialize;

use crate::code_generator::CodeGenerator;
use crate::code_generator::target_files::SameFilenameError;
use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::resolve_strategy::NoValidResolveStrategyError;
use crate::code_generator::template::resolve_strategy::ResolveStrategy;
use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::raw_api::RawApi;

mod template_file;
mod template_resolver;
mod resolve_strategy;
mod objects;
pub mod template_reader;

/// Generates code fom a given JSON-template.
pub struct TemplateCodeGenerator {
    template: Template
}

impl TemplateCodeGenerator {
    pub fn new(template: Template) -> Self {
        TemplateCodeGenerator { template }
    }
}

impl CodeGenerator for TemplateCodeGenerator {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error> {
        let mut result = TargetFiles::new();
        let resolver = TemplateResolver::new(&self.template)?;
        let dtos = api.get_dtos();

        for template_file in &self.template.template_files {
            let file_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match file_strategy {
                ResolveStrategy::ForAllDTOs => result.insert_all(resolver.resolve_for_each_dto(template_file, dtos)?)?,
                ResolveStrategy::ForEachDTO => {
                    for dto in dtos {
                        result.insert_all(resolver.resolve_for_single_dto(template_file, dto)?)?
                    }
                }
            };
        }

        Ok(result)
    }
}

/// A template that is extracted from the templates.json.
#[derive(Deserialize, Debug)]
pub struct Template {
    pub integer_type: String,
    pub string_type: String,
    pub boolean_type: String,
    pub array_type: String,
    pub optional_type: String,
    pub template_files: Vec<TemplateFile>
}

/// Contains the data of a template-file and how it should be transformed into generated code
#[derive(Deserialize, Debug)]
pub struct TemplateFile {
    pub template_path: String,
    pub target_path: String,
    pub resolve_strategy: String,
}

#[derive(Debug)]
pub enum TemplateCodeGenerationError {
    NoValidResolveStrategyError(NoValidResolveStrategyError),
    IoError(std::io::Error),
    SameFilenameError(SameFilenameError),
    HandlebarsTemplateError(TemplateError),
    HandlebarsTemplateFileError(TemplateFileError),
    HandlebarsRenderError(RenderError)
}

impl std::error::Error for TemplateCodeGenerationError {}

impl std::fmt::Display for TemplateCodeGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateCodeGenerationError::NoValidResolveStrategyError(e) => e.fmt(f),
            TemplateCodeGenerationError::IoError(e) => e.fmt(f),
            TemplateCodeGenerationError::SameFilenameError(e) => e.fmt(f),
            TemplateCodeGenerationError::HandlebarsTemplateError(e) => e.fmt(f),
            TemplateCodeGenerationError::HandlebarsTemplateFileError(e) => std::fmt::Display::fmt(&e, f),
            TemplateCodeGenerationError::HandlebarsRenderError(e) => e.fmt(f)
        }
    }
}

impl From<NoValidResolveStrategyError> for TemplateCodeGenerationError {
    fn from(e: NoValidResolveStrategyError) -> Self {
        TemplateCodeGenerationError::NoValidResolveStrategyError(e)
    }
}

impl From<std::io::Error> for TemplateCodeGenerationError {
    fn from(e: std::io::Error) -> Self {
        TemplateCodeGenerationError::IoError(e)
    }
}

impl From<SameFilenameError> for TemplateCodeGenerationError {
    fn from(e: SameFilenameError) -> Self {
        TemplateCodeGenerationError::SameFilenameError(e)
    }
}

impl From<TemplateError> for TemplateCodeGenerationError {
    fn from(e: TemplateError) -> Self {
        TemplateCodeGenerationError::HandlebarsTemplateError(e)
    }
}

impl From<TemplateFileError> for TemplateCodeGenerationError {
    fn from(e: TemplateFileError) -> Self {
        TemplateCodeGenerationError::HandlebarsTemplateFileError(e)
    }
}

impl From<RenderError> for TemplateCodeGenerationError {
    fn from(e: RenderError) -> Self {
        TemplateCodeGenerationError::HandlebarsRenderError(e)
    }
}