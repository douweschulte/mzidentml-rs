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
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.analysis_software.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "AnalysisSoftwareList",
                "AnalysisSoftware",
            ));
        }

        for software in &self.analysis_software {
            software.validate(version, strict)?;
        }
        Ok(())
    }
}
