use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement, role::Role},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactRole {
    #[serde(rename = "@contact_ref")]
    pub contact_ref: String,
    #[serde(rename = "Role")]
    pub role: Role,
}

impl IsElement for ContactRole {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        self.role.validate(version, strict)?;
        Ok(())
    }
}
