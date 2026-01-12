use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement,
        protein_detection_list::ProteinDetectionList,
        spectrum_identification_list::SpectrumIdentificationList,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisData {
    #[serde(rename = "SpectrumIdentificationList")]
    pub spectrum_identification_lists: Vec<SpectrumIdentificationList>,
    #[serde(rename = "ProteinDetectionList")]
    pub protein_detection_list: Option<ProteinDetectionList>,
}

impl IsElement for AnalysisData {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.spectrum_identification_lists.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AnalysisData",
                "SpectrumIdentificationList",
            ));
        }

        for spectrum_identification_list in &self.spectrum_identification_lists {
            spectrum_identification_list.validate(version, strict)?;
        }

        if let Some(protein_detection_list) = &self.protein_detection_list {
            protein_detection_list.validate(version, strict)?;
        }

        Ok(())
    }
}
