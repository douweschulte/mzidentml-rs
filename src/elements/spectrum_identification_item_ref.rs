use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationItemRef {
    #[serde(rename = "@spectrumIdentificationItem_ref")]
    pub spectrum_identification_item_ref: String,
}

impl IsElement for SpectrumIdentificationItemRef {
    const ELEMENT_TAG: &str = "SpectrumIdentificationItemRef";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.spectrum_identification_item_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "spectrumIdentificationItem_ref",
            ));
        }
        Ok(())
    }
}
