use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSpectra {
    #[serde(rename = "@spectraData_ref")]
    pub spectra_data_ref: String,
}

impl IsElement for InputSpectra {
    const ELEMENT_TAG: &str = "InputSpectra";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.spectra_data_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "spectraData_ref",
            ));
        }
        Ok(())
    }
}
