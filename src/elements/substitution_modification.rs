use serde::{Deserialize, Serialize};

use crate::{
    elements::{ALLOWED_SUBSTITUTION_RESIDUES, attributes::semver::SemVer, is_element::IsElement},
    error::ValidationError,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubstitutionModification {
    #[serde(rename = "@originalResidue")]
    pub original_residue: char,
    #[serde(rename = "replacementResidue")]
    pub replacement_residue: char,

    #[serde(rename = "@avgMassDelta")]
    pub avg_mass_deltaa: Option<f64>,
    #[serde(rename = "@location")]
    pub location: Option<usize>,
    #[serde(rename = "@monoisotopicMassDelta")]
    pub monoisotopic_mass_delta: Option<f64>,
}

impl IsElement for SubstitutionModification {
    fn validate(&self, _version: &SemVer, _strict: bool) -> Result<(), ValidationError> {
        if !ALLOWED_SUBSTITUTION_RESIDUES.contains(&self.original_residue) {
            return Err(ValidationError::InvalidAttributeValue(
                "SubstitutionModification",
                "original_residue",
                ALLOWED_SUBSTITUTION_RESIDUES
                    .iter()
                    .map(|res| res.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }

        if !ALLOWED_SUBSTITUTION_RESIDUES.contains(&self.replacement_residue) {
            return Err(ValidationError::InvalidAttributeValue(
                "SubstitutionModification",
                "replacement_residue",
                ALLOWED_SUBSTITUTION_RESIDUES
                    .iter()
                    .map(|res| res.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        Ok(())
    }
}
