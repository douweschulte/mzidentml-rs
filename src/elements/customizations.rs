use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Customizations(String);

impl IsElement for Customizations {
    const ELEMENT_TAG: &str = "Customizations";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        _element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
