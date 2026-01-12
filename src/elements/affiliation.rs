use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Affiliation {
    #[serde(rename = "@organization_ref")]
    organization_ref: String,
}

impl IsElement for Affiliation {
    fn validate(&self, _version: &SemVer, _strict: bool) -> Result<(), ValidationError> {
        if self.organization_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "Affiliation",
                "organization_ref",
            ));
        }
        Ok(())
    }
}
