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
    const ELEMENT_TAG: &str = "AnalysisCollection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.spectrum_identification.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentification",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification.iter(),
        )?;

        if let Some(protein_detection) = &self.protein_detection {
            protein_detection.validate(version, strict, element_path, None)?;
        }
        Ok(())
    }
}
