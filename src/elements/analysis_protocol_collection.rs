use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        is_element::IsElement, protein_detection_protocol::ProteinDetectionProtocol,
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        for spectrum_identification_protocol in &self.spectrum_identification_protocols {
            spectrum_identification_protocol.validate(strict)?;
        }
        if let Some(protein_detection_protocol) = &self.protein_detection_protocol {
            protein_detection_protocol.validate(strict)?;
        }
        Ok(())
    }
}
