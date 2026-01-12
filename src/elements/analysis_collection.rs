use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement, protein_detection::ProteinDetection,
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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.spectrum_identification.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AnalysisCollection",
                "SpectrumIdentification",
            ));
        }

        for spectrum_id in &self.spectrum_identification {
            spectrum_id.validate(version, strict)?;
        }

        if let Some(protein_detection) = &self.protein_detection {
            protein_detection.validate(version, strict)?;
        }
        Ok(())
    }
}
