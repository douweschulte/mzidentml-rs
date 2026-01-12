use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customizations(String);

impl IsElement for Customizations {
    fn validate(&self, _version: &SemVer, _strict: bool) -> Result<(), ValidationError> {
        Ok(())
    }
}
