use serde::{Deserialize, Serialize};

use crate::{
    elements::{is_element::IsElement, search_modification::SearchModification},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModificationParams {
    #[serde(rename = "SearchModification")]
    pub search_modifications: Vec<SearchModification>,
}

impl IsElement for ModificationParams {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.search_modifications.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "ModificationParams",
                "SearchModification",
            ));
        }

        for search_mod in &self.search_modifications {
            search_mod.validate(strict)?;
        }

        Ok(())
    }
}
