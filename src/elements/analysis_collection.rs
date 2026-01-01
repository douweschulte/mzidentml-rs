use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        is_element::IsElement, protein_detection::ProteinDetection,
        spectrum_identification::SpectrumIdentification,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisCollection {
    #[serde(rename = "SpectrumIdentification")]
    pub spectrum_identification: Vec<SpectrumIdentification>,
    #[serde(rename = "ProteinDetection")]
    pub protein_detection: Option<ProteinDetection>,
}

impl IsElement for AnalysisCollection {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.spectrum_identification.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AnalysisCollection",
                "SpectrumIdentification",
            ));
        }

        for spectrum_id in &self.spectrum_identification {
            spectrum_id.validate(strict)?;
        }

        if let Some(protein_detection) = &self.protein_detection {
            protein_detection.validate(strict)?;
        }
        Ok(())
    }
}
