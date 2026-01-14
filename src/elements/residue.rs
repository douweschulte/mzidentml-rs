use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Residue {
    #[serde(rename = "@code")]
    pub code: char,
    #[serde(rename = "@mass")]
    pub mass: f64,
}

impl IsElement for Residue {
    const ELEMENT_TAG: &str = "Residue";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        _element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
