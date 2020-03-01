/// Contains the data of custom templates which need to be resolved for a data-set (dtos/fields)
pub struct TemplateListing {
    name: String,
    item_template: String,
    strategy: String
}

impl TemplateListing {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_item_template(&self) -> &String {
        &self.item_template
    }

    pub fn get_strategy(&self) -> &String {
        &self.strategy
    }
}