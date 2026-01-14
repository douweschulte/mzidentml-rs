use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        attributes::semver::SemVer, db_sequence::DbSequence, is_element::IsElement,
        peptide::Peptide, peptide_evidence::PeptideEvidence,
    },
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SequenceCollection {
    #[serde(rename = "DBSequence")]
    pub db_sequences: Vec<DbSequence>,
    #[serde(rename = "Peptide")]
    pub peptides: Vec<Peptide>,
    #[serde(rename = "PeptideEvidence")]
    pub peptide_evidence: Vec<PeptideEvidence>,
}

impl IsElement for SequenceCollection {
    const ELEMENT_TAG: &str = "SequenceCollection";

    fn inner_validate(
        &self,
        version: &SemVer,
        strict: bool,
        element_path: &mut Vec<String>,
    ) -> Result<(), ValidationError> {
        Self::validate_elements(version, strict, element_path, self.db_sequences.iter())?;
        Self::validate_elements(version, strict, element_path, self.peptides.iter())?;
        Self::validate_elements(version, strict, element_path, self.peptide_evidence.iter())
    }
}
