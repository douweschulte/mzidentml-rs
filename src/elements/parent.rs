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
    const ELEMENT_TAG: &str = "Parent";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.organization_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "organization_ref",
            ));
        }
        Ok(())
    }
}
