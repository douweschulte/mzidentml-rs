use serde::{Deserialize, Serialize};

use crate::{
    elements::{is_element::IsElement, spectrum_identification_item::SpectrumIdentificationItem},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpectrumIdentificationResult {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@spectraData_ref")]
    pub spectra_data_ref: String,
    #[serde(rename = "@spectrumID")]
    pub spectrum_id: String,

    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "SpectrumIdentificationItem")]
    pub spectrum_identification_items: Vec<SpectrumIdentificationItem>,
}

impl IsElement for SpectrumIdentificationResult {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "id",
            ));
        }

        if self.spectra_data_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "spectraData_ref",
            ));
        }

        if self.spectrum_identification_items.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "SpectrumIdentificationResult",
                "SpectrumIdentificationItem",
            ));
        }

        for item in &self.spectrum_identification_items {
            item.validate(strict)?;
        }
        Ok(())
    }
}
