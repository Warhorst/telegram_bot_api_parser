use std::convert::TryFrom;
use std::fmt;

use handlebars::{RenderError, TemplateError, TemplateFileError};
use serde::Serialize;

use crate::code_generator::api::Api;
use crate::code_generator::configuration::Configuration;
use crate::code_generator::resolve_strategy::{NoValidResolveStrategyError, ResolveStrategy};
use crate::code_generator::resolver::Resolver;
use crate::code_generator::target_files::{SameFilenameError, TargetFiles};
use crate::raw_api::RawApi;

pub mod configuration;
pub mod configuration_reader;
pub mod resolver;
pub mod resolve_strategy;
pub mod renderer;
pub mod api;
pub mod target_files;

/// Generates code from the extracted api and stores it in a file-filecontent-map.
pub trait CodeGenerator {
    type Error;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error>;
}

pub struct CodeGeneratorImpl<R: Resolver> {
    configuration: Configuration,
    resolver: R,
}

impl<R: Resolver> CodeGenerator for CodeGeneratorImpl<R> {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error> {
        let mut result = TargetFiles::new();
        let template_api = Api::new(api, &self.resolver);

        for template_file in self.configuration.template_files.iter() {
            let resolve_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match resolve_strategy {
                ResolveStrategy::ForAllDTOs => result.insert_all(self.resolver.resolve_for_each_dto(template_file, template_api.get_dtos()))?,
                ResolveStrategy::ForEachDTO => {
                    for dto in template_api.get_dtos() {
                        result.insert_all(self.resolver.resolve_for_single_dto(template_file, dto))?
                    }
                }
            }
        }

        Ok(result)
    }
}

impl<R: Resolver> CodeGeneratorImpl<R> {
    pub fn new(configuration: Configuration, resolver: R) -> Result<Self, TemplateCodeGenerationError> {
        Ok(CodeGeneratorImpl {
            configuration,
            resolver,
        })
    }
}

/// Wraps a single String so it can be processed by handlebars.
#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

#[derive(Debug)]
pub enum TemplateCodeGenerationError {
    NoValidResolveStrategyError(NoValidResolveStrategyError),
    IoError(std::io::Error),
    SameFilenameError(SameFilenameError),
    HandlebarsTemplateError(TemplateError),
    HandlebarsTemplateFileError(TemplateFileError),
    HandlebarsRenderError(RenderError),
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