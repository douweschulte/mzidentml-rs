use serde::{Deserialize, Serialize};

use crate::{
    elements::{analysis_params::AnalysisParams, is_element::IsElement, threshold::Threshold},
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.analysis_software_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "ProteinDetectionProtocol",
                "analysisSoftware_ref",
            ));
        }

        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "ProteinDetectionProtocol",
                "id",
            ));
        }

        if let Some(analysis_params) = &self.analysis_params {
            analysis_params.validate(strict)?;
        }

        self.threshold.validate(strict)
    }
}
