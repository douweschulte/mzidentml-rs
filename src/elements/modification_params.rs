use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement, search_modification::SearchModification,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModificationParams {
    #[serde(rename = "SearchModification")]
    pub search_modifications: Vec<SearchModification>,
}

impl IsElement for ModificationParams {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.search_modifications.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "ModificationParams",
                "SearchModification",
            ));
        }

        for search_mod in &self.search_modifications {
            search_mod.validate(version, strict)?;
        }

        Ok(())
    }
}
