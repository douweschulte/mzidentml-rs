use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        analysis_data::AnalysisData, attributes::semver::SemVer, inputs::Inputs,
        is_element::IsElement,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataCollection {
    #[serde(rename = "Inputs")]
    inputs: Inputs,
    #[serde(rename = "AnalysisData")]
    analysis_data: AnalysisData,
}

impl IsElement for DataCollection {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        self.inputs.validate(version, strict)?;
        self.analysis_data.validate(version, strict)
    }
}
