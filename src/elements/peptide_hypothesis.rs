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
    const ELEMENT_TAG: &str = "PeptideHypothesis";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.peptide_evidence_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "peptideEvidence_ref",
            ));
        }

        if self.spectrum_identification_item_refs.is_empty() {
            return Err(ValidationError::ChildRequiredAtLeastOnce(
                Self::element_path_to_string(element_path),
                "SpectrumIdentificationItemRef",
            ));
        }

        Self::validate_elements(
            version,
            strict,
            element_path,
            self.spectrum_identification_item_refs.iter(),
        )
    }
}
