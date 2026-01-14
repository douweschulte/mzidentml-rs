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
    const ELEMENT_TAG: &str = "ProteinDetection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.id.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "id",
            ));
        }
        if self.protein_detection_list_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "proteinDetectionList_ref",
            ));
        }
        if self.protein_detection_protocol_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "proteinDetectionProtocol_ref",
            ));
        }

        if self.input_spectrum_identifications.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "InputSpectrumIdentifications",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.input_spectrum_identifications.iter(),
        )?;
        Ok(())
    }
}
