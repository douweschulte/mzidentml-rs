use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement, translation_table::TranslationTable,
    },
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
    const ELEMENT_TAG: &str = "DatabaseTranslation";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(
            version,
            strict,
            element_path,
            self.translation_tables.iter(),
        )
    }
}
