use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement,
        protein_detection_list::ProteinDetectionList,
        spectrum_identification_list::IsSpectrumIdentificationList,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "SIL: IsSpectrumIdentificationList")]
pub struct AnalysisData<SIL: IsSpectrumIdentificationList> {
    #[serde(rename = "SpectrumIdentificationList")]
    pub spectrum_identification_lists: Vec<SIL>,
    #[serde(rename = "ProteinDetectionList")]
    pub protein_detection_list: Option<ProteinDetectionList>,
}

impl<SIL: IsSpectrumIdentificationList> IsElement for AnalysisData<SIL> {
    const ELEMENT_TAG: &str = "AnalysisData";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.spectrum_identification_lists.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentificationList",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification_lists.iter(),
        )?;

        if let Some(protein_detection_list) = &self.protein_detection_list {
            protein_detection_list.validate(version, strict, element_path, None)?;
        }

        Ok(())
    }
}
