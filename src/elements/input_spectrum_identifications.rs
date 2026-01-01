use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSpectrumIdentifications {
    #[serde(rename = "@spectrumIdentificationList_ref")]
    pub spectrum_identification_list_ref: String,
}

impl IsElement for InputSpectrumIdentifications {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.spectrum_identification_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "InputSpectrumIdentifications",
                "spectrumIdentificationList_ref",
            ));
        }
        Ok(())
    }
}
