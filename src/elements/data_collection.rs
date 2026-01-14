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
    const ELEMENT_TAG: &str = "DataCollection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        self.inputs.validate(version, strict, element_path, None)?;
        self.analysis_data
            .validate(version, strict, element_path, None)
    }
}
