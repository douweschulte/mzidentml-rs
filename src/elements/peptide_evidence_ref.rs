use serde::{Deserialize, Serialize};

use crate::{
    elements::{attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeptideEvidenceRef {
    #[serde(rename = "@peptideEvidence_ref")]
    pub peptide_evidence_ref: String,
}

impl IsElement for PeptideEvidenceRef {
    fn validate(&self, _version: &SemVer, _strict: bool) -> Result<(), ValidationError> {
        if self.peptide_evidence_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                "PeptideEvidenceRef",
                "peptideEvidence_ref",
            ));
        }
        Ok(())
    }
}
