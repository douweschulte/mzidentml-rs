use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalFormatDocumentation(String);

impl IsElement for ExternalFormatDocumentation {
    const ELEMENT_TAG: &str = "ExternalFormatDocumentation";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        _elements_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}
