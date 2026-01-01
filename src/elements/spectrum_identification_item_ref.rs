use serde::{Deserialize, Serialize};

use crate::{elements::is_element::IsElement, error::ValidationError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationItemRef {
    #[serde(rename = "@spectrumIdentificationItem_ref")]
    pub spectrum_identification_item_ref: String,
}

impl IsElement for SpectrumIdentificationItemRef {
    fn validate(&self, _strict: bool) -> Result<(), ValidationError> {
        if self.spectrum_identification_item_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationItemRef",
                "spectrumIdentificationItem_ref",
            ));
        }
        Ok(())
    }
}
