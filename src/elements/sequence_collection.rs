use serde::{Deserialize, Serialize};

use crate::{
    elements::{
        db_sequence::DbSequence, is_element::IsElement, peptide::Peptide,
        peptide_evidence::PeptideEvidence,
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
    fn validate(&self, strict: bool) -> Result<(), ValidationError> {
        for db_sequence in &self.db_sequences {
            db_sequence.validate(strict)?;
        }
        for peptide in &self.peptides {
            peptide.validate(strict)?;
        }
        for peptide_evidence in &self.peptide_evidence {
            peptide_evidence.validate(strict)?;
        }
        Ok(())
    }
}
