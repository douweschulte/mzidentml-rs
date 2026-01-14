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
    const ELEMENT_TAG: &str = "PeptideEvidenceRef";

    fn inner_validate(
        &self,
        _version: &SemVer,
        _strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        if self.peptide_evidence_ref.is_empty() {
            return Err(ValidationError::EmptyAttribute(
                Self::element_path_to_string(element_path),
                "peptideEvidence_ref",
            ));
        }
        Ok(())
    }
}
