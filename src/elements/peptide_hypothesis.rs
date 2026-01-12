use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, is_element::IsElement,
        spectrum_identification_item_ref::SpectrumIdentificationItemRef,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeptideHypothesis {
    #[serde(rename = "@peptideEvidence_ref")]
    pub peptide_evidence_ref: String,

    #[serde(rename = "SpectrumIdentificationItemRef")]
    pub spectrum_identification_item_refs: Vec<SpectrumIdentificationItemRef>,
}

impl IsElement for PeptideHypothesis {
    fn validate(&self, version: &SemVer, strict: bool) -> Result<(), ValidationError> {
        if self.peptide_evidence_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "PeptideHypothesi",
                "peptideEvidence_ref",
            ));
        }

        if self.spectrum_identification_item_refs.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                "PeptideHypothesis",
                "SpectrumIdentificationItemRef",
            ));
        }

        for spectrum_identification_item_ref in &self.spectrum_identification_item_refs {
            spectrum_identification_item_ref.validate(version, strict)?;
        }
        Ok(())
    }
}
