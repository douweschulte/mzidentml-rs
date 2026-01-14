use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSpectrumIdentifications {
    #[serde(rename = "@spectrumIdentificationList_ref")]
    pub spectrum_identification_list_ref: String,
}

impl IsElement for InputSpectrumIdentifications {
    const ELEMENT_TAG: &str = "InputSpectrumIdentifications";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.spectrum_identification_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "spectrumIdentificationList_ref",
            ));
        }
        Ok(())
    }
}
