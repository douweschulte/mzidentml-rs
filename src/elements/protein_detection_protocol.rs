use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        analysis_params::AnalysisParams, attributes::semver::SemVer, is_element::IsElement,
        threshold::Threshold,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProteinDetectionProtocol {
    #[serde(rename = "@analysisSoftware_ref")]
    pub analysis_software_ref: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "AnalysisParams")]
    pub analysis_params: Option<AnalysisParams>,
    #[serde(rename = "Threshold")]
    pub threshold: Threshold,
}

impl IsElement for ProteinDetectionProtocol {
    const ELEMENT_TAG: &str = "ProteinDetectionProtocol";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.analysis_software_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "analysisSoftware_ref",
            ));
        }

        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }

        Self::validate_elements(version, strict, element_path, self.analysis_params.iter())?;

        self.threshold.validate(version, strict, element_path, None)
    }
}
