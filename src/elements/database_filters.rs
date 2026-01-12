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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.filters.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "DatabaseFilters",
                "Filter",
            ));
        }

        for filter in &self.filters {
            filter.validate(version, strict)?;
        }
        Ok(())
    }
}
