use serde::{Deserialize, Serialize};

use crate::{
    elements::{analysis_data::AnalysisData, inputs::Inputs, is_element::IsElement},
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        self.inputs.validate(strict)?;
        self.analysis_data.validate(strict)
    }
}
