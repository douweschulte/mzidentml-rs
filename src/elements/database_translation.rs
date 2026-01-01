use serde::{Deserialize, Serialize};

use crate::{
    elements::{is_element::IsElement, translation_table::TranslationTable},
    error::ValidationError,
    parsing::opt_space_separated_vec_parsing,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseTranslation {
    #[serde(rename = "@frames", with = "opt_space_separated_vec_parsing")]
    pub frames: Option<Vec<i64>>,
    #[serde(rename = "TranslationTable")]
    pub translation_tables: Vec<TranslationTable>,
}

impl IsElement for DatabaseTranslation {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        for table in &self.translation_tables {
            table.validate(strict)?;
        }
        Ok(())
    }
}
