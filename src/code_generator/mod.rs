use std::convert::TryFrom;
use std::fmt;

use serde::Serialize;

use crate::code_generator::api::Api;
use crate::code_generator::configuration::Configuration;
use crate::code_generator::renderer::{Renderer, RendererError};
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

pub struct CodeGenerator<R: Renderer> {
    configuration: Configuration,
    renderer: R
}

impl<R: Renderer> CodeGenerator<R> {
    pub fn new(configuration: Configuration, renderer: R) -> Self {
        CodeGenerator {
            configuration,
            renderer
        }
    }

    pub fn generate(&self, api: RawApi) -> Result<TargetFiles, TemplateCodeGenerationError> {
        let mut target_files = TargetFiles::new();
        let api = Api::new(api, &self.renderer);

        for template_file in self.configuration.template_files.iter() {
            let resolve_strategy = ResolveStrategy::try_from(&template_file.resolve_strategy)?;

            match resolve_strategy {
                ResolveStrategy::ForAllDTOs => target_files.insert(self.renderer.render_for_all_dtos(&api.dtos, template_file)?)?,
                ResolveStrategy::ForAllMethods => target_files.insert(self.renderer.render_for_all_methods(&api.methods, template_file)?)?,
                ResolveStrategy::ForEachDTO => {
                    for dto in &api.dtos {
                        target_files.insert(self.renderer.render_for_single_dto(dto, template_file)?)?
                    }
                }
                ResolveStrategy::ForEachMethod => {
                    for method in &api.methods {
                        target_files.insert(self.renderer.render_for_single_method(method, template_file)?)?
                    }
                }
            }
        }

        Ok(target_files)
    }
}

/// Wraps a single String so it can be processed by handlebars.
#[derive(Serialize)]
struct SingleValueHolder {
    pub value: String
}

#[derive(Debug)]
pub enum TemplateCodeGenerationError<'a> {
    NoValidResolveStrategyError(NoValidResolveStrategyError),
    SameFilenameError(SameFilenameError),
    RendererError(Box<dyn RendererError + 'a>)
}

impl<'a> std::error::Error for TemplateCodeGenerationError<'a> {}

impl<'a> std::fmt::Display for TemplateCodeGenerationError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateCodeGenerationError::NoValidResolveStrategyError(e) => e.fmt(f),
            TemplateCodeGenerationError::SameFilenameError(e) => e.fmt(f),
            TemplateCodeGenerationError::RendererError(e) => e.fmt(f)
        }
    }
}

impl<'a, R: RendererError + 'a> From<R> for TemplateCodeGenerationError<'a> {
    fn from(error: R) -> Self {
        TemplateCodeGenerationError::RendererError(Box::new(error))
    }
}

impl<'a> From<NoValidResolveStrategyError> for TemplateCodeGenerationError<'a> {
    fn from(e: NoValidResolveStrategyError) -> Self {
        TemplateCodeGenerationError::NoValidResolveStrategyError(e)
    }
}

impl<'a> From<SameFilenameError> for TemplateCodeGenerationError<'a> {
    fn from(e: SameFilenameError) -> Self {
        TemplateCodeGenerationError::SameFilenameError(e)
    }
}