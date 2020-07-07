use std::convert::TryFrom;
use std::fmt;

use handlebars::{RenderError, TemplateError, TemplateFileError};
use serde::Serialize;

use crate::code_generator::api::Api;
use crate::code_generator::configuration::Configuration;
use crate::code_generator::renderer::Renderer;
use crate::code_generator::resolve_strategy::{NoValidResolveStrategyError, ResolveStrategy};
use crate::code_generator::target_files::{SameFilenameError, TargetFiles};
use crate::raw_api::RawApi;

pub mod configuration;
pub mod configuration_reader;
pub mod resolve_strategy;
pub mod renderer;
pub mod api;
mod names;
pub mod target_files;

/// Generates code from the extracted api and stores it in a file-filecontent-map.
pub trait CodeGenerator {
    type Error;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error>;
}

pub struct CodeGeneratorImpl<R: Renderer> {
    configuration: Configuration,
    renderer: R
}

impl<R: Renderer> CodeGenerator for CodeGeneratorImpl<R> {
    type Error = TemplateCodeGenerationError;

    fn generate(&self, api: RawApi) -> Result<TargetFiles, Self::Error> {
        let mut target_files = TargetFiles::new();
        let api = Api::new(api, &self.renderer);

        for template_file in self.configuration.template_files.iter() {
            let resolve_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match resolve_strategy {
                ResolveStrategy::ForAllDTOs => target_files.insert(self.renderer.render_for_all_dtos(&api.dtos, template_file).unwrap())?,
                ResolveStrategy::ForAllMethods => target_files.insert(self.renderer.render_for_all_methods(&api.methods, template_file).unwrap())?,
                ResolveStrategy::ForEachDTO => {
                    for dto in &api.dtos {
                        target_files.insert(self.renderer.render_for_single_dto(dto, template_file).unwrap())?
                    }
                }
                ResolveStrategy::ForEachMethod => {
                    for method in &api.methods {
                        target_files.insert(self.renderer.render_for_single_method(method, template_file).unwrap())?
                    }
                }
            }
        }

        Ok(target_files)
    }
}

impl<R: Renderer> CodeGeneratorImpl<R> {
    pub fn new(configuration: Configuration, renderer: R) -> Self {
        CodeGeneratorImpl {
            configuration,
            renderer
        }
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