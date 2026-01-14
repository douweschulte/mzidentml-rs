use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement,
        protein_detection_protocol::ProteinDetectionProtocol,
        spectrum_identification_protocol::SpectrumIdentificationProtocol,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalysisProtocolCollection {
    #[serde(rename = "SpectrumIdentificationProtocol")]
    pub spectrum_identification_protocols: Vec<SpectrumIdentificationProtocol>,
    #[serde(rename = "ProteinDetectionProtocol")]
    pub protein_detection_protocol: Option<ProteinDetectionProtocol>,
}

impl IsElement for AnalysisProtocolCollection {
    const ELEMENT_TAG: &str = "AnalysisProtocolCollection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification_protocols.iter(),
        )?;
        if let Some(protein_detection_protocol) = &self.protein_detection_protocol {
            protein_detection_protocol.validate(version, strict, element_path, None)?;
        }
        Ok(())
    }
}
