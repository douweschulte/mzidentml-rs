use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, filter::Filter, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseFilters {
    #[serde(default, rename = "Filter")]
    pub filters: Vec<Filter>,
}

impl IsElement for DatabaseFilters {
    const ELEMENT_TAG: &str = "DatabaseFilters";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.filters.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "Filter",
            ));
        }
        Self::validate_elements(version, strict, element_path, self.filters.iter())
    }
}
