use serde::{Deserialize, Serialize};

use chrono::{DateTime, FixedOffset};

use crate::elements::attributes::semver::SemVer;
use crate::elements::{
    input_spectrum_identifications::InputSpectrumIdentifications, is_element::IsElement,
};
use crate::error::ValidationError;
use crate::parsing::opt_date_time_parsing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProteinDetection {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@proteinDetectionList_ref")]
    pub protein_detection_list_ref: String,
    #[serde(rename = "@proteinDetectionProtocol_ref")]
    pub protein_detection_protocol_ref: String,
    #[serde(default, rename = "@activityDate", with = "opt_date_time_parsing")]
    pub activity_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "InputSpectrumIdentifications")]
    pub input_spectrum_identifications: Vec<InputSpectrumIdentifications>,
}

impl IsElement for ProteinDetection {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute("ProteinDetection", "id"));
        }
        if self.protein_detection_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "ProteinDetection",
                "proteinDetectionList_ref",
            ));
        }
        if self.protein_detection_protocol_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "ProteinDetection",
                "proteinDetectionProtocol_ref",
            ));
        }

        if self.input_spectrum_identifications.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "ProteinDetection",
                "InputSpectrumIdentifications",
            ));
        }

        for input_spec_id in self.input_spectrum_identifications.iter() {
            input_spec_id.validate(version, strict)?;
        }
        Ok(())
    }
}
