use serde::Serialize;

use crate::code_generator::template::objects::template_dto_field::TemplateDTOField;
use crate::code_generator::template::template_resolver::TemplateResolver;
use crate::raw_api::bot_dto::BotDTO;
use crate::util::to_snake_case;
use crate::code_generator::template::template_code_generator::TemplateCodeGenerationError;

#[derive(Serialize)]
pub struct TemplateBotDTO {
    name: String,
    name_snake_case: String,
    fields: Vec<TemplateDTOField>,
}

impl TemplateBotDTO {
    pub fn new(bot_dto: &BotDTO, template_resolver: &TemplateResolver) -> Result<Self, TemplateCodeGenerationError> {
        let name = bot_dto.get_name().clone();
        let name_snake_case = to_snake_case(&name);
        let mut fields = Vec::new();

        for field in bot_dto.get_fields().iter() {
            fields.push(TemplateDTOField::new(field, template_resolver)?)
        }

        Ok(TemplateBotDTO {
            name,
            name_snake_case,
            fields,
        })
    }
}