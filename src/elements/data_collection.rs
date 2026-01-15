use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        analysis_data::AnalysisData, attributes::semver::SemVer, inputs::Inputs,
        is_element::IsElement, spectrum_identification_list::IsSpectrumIdentificationList,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(bound = "SIL: IsSpectrumIdentificationList")]
pub struct DataCollection<SIL: IsSpectrumIdentificationList> {
    #[serde(rename = "Inputs")]
    pub inputs: Inputs,
    #[serde(rename = "AnalysisData")]
    pub analysis_data: AnalysisData<SIL>,
}

impl<SIL: IsSpectrumIdentificationList> IsElement for DataCollection<SIL> {
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
