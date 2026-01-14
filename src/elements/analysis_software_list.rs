use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        analysis_software::AnalysisSoftware, attributes::semver::SemVer, is_element::IsElement,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisSoftwareList {
    #[serde(rename = "AnalysisSoftware")]
    pub analysis_software: Vec<AnalysisSoftware>,
}

impl IsElement for AnalysisSoftwareList {
    const ELEMENT_TAG: &str = "AnalysisSoftwareList";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.analysis_software.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "AnalysisSoftware",
            ));
        }
        Self::validate_elements(version, strict, element_path, self.analysis_software.iter())?;
        Ok(())
    }
}
