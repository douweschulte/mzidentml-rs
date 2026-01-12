use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "@organization_ref")]
    pub organization_ref: String,
}

impl IsElement for Parent {
    fn validate(&self, _version: &SemVer, _strict: bool) -> Result<(), ValidationError> {
        if self.organization_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "Parent",
                "organization_ref",
            ));
        }
        Ok(())
    }
}
