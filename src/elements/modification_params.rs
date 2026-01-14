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
    const ELEMENT_TAG: &str = "ModificationParams";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.search_modifications.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SearchModification",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.search_modifications.iter(),
        )
    }
}
