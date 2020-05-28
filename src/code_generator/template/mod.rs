use core::fmt;
use std::convert::TryFrom;

use handlebars::{Handlebars, RenderError, TemplateError, TemplateFileError};
use serde::Serialize;

use crate::code_generator::CodeGenerator;
use crate::code_generator::target_files::SameFilenameError;
use crate::code_generator::target_files::TargetFiles;
use crate::code_generator::template::api::{ResolvedApi, ResolvedDto};
use crate::code_generator::template::configuration::{Configuration, Rename, TemplateFile};
use crate::code_generator::template::resolve_strategy::{NoValidResolveStrategyError, ResolveStrategy};
use crate::code_generator::template::resolver::{HandlebarsResolver, Resolver};
use crate::raw_api::{Dtos, RawApi};
use crate::raw_api::field_type::FieldType;

mod resolve_strategy;
mod api;
pub mod resolver;
pub mod configuration;
pub mod configuration_reader;

/// Generates code fom a given JSON-template.
pub struct TemplateCodeGenerator<R: Resolver> {
    configuration: Configuration,
    resolver: R
}

impl<R: Resolver> CodeGenerator for TemplateCodeGenerator<R> {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error> {
        let mut result = TargetFiles::new();
        let template_api = ResolvedApi::new(api, &self.resolver).unwrap();

        for template_file in self.configuration.template_files.iter() {
            let resolve_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match resolve_strategy {
                ResolveStrategy::ForAllDTOs => result.insert_all(self.resolver.resolve_for_each_dto(template_file, template_api.get_dtos()).unwrap())?,
                ResolveStrategy::ForEachDTO => {
                    for dto in template_api.get_dtos() {
                        result.insert_all(self.resolver.resolve_for_single_dto(template_file, dto).unwrap())?
                    }
                }
            }
        }

        Ok(result)
    }
}

impl<R: Resolver> TemplateCodeGenerator<R> {

    pub fn new(configuration: Configuration, resolver: R) -> Result<Self, TemplateCodeGenerationError> {
        Ok(TemplateCodeGenerator {
            configuration,
            resolver
        })
    }

    // /// Converts given Dtos to TemplateDtos. These contain more fields to generate the desired code.
    // fn convert_to_template_dtos(&self, dtos: &Dtos) -> Result<Vec<TemplateDto>, TemplateCodeGenerationError> {
    //     let mut result = Vec::new();
    //
    //     for dto in dtos.into_iter() {
    //         result.push(TemplateDto::new(dto, &self)?)
    //     }
    //
    //     Ok(result)
    // }
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