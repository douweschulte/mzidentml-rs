use serde::{Deserialize, Serialize};

use crate::{
    elements::{analysis_software::AnalysisSoftware, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisSoftwareList {
    #[serde(rename = "AnalysisSoftware")]
    pub analysis_software: Vec<AnalysisSoftware>,
}

impl IsElement for AnalysisSoftwareList {
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        if self.analysis_software.is_empty() {
            return Err(ValidationError::ChildRequiredOnce(
                "AnalysisSoftwareList",
                "AnalysisSoftware",
            ));
        }

        for software in &self.analysis_software {
            software.validate(strict)?;
        }
        Ok(())
    }
}
